use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_username")]
    pub username: String,

    #[serde(default)]
    pub github_token: Option<String>,

    #[serde(default)]
    pub cache: CacheConfig,

    #[serde(default)]
    pub display: DisplayConfig,

    #[serde(default)]
    pub icons: IconsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_cache_ttl_avatar")]
    pub avatar_ttl_hours: u64,

    #[serde(default = "default_cache_ttl_profile")]
    pub profile_ttl_hours: u64,

    #[serde(default = "default_cache_ttl_contrib")]
    pub contributions_ttl_hours: u64,

    #[serde(default = "default_cache_dir")]
    pub cache_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    #[serde(default = "default_true")]
    pub show_avatar: bool,

    #[serde(default = "default_true")]
    pub show_system_info: bool,

    #[serde(default = "default_true")]
    pub show_header: bool,

    #[serde(default = "default_true")]
    pub show_greeting: bool,

    #[serde(default = "default_true")]
    pub show_contributions: bool,

    #[serde(default)]
    pub avatar_width: Option<usize>,

    #[serde(default)]
    pub nerd_font_icons: bool,

    #[serde(default = "default_max_contrib_weeks")]
    pub max_contrib_weeks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconsConfig {
    #[serde(default = "default_ghost_icon")]
    pub ghost: String,

    #[serde(default)]
    pub overrides: std::collections::HashMap<String, String>,
}

fn default_username() -> String {
    String::new()
}

fn default_cache_ttl_avatar() -> u64 {
    24
}
fn default_cache_ttl_profile() -> u64 {
    1
}
fn default_cache_ttl_contrib() -> u64 {
    2
}
fn default_cache_dir() -> String {
    ".cache/ghfetch".to_string()
}
fn default_true() -> bool {
    true
}
fn default_max_contrib_weeks() -> usize {
    53
}
fn default_ghost_icon() -> String {
    "\u{1F47B}".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: default_username(),
            github_token: None,
            cache: CacheConfig::default(),
            display: DisplayConfig::default(),
            icons: IconsConfig::default(),
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            avatar_ttl_hours: default_cache_ttl_avatar(),
            profile_ttl_hours: default_cache_ttl_profile(),
            contributions_ttl_hours: default_cache_ttl_contrib(),
            cache_dir: default_cache_dir(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            show_avatar: true,
            show_system_info: true,
            show_header: false,
            show_greeting: true,
            show_contributions: true,
            avatar_width: None,
            nerd_font_icons: false,
            max_contrib_weeks: default_max_contrib_weeks(),
        }
    }
}

impl Default for IconsConfig {
    fn default() -> Self {
        Self {
            ghost: default_ghost_icon(),
            overrides: std::collections::HashMap::new(),
        }
    }
}

fn config_path() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("GHFETCH_CONFIG") {
        return Some(PathBuf::from(p));
    }
    dirs::config_dir().map(|d| d.join("ghfetch").join("config.toml"))
}

fn resolve_cache_dir(raw: &str) -> PathBuf {
    if raw.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            let rest = raw.strip_prefix("~/").unwrap_or(raw.strip_prefix('~').unwrap_or(""));
            return home.join(rest);
        }
    }
    if raw == ".cache/ghfetch" {
        if let Some(cache) = dirs::cache_dir() {
            return cache.join("ghfetch");
        }
    }
    if raw.starts_with('.') {
        if let Some(home) = dirs::home_dir() {
            return home.join(raw);
        }
    }
    PathBuf::from(raw)
}

pub fn load() -> Config {
    let path = config_path();
    let mut cfg = Config::default();

    if let Some(ref p) = path {
        if p.exists() {
            match std::fs::read_to_string(p) {
                Ok(contents) => match toml::from_str::<Config>(&contents) {
                    Ok(c) => cfg = c,
                    Err(e) => eprintln!("Warning: failed to parse config: {}", e),
                },
                Err(e) => eprintln!("Warning: failed to read config: {}", e),
            }
        }
    }

    if let Ok(token) = std::env::var("GITHUB_TOKEN") {
        cfg.github_token = Some(token);
    }

    if let Ok(user) = std::env::var("GHFETCH_USERNAME") {
        cfg.username = user;
    }

    cfg.cache.cache_dir = resolve_cache_dir(&cfg.cache.cache_dir).to_string_lossy().to_string();
    cfg
}
