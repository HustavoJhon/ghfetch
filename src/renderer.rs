use crate::avatar::AvatarArt;
use crate::config::Config;
use crate::contributions;
use crate::github::ContributionData;
use crate::system::SystemInfo;
use crate::theme::{Ansi, Gruvbox, Icons};

pub fn right_panel_height(
    system: &SystemInfo,
    contributions: Option<&ContributionData>,
    config: &Config,
) -> usize {
    let sys = build_system_lines(system, config, None).len();
    let contrib = if config.display.show_contributions {
        if let Some(c) = contributions {
            contributions::render_graph(c, config.display.max_contrib_weeks)
                .lines()
                .count()
        } else {
            0
        }
    } else {
        0
    };
    sys + 1 + contrib
}

pub struct Output {
    pub lines: Vec<String>,
}

pub fn render(
    avatar: Option<&AvatarArt>,
    system: &SystemInfo,
    contributions: Option<&ContributionData>,
    config: &Config,
) -> Output {
    let term_width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(80);

    let sys_lines = build_system_lines(system, config, None);

    let mut contrib_lines: Vec<String> = Vec::new();
    if config.display.show_contributions {
        if let Some(contrib) = contributions {
            let graph = contributions::render_graph(contrib, config.display.max_contrib_weeks);
            contrib_lines = graph.lines().map(|l| l.to_string()).collect();
        }
    }

    let right_lines: Vec<String> = sys_lines
        .iter()
        .chain(std::iter::once(&String::new()))
        .chain(contrib_lines.iter())
        .cloned()
        .collect();

    let av_lines = avatar
        .map(|a| a.lines.clone())
        .unwrap_or_default();

    let avatar_width = av_lines.first().map(|l| strip_ansi(l).chars().count()).unwrap_or(0);
    let padding = if avatar_width > 0 { 4usize } else { 0 };
    let right_width = term_width.saturating_sub(avatar_width + padding);

    let mut lines: Vec<String> = Vec::new();
    lines.push(String::new());

    let max_height = av_lines.len().max(right_lines.len());

    for i in 0..max_height {
        let mut line = String::new();
        let av = av_lines.get(i);
        let ri = right_lines.get(i);

        if let Some(a) = av {
            line.push_str(a);
            let clean = strip_ansi(a).chars().count();
            let fill = avatar_width.saturating_sub(clean) + padding;
            line.push_str(&" ".repeat(fill));
        } else if avatar_width > 0 {
            line.push_str(&" ".repeat(avatar_width + padding));
        }

        if let Some(r) = ri {
            let truncated = truncate_to_width(r, right_width);
            line.push_str(&truncated);
        }

        lines.push(line);
    }

    lines.push(String::new());
    Output { lines }
}

fn truncate_to_width(s: &str, max_width: usize) -> String {
    if strip_ansi(s).chars().count() <= max_width {
        return s.to_string();
    }
    let mut result = String::new();
    let mut visible = 0usize;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            result.push(c);
            while let Some(n) = chars.next() {
                result.push(n);
                if n == 'm' {
                    break;
                }
            }
        } else {
            if visible >= max_width {
                break;
            }
            visible += 1;
            result.push(c);
        }
    }
    result.push_str("\x1b[0m");
    result
}

fn strip_ansi(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            for n in chars.by_ref() {
                if n == 'm' {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

fn build_system_lines(system: &SystemInfo, config: &Config, _max_width: Option<usize>) -> Vec<String> {
    let icon = |key: &str| -> String {
        if config.display.nerd_font_icons {
            config.icons.overrides.get(key)
                .cloned()
                .unwrap_or_else(|| Icons::get(key).to_string())
        } else {
            config.icons.ghost.clone()
        }
    };

    let fmt_val = |v: &str, c: u8| format!("{}{}{}", Ansi::fg_gruv(c), v, Ansi::reset());
    let fmt_lbl = |l: &str, c: u8| format!("{}{}{}", Ansi::fg_gruv(c), l, Ansi::reset());
    let fmt_icon = |icon_key: &str, c: u8| -> String {
        let i = icon(icon_key);
        format!("{}{}{}", Ansi::fg_gruv(c), i, Ansi::reset())
    };

    let items: Vec<(&str, String, String, u8)> = vec![
        ("os", system.os.clone(), "OS".into(), Gruvbox::GREEN),
        ("kernel", system.kernel.clone(), "Kernel".into(), Gruvbox::YELLOW),
        ("hostname", system.hostname.clone(), "Host".into(), Gruvbox::BLUE),
        ("cpu", system.cpu.clone(), "CPU".into(), Gruvbox::ORANGE),
        ("gpu", system.gpu.clone(), "GPU".into(), Gruvbox::PURPLE),
        ("ram", format!("{} / {}", system.ram_used_str(), system.ram_total_str()), "RAM".into(), Gruvbox::AQUA),
        ("disk", if system.disk_total > 0 {
            format!("{} / {}", system.disk_used_str(), system.disk_total_str())
        } else {
            "N/A".into()
        }, "Disk".into(), Gruvbox::FG0),
        ("shell", system.shell.clone(), "Shell".into(), Gruvbox::GREEN),
        ("terminal", system.terminal.clone(), "Terminal".into(), Gruvbox::YELLOW),
        ("resolution", system.resolution.clone(), "Res".into(), Gruvbox::BLUE),
        ("uptime", system.uptime.clone(), "Uptime".into(), Gruvbox::ORANGE),
        ("arch", system.arch.clone(), "Arch".into(), Gruvbox::PURPLE),
    ];

    let mut lines = Vec::new();
    for (key, value_str, label_str, color) in &items {
        let icon_str = fmt_icon(key, *color);
        let full = format!("{} {}  {}", icon_str, fmt_lbl(label_str, *color), fmt_val(value_str, *color));
        lines.push(full);
    }

    lines
}
