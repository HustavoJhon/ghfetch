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
    #[arg(short, long, default_value = "hustavojhon")]
    username: String,

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

fn main() {
    let start = Instant::now();
    let cli = Cli::parse();

    if let Some(ref p) = cli.config {
        std::env::set_var("GHFETCH_CONFIG", p);
    }

    let mut config = config::load();
    config.username = cli.username;

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

    let avatar_width = config.display.avatar_width.unwrap_or({
        if term_width >= 120 {
            35
        } else if term_width >= 100 {
            28
        } else if term_width >= 80 {
            22
        } else {
            16
        }
    });

    let github_client = github::GitHubClient::new(config.github_token.clone(), cache.clone());

    let profile = github_client.fetch_profile(&config.username, config.cache.profile_ttl_hours);

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

    let system = system::gather();

    let contributions = if config.display.show_contributions {
        github_client.fetch_contributions(&config.username, config.cache.contributions_ttl_hours)
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
