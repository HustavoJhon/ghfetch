#![allow(dead_code)]

use chrono::{Local, Timelike};

pub struct Greeting {
    pub emoji: &'static str,
    pub text: String,
}

pub fn generate(name: &str) -> Greeting {
    let now = Local::now();
    let hour = now.hour();

    let greeting = if hour < 12 {
        Greeting {
            emoji: "\u{1F305}",
            text: format!("Buenos d\u{ed}as, {}", name),
        }
    } else if hour < 18 {
        Greeting {
            emoji: "\u{2600}\u{FE0F}",
            text: format!("Buenas tardes, {}", name),
        }
    } else {
        Greeting {
            emoji: "\u{1F319}",
            text: format!("Buenas noches, {}", name),
        }
    };

    greeting
}
