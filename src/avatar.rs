use crate::cache::Cache;
use crate::theme::{Ansi, Gruvbox};
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CachedAvatar {
    pixels: Vec<Vec<[u8; 3]>>,
    width: usize,
    height: usize,
}

pub struct AvatarArt {
    pub lines: Vec<String>,
    pub width: usize,
}

pub fn fetch_ascii_avatar(
    avatar_url: &str,
    target_width: usize,
    cache: &Cache,
    ttl_hours: u64,
) -> Option<AvatarArt> {
    let cache_key = format!(
        "avatar_v3_{}_{}",
        avatar_url.split('/').last().unwrap_or("avatar"),
        target_width
    );

    if let Some(cached) = cache.get::<CachedAvatar>(&cache_key, ttl_hours) {
        return Some(AvatarArt {
            lines: cached
                .pixels
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|rgb| {
                            format!(
                                "{}  {}",
                                Ansi::bg_rgb(rgb[0], rgb[1], rgb[2]),
                                Ansi::reset()
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect(),
            width: cached.width * 2,
        });
    }

    let raw_img = cache.get_raw(&format!("avatar_raw_{}", avatar_url), ttl_hours)?;
    let img = image::ImageReader::new(Cursor::new(raw_img))
        .with_guessed_format()
        .ok()?
        .decode()
        .ok()?;

    let (iw, ih) = img.dimensions();
    let aspect_ratio = ih as f64 / iw as f64;
    let char_aspect = 2.0;
    let target_height = ((target_width as f64 * aspect_ratio) / char_aspect) as usize;
    let target_height = target_height.max(5);

    let resized = img.resize_exact(
        target_width as u32,
        target_height as u32,
        image::imageops::FilterType::Lanczos3,
    );

    let bg = Gruvbox::COLORS[Gruvbox::BG as usize];

    let mut pixels: Vec<Vec<[u8; 3]>> = Vec::new();

    for y in 0..target_height {
        let mut row: Vec<[u8; 3]> = Vec::new();
        for x in 0..target_width {
            let pixel = resized.get_pixel(x as u32, y as u32);
            let rgba = pixel.0;
            let alpha = rgba[3] as f32 / 255.0;

            let (r, g, b) = if alpha < 0.3 {
                (bg[0], bg[1], bg[2])
            } else {
                let r = (rgba[0] as f32 * alpha + bg[0] as f32 * (1.0 - alpha)) as u8;
                let g = (rgba[1] as f32 * alpha + bg[1] as f32 * (1.0 - alpha)) as u8;
                let b = (rgba[2] as f32 * alpha + bg[2] as f32 * (1.0 - alpha)) as u8;
                (r, g, b)
            };

            row.push([r, g, b]);
        }
        pixels.push(row);
    }

    let cached = CachedAvatar {
        pixels: pixels.clone(),
        width: target_width,
        height: target_height,
    };
    cache.set(&cache_key, &cached);

    let art_lines = pixels
        .iter()
        .map(|row| {
            row.iter()
                .map(|rgb| {
                    format!(
                        "{}  {}",
                        Ansi::bg_rgb(rgb[0], rgb[1], rgb[2]),
                        Ansi::reset()
                    )
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect();

    let visible_width = target_width * 2;

    Some(AvatarArt {
        lines: art_lines,
        width: visible_width,
    })
}

pub fn download_avatar(
    avatar_url: &str,
    cache: &Cache,
    ttl_hours: u64,
) -> Option<Vec<u8>> {
    let cache_key = format!("avatar_raw_{}", avatar_url);

    if let Some(cached) = cache.get_raw(&cache_key, ttl_hours) {
        return Some(cached);
    }

    let agent = ureq::AgentBuilder::new()
        .timeout_read(std::time::Duration::from_secs(10))
        .build();

    match agent
        .get(avatar_url)
        .set("User-Agent", "ghfetch/0.1.0")
        .call()
    {
        Ok(resp) => {
            let mut data = Vec::new();
            resp.into_reader().read_to_end(&mut data).ok()?;
            cache.set_raw(&cache_key, &data);
            Some(data)
        }
        Err(e) => {
            eprintln!("Warning: failed to download avatar: {}", e);
            None
        }
    }
}
