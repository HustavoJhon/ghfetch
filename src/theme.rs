pub struct Gruvbox;

#[allow(dead_code)]
impl Gruvbox {
    pub const BG: u8 = 0;
    pub const BG0_H: u8 = 1;
    pub const BG0: u8 = 2;
    pub const BG1: u8 = 3;
    pub const BG2: u8 = 4;
    pub const BG3: u8 = 5;
    pub const BG4: u8 = 6;
    pub const FG: u8 = 7;
    pub const FG0: u8 = 8;
    pub const FG1: u8 = 9;
    pub const FG2: u8 = 10;
    pub const FG3: u8 = 11;
    pub const FG4: u8 = 12;
    pub const RED: u8 = 13;
    pub const GREEN: u8 = 14;
    pub const YELLOW: u8 = 15;
    pub const BLUE: u8 = 16;
    pub const PURPLE: u8 = 17;
    pub const AQUA: u8 = 18;
    pub const ORANGE: u8 = 19;

    pub const COLORS: [[u8; 3]; 20] = [
        [0x28, 0x28, 0x28], // bg
        [0x1d, 0x20, 0x21], // bg0_h
        [0x28, 0x28, 0x28], // bg0
        [0x3c, 0x38, 0x36], // bg1
        [0x50, 0x49, 0x45], // bg2
        [0x66, 0x5c, 0x54], // bg3
        [0x7c, 0x6f, 0x64], // bg4
        [0xfb, 0xf1, 0xc7], // fg
        [0xeb, 0xdb, 0xb2], // fg0
        [0xd5, 0xc4, 0xa1], // fg1
        [0xbd, 0xae, 0x93], // fg2
        [0xa8, 0x99, 0x84], // fg3
        [0x92, 0x83, 0x74], // fg4
        [0xcc, 0x24, 0x1d], // red
        [0x98, 0x97, 0x1a], // green
        [0xd7, 0x99, 0x21], // yellow
        [0x45, 0x85, 0x88], // blue
        [0xb1, 0x62, 0x86], // purple
        [0x68, 0x9d, 0x6a], // aqua
        [0xd6, 0x5d, 0x0e], // orange
    ];

    pub fn rgb(idx: u8) -> (u8, u8, u8) {
        let c = Self::COLORS[idx as usize];
        (c[0], c[1], c[2])
    }

    pub fn nearest(r: u8, g: u8, b: u8) -> u8 {
        let mut best = 0u8;
        let mut best_dist = u32::MAX;
        for (i, color) in Self::COLORS.iter().enumerate() {
            let dr = r as i32 - color[0] as i32;
            let dg = g as i32 - color[1] as i32;
            let db = b as i32 - color[2] as i32;
            let dist = (dr * dr + dg * dg + db * db) as u32;
            if dist < best_dist {
                best_dist = dist;
                best = i as u8;
            }
        }
        best
    }
}

pub struct Ansi;

impl Ansi {
    pub fn reset() -> String {
        "\x1b[0m".to_string()
    }

    pub fn bold() -> String {
        "\x1b[1m".to_string()
    }

    pub fn fg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }

    pub fn bg_rgb(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{};{};{}m", r, g, b)
    }

    pub fn fg_gruv(idx: u8) -> String {
        let (r, g, b) = Gruvbox::rgb(idx);
        Self::fg_rgb(r, g, b)
    }

    #[allow(dead_code)]
    pub fn bg_gruv(idx: u8) -> String {
        let (r, g, b) = Gruvbox::rgb(idx);
        Self::bg_rgb(r, g, b)
    }

    #[allow(dead_code)]
    pub fn colored(text: &str, color_idx: u8) -> String {
        format!("{}{}{}", Self::fg_gruv(color_idx), text, Self::reset())
    }

    #[allow(dead_code)]
    pub fn colored_bold(text: &str, color_idx: u8) -> String {
        format!("{}{}{}{}", Self::bold(), Self::fg_gruv(color_idx), text, Self::reset())
    }
}

pub struct Icons;

impl Icons {
    pub const GHOST: &str = "\u{1F47B}";

    pub const SYSTEM_ICONS: &[(&str, &str)] = &[
        ("os", "\u{1F47B}"),
        ("kernel", "\u{1F47B}"),
        ("hostname", "\u{1F47B}"),
        ("cpu", "\u{1F47B}"),
        ("gpu", "\u{1F47B}"),
        ("ram", "\u{1F47B}"),
        ("disk", "\u{1F47B}"),
        ("shell", "\u{1F47B}"),
        ("terminal", "\u{1F47B}"),
        ("resolution", "\u{1F47B}"),
        ("uptime", "\u{1F47B}"),
        ("arch", "\u{1F47B}"),
    ];

    pub fn get(key: &str) -> &str {
        Self::SYSTEM_ICONS
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v)
            .unwrap_or(Self::GHOST)
    }
}
