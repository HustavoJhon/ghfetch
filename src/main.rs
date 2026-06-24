mod avatar;
mod cache;
mod config;
mod contributions;
mod github;
mod greeting;
mod renderer;
mod system;
mod theme;

use clap::Parser;
use std::time::Instant;

#[derive(Parser)]
#[command(name = "ghfetch", version, about = "A Neofetch-inspired CLI for GitHub profiles")]
struct Cli {
    #[arg(short, long)]
    username: Option<String>,

    #[arg(short = 'c', long)]
    config: Option<String>,

    #[arg(long)]
    no_avatar: bool,

    #[arg(long)]
    no_system: bool,

    #[arg(long)]
    no_header: bool,

    #[arg(long)]
    no_contributions: bool,

    #[arg(short = 'w', long)]
    avatar_width: Option<usize>,

    #[arg(short, long)]
    max_weeks: Option<usize>,

    #[arg(long)]
    nerd_font_icons: bool,
}

fn detect_github_user() -> Option<String> {
    if let Ok(out) = std::process::Command::new("git")
        .args(["config", "--global", "github.user"])
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if !s.is_empty() {
            return Some(s);
        }
    }

    if let Ok(out) = std::process::Command::new("gh")
        .args(["auth", "status"])
        .output()
    {
        let s = String::from_utf8_lossy(&out.stdout);
        for line in s.lines() {
            if let Some(idx) = line.find("Logged in to github.com as ") {
                let rest = &line[idx + "Logged in to github.com as ".len()..];
                return Some(rest.trim().to_string());
            }
        }
    }

    None
}

fn main() {
    let start = Instant::now();
    let cli = Cli::parse();

    if let Some(ref p) = cli.config {
        std::env::set_var("GHFETCH_CONFIG", p);
    }

    let mut config = config::load();

    if let Some(ref u) = cli.username {
        config.username = u.clone();
    }

    if config.username.is_empty() {
        if let Some(detected) = detect_github_user() {
            config.username = detected;
        } else {
            eprintln!("ghfetch: no se pudo detectar tu usuario de GitHub.");
            eprintln!("  Configuralo con:  ghfetch -u TU_USUARIO");
            eprintln!("  O en el archivo:  ~/.config/ghfetch/config.toml");
            eprintln!("  O ejecuta:        git config --global github.user TU_USUARIO");
            std::process::exit(1);
        }
    }

    if cli.no_avatar {
        config.display.show_avatar = false;
    }
    if cli.no_system {
        config.display.show_system_info = false;
    }
    if cli.no_header {
        config.display.show_header = false;
    }
    if cli.no_contributions {
        config.display.show_contributions = false;
    }
    if let Some(w) = cli.avatar_width {
        config.display.avatar_width = Some(w);
    }
    if let Some(w) = cli.max_weeks {
        config.display.max_contrib_weeks = w;
    }
    if cli.nerd_font_icons {
        config.display.nerd_font_icons = true;
    }

    let cache = cache::Cache::new(&config.cache.cache_dir);

    let term_width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80);

    let github_client = github::GitHubClient::new(config.github_token.clone(), cache.clone());

    let profile = github_client.fetch_profile(&config.username, config.cache.profile_ttl_hours);

    let system = system::gather();

    let contributions = if config.display.show_contributions {
        github_client.fetch_contributions(&config.username, config.cache.contributions_ttl_hours)
    } else {
        None
    };

    let right_height = renderer::right_panel_height(&system, contributions.as_ref(), &config);
    let desired_width = right_height;
    let max_avatar_width = term_width.saturating_sub(45) / 2;
    let avatar_width = config.display.avatar_width.unwrap_or_else(|| {
        desired_width.clamp(16, max_avatar_width.max(16))
    });

    let avatar = if config.display.show_avatar {
        if let Some(ref p) = profile {
            avatar::download_avatar(&p.avatar_url, &cache, config.cache.avatar_ttl_hours);
            avatar::fetch_ascii_avatar(
                &p.avatar_url,
                avatar_width,
                &cache,
                config.cache.avatar_ttl_hours,
            )
        } else {
            None
        }
    } else {
        None
    };

    let output = renderer::render(
        avatar.as_ref(),
        &system,
        contributions.as_ref(),
        &config,
    );

    for line in &output.lines {
        println!("{}", line);
    }

    let elapsed = start.elapsed();
    eprintln!(
        "\n{}fetched in {:.0}ms{}",
        theme::Ansi::fg_gruv(theme::Gruvbox::FG4),
        elapsed.as_secs_f64() * 1000.0,
        theme::Ansi::reset(),
    );
}
