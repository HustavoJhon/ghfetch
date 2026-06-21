use crate::github::ContributionData;
use crate::theme::{Ansi, Gruvbox};
use chrono::Local;

pub fn render_graph(data: &ContributionData, _max_weeks: usize) -> String {
    let weeks = &data.weeks;
    if weeks.is_empty() {
        return String::new();
    }

    let now = Local::now();
    let year_start = format!("{}-01-01", now.format("%Y"));

    let filtered: Vec<&crate::github::ContributionWeek> = weeks
        .iter()
        .filter(|w| {
            w.days.iter().any(|d| d.date >= year_start)
        })
        .collect();

    if filtered.is_empty() {
        return String::new();
    }

    let total: u32 = filtered
        .iter()
        .flat_map(|w| w.days.iter())
        .filter(|d| d.date >= year_start)
        .map(|d| d.count)
        .sum();

    let cutoff = year_start;

    let mut output = String::new();

    output.push_str(&format!(
        "{}{} contribuciones{}{}  ",
        Ansi::fg_gruv(Gruvbox::YELLOW),
        total,
        Ansi::reset(),
        Ansi::fg_gruv(Gruvbox::FG3),
    ));
    output.push_str(&format!(
        "{}Menos{}{}",
        Ansi::fg_gruv(Gruvbox::BG3),
        Ansi::reset(),
        bg_cell(Gruvbox::BG1),
    ));
    output.push_str(&format!("{}", bg_cell(Gruvbox::GREEN)));
    output.push_str(&format!("{}", bg_cell(Gruvbox::YELLOW)));
    output.push_str(&format!("{}", bg_cell(Gruvbox::ORANGE)));
    output.push_str(&format!(
        "  {}{}Mas{}",
        Ansi::fg_gruv(Gruvbox::BG3),
        Ansi::reset(),
        Ansi::fg_gruv(Gruvbox::BG3),
    ));
    output.push('\n');
    output.push('\n');

    for day_of_week in 0..7 {
        let mut row = String::new();
        for week in &filtered {
            if day_of_week < week.days.len() {
                let day = &week.days[day_of_week];
                if day.date >= cutoff {
                    let level = contribution_level(day.count);
                    row.push_str(&cell_for_level(level));
                } else {
                    row.push_str(&bg_cell(Gruvbox::BG));
                }
            } else {
                row.push_str(&bg_cell(Gruvbox::BG));
            }
        }
        output.push_str("  ");
        output.push_str(&row);
        output.push('\n');
    }

    output
}

fn bg_cell(color: u8) -> String {
    let (r, g, b) = Gruvbox::rgb(color);
    format!("{}{}{}", Ansi::bg_rgb(r, g, b), ' ', Ansi::reset())
}

fn cell_for_level(level: u32) -> String {
    let color = match level {
        0 => Gruvbox::BG1,
        1 => Gruvbox::AQUA,
        2 => Gruvbox::GREEN,
        3 => Gruvbox::YELLOW,
        _ => Gruvbox::ORANGE,
    };
    bg_cell(color)
}

fn contribution_level(count: u32) -> u32 {
    match count {
        0 => 0,
        1..=3 => 1,
        4..=9 => 2,
        10..=19 => 3,
        _ => 4,
    }
}
