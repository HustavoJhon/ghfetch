use crate::cache::Cache;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubProfile {
    pub login: String,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: String,
    pub followers: u64,
    pub following: u64,
    pub public_repos: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionDay {
    pub count: u32,
    pub color: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionWeek {
    pub days: Vec<ContributionDay>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionData {
    pub weeks: Vec<ContributionWeek>,
    pub total: u32,
}

pub struct GitHubClient {
    token: Option<String>,
    cache: Cache,
    agent: ureq::Agent,
}

impl GitHubClient {
    pub fn new(token: Option<String>, cache: Cache) -> Self {
        Self {
            token,
            cache,
            agent: ureq::AgentBuilder::new()
                .timeout_read(std::time::Duration::from_secs(10))
                .timeout_write(std::time::Duration::from_secs(10))
                .build(),
        }
    }

    fn auth_header(&self) -> Option<String> {
        self.token
            .as_ref()
            .map(|t| format!("Bearer {}", t))
    }

    fn get_json<T: serde::de::DeserializeOwned + serde::Serialize>(
        &self,
        url: &str,
        cache_key: &str,
        ttl_hours: u64,
    ) -> Option<T> {
        if let Some(cached) = self.cache.get::<T>(cache_key, ttl_hours) {
            return Some(cached);
        }

        let mut req = self.agent.get(url).set("User-Agent", "ghfetch/0.1.0");
        if let Some(ref auth) = self.auth_header() {
            req = req.set("Authorization", auth);
        }

        match req.call() {
            Ok(resp) => {
                let body: T = resp.into_json().ok()?;
                self.cache.set(cache_key, &body);
                Some(body)
            }
            Err(e) => {
                eprintln!("Warning: GitHub API error ({}): {}", url, e);
                None
            }
        }
    }

    pub fn fetch_profile(&self, username: &str, ttl_hours: u64) -> Option<GitHubProfile> {
        let url = format!("https://api.github.com/users/{}", username);
        let key = format!("profile_{}", username);
        self.get_json(&url, &key, ttl_hours)
    }

    pub fn fetch_contributions(
        &self,
        username: &str,
        ttl_hours: u64,
    ) -> Option<ContributionData> {
        let cache_key = format!("contrib_{}", username);

        if let Some(cached) = self.cache.get::<ContributionData>(&cache_key, ttl_hours) {
            return Some(cached);
        }

        if let Some(ref token) = self.token {
            if let Some(data) = self.fetch_contributions_graphql(username, token) {
                self.cache.set(&cache_key, &data);
                return Some(data);
            }
        }

        if let Some(data) = self.fetch_contributions_rest(username) {
            self.cache.set(&cache_key, &data);
            return Some(data);
        }

        None
    }

    fn fetch_contributions_graphql(
        &self,
        username: &str,
        token: &str,
    ) -> Option<ContributionData> {
        let query = format!(
            r#"{{"query":"query {{ user(login: \"{}\") {{ contributionsCollection {{ contributionCalendar {{ totalContributions weeks {{ contributionDays {{ contributionCount date color }} }} }} }} }} }}"}}"#,
            username
        );

        match self
            .agent
            .post("https://api.github.com/graphql")
            .set("Authorization", &format!("Bearer {}", token))
            .set("User-Agent", "ghfetch/0.1.0")
            .set("Content-Type", "application/json")
            .send_string(&query)
        {
            Ok(resp) => {
                let json: serde_json::Value = resp.into_json().ok()?;
                let calendar = json
                    .pointer("/data/user/contributionsCollection/contributionCalendar")?;
                let total = calendar
                    .get("totalContributions")?
                    .as_u64()? as u32;
                let weeks_raw = calendar.get("weeks")?.as_array()?;
                let mut weeks = Vec::new();
                for w in weeks_raw {
                    let days_raw = w.get("contributionDays")?.as_array()?;
                    let mut days = Vec::new();
                    for d in days_raw {
                        days.push(ContributionDay {
                            count: d.get("contributionCount")?.as_u64()? as u32,
                            color: d.get("color")?.as_str()?.to_string(),
                            date: d.get("date")?.as_str()?.to_string(),
                        });
                    }
                    weeks.push(ContributionWeek { days });
                }
                Some(ContributionData { weeks, total })
            }
            Err(e) => {
                eprintln!("Warning: GraphQL API error: {}", e);
                None
            }
        }
    }

    fn fetch_contributions_rest(&self, username: &str) -> Option<ContributionData> {
        let url = format!("https://github.com/users/{}/contributions", username);
        match self
            .agent
            .get(&url)
            .set("User-Agent", "ghfetch/0.1.0")
            .call()
        {
            Ok(resp) => {
                let html = resp.into_string().ok()?;
                Self::parse_contributions_html(&html)
            }
            Err(e) => {
                eprintln!("Warning: Contributions REST error: {}", e);
                None
            }
        }
    }

    fn parse_contributions_html(html: &str) -> Option<ContributionData> {
        let mut weeks = Vec::new();
        let mut total = 0u32;

        let table_start = html.find("<table")?;
        let table_end = html[table_start..].find("</table>")?;
        let table = &html[table_start..table_start + table_end + 8];

        let mut week_days = Vec::new();
        for line in table.lines() {
            if line.contains("<td") {
                let data_level = extract_attr(line, "data-level");
                let data_date = extract_attr(line, "data-date").unwrap_or("");
                let count = data_level
                    .and_then(|s| s.parse::<u32>().ok());
                let color = match data_level {
                    Some("0") | None => "#282828",
                    Some("1") => "#0e4429",
                    Some("2") => "#006d32",
                    Some("3") => "#26a641",
                    Some("4") => "#39d353",
                    _ => "#282828",
                };
                let c = count.unwrap_or(0);
                total += c;
                week_days.push(ContributionDay {
                    count: c,
                    color: color.to_string(),
                    date: data_date.to_string(),
                });
                if week_days.len() == 7 {
                    weeks.push(ContributionWeek { days: week_days });
                    week_days = Vec::new();
                }
            }
        }
        if !week_days.is_empty() {
            weeks.push(ContributionWeek { days: week_days });
        }

        if weeks.is_empty() {
            return None;
        }
        Some(ContributionData { weeks, total })
    }
}

fn extract_attr<'a>(line: &'a str, attr: &str) -> Option<&'a str> {
    let pattern = format!("{}=\"", attr);
    let start = line.find(&pattern)? + pattern.len();
    let end = line[start..].find('"')?;
    Some(&line[start..start + end])
}
