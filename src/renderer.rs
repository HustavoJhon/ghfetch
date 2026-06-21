use crate::avatar::AvatarArt;
use crate::config::Config;
use crate::contributions;
use crate::github::ContributionData;
use crate::system::SystemInfo;
use crate::theme::{Ansi, Gruvbox, Icons};

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

    let layout = determine_layout(term_width, avatar.map(|a| a.width).unwrap_or(0));

    let mut lines: Vec<String> = Vec::new();
    lines.push(String::new());

    match layout {
        Layout::Horizontal {
            _avatar_width: _,
            info_width,
        } => {
            let av_lines = avatar
                .map(|a| a.lines.clone())
                .unwrap_or_else(|| vec![String::new()]);
            let info_lines = build_system_lines(system, config, Some(info_width));

            let total_av_height = av_lines.len();
            let total_info_height = info_lines.len();
            let max_height = total_av_height.max(total_info_height);

            for i in 0..max_height {
                let mut line = String::new();
                let av = av_lines.get(i);
                let inf = info_lines.get(i);

                match (av, inf) {
                    (Some(a), Some(inf)) => {
                        let clean_av = strip_ansi(a);
                        let width = clean_av.chars().count();
                        line.push_str(a);
                        let padding = 4usize
                            .saturating_sub(width.saturating_sub(av_lines[0].len()));
                        if padding > 0 {
                            line.push_str(&" ".repeat(padding));
                        }
                        line.push_str("  ");
                        line.push_str(inf);
                    }
                    (Some(a), None) => {
                        line.push_str(a);
                    }
                    (None, Some(inf)) => {
                        let av_width = av_lines
                            .first()
                            .map(|l| strip_ansi(l).chars().count())
                            .unwrap_or(0);
                        line.push_str(&" ".repeat(av_width + 4));
                        line.push_str(inf);
                    }
                    (None, None) => {}
                }
                lines.push(line);
            }
        }
        Layout::Vertical => {
            if let Some(a) = avatar {
                for line in &a.lines {
                    lines.push(line.clone());
                }
                lines.push(String::new());
            }

            for line in build_system_lines(system, config, None) {
                lines.push(line);
            }
        }
    }
    lines.push(String::new());

    if config.display.show_contributions {
        if let Some(contrib) = contributions {
            lines.push(contributions::render_graph(
                contrib,
                config.display.max_contrib_weeks,
            ));
        }
    }

    lines.push(String::new());
    Output { lines }
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

fn build_system_lines(system: &SystemInfo, config: &Config, max_width: Option<usize>) -> Vec<String> {
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
        if let Some(mw) = max_width {
            if strip_ansi(&full).chars().count() > mw {
                let short = format!("{} {}  {}", icon_str, fmt_lbl(label_str, *color), fmt_val(value_str, *color));
                lines.push(short);
            } else {
                lines.push(full);
            }
        } else {
            lines.push(full);
        }
    }

    lines
}

#[derive(Debug)]
enum Layout {
    Horizontal {
        _avatar_width: usize,
        info_width: usize,
    },
    Vertical,
}

fn determine_layout(term_width: usize, avatar_width: usize) -> Layout {
    if avatar_width == 0 {
        return Layout::Vertical;
    }

    let info_min = 42;
    let min_side_by_side = avatar_width + 4 + info_min;

    if term_width >= min_side_by_side {
        let remaining = term_width.saturating_sub(avatar_width + 4);
        Layout::Horizontal {
            _avatar_width: avatar_width,
            info_width: remaining.min(80),
        }
    } else {
        Layout::Vertical
    }
}
