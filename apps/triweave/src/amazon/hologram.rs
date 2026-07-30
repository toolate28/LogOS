//! Hologram rendering — armor stands with floating text for search results.
//!
//! Each search result is rendered as:
//! - Invisible armor stand with CustomName (product title)
//! - Glowing marker for visual indicator
//! - Positioned on the 3x3 pedestal grid
//!
//! Conservation: α + ω = 15

use serde::{Deserialize, Serialize};

/// A search result from Vectorize.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub price: Option<String>,
    pub category: Option<String>,
    pub score: f64,
    pub metadata_id: String,
}

/// Block position in Minecraft world.
#[derive(Debug, Clone, Copy)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

/// Origin of the hologram grid inside Amazon Room.
pub const HOLOGRAM_ORIGIN: BlockPos = BlockPos {
    x: super::blueprint::ORIGIN_X + 8,
    y: super::blueprint::ORIGIN_Y + 3,
    z: super::blueprint::ORIGIN_Z + 8,
};

/// Generate RCON commands to clear all existing holograms.
pub fn clear_holograms() -> Vec<String> {
    vec![
        "/kill @e[tag=amazon_hologram]".to_string(),
        "/kill @e[tag=amazon_price]".to_string(),
    ]
}

/// Generate RCON commands to render search results as holograms.
///
/// Places up to 9 results on the 3x3 pedestal grid.
pub fn render_holograms(results: &[SearchResult]) -> Vec<String> {
    let mut cmds = clear_holograms();

    for (i, result) in results.iter().take(9).enumerate() {
        let col = (i % 3) as i32;
        let row = (i / 3) as i32;
        let x = HOLOGRAM_ORIGIN.x + col * 6;
        let y = HOLOGRAM_ORIGIN.y;
        let z = HOLOGRAM_ORIGIN.z + row * 6;

        // Title hologram (armor stand with custom name)
        let escaped_title = result.title.replace('"', "\\\"");
        cmds.push(format!(
            r#"/summon armor_stand {} {} {} {{CustomName:'{{"text":"{}","color":"gold"}}',CustomNameVisible:1b,Invisible:1b,Marker:1b,Tags:["amazon_hologram","reson8_npc"],NoGravity:1b}}"#,
            x, y, z, escaped_title
        ));

        // Price hologram (below title)
        if let Some(price) = &result.price {
            cmds.push(format!(
                r#"/summon armor_stand {} {} {} {{CustomName:'{{"text":"{}","color":"green"}}',CustomNameVisible:1b,Invisible:1b,Marker:1b,Tags:["amazon_price","reson8_npc"],NoGravity:1b}}"#,
                x, y - 1, z, price
            ));
        }

        // Score hologram (WAVE relevance)
        cmds.push(format!(
            r#"/summon armor_stand {} {} {} {{CustomName:'{{"text":"Φ {:.3}","color":"aqua"}}',CustomNameVisible:1b,Invisible:1b,Marker:1b,Tags:["amazon_hologram","reson8_npc"],NoGravity:1b}}"#,
            x, y - 2, z, result.score
        ));

        // Particle effect on pedestal
        cmds.push(format!(
            "/particle end_rod {} {} {} 0.2 0.5 0.2 0.02 10",
            x, y - 1, z
        ));
    }

    cmds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_holograms() {
        let results = vec![
            SearchResult {
                title: "Rust Programming Book".to_string(),
                price: Some("$39.99".to_string()),
                category: Some("Books".to_string()),
                score: 0.95,
                metadata_id: "B001".to_string(),
            },
            SearchResult {
                title: "RTX 5090 GPU".to_string(),
                price: Some("$1,999.99".to_string()),
                category: Some("Electronics".to_string()),
                score: 0.87,
                metadata_id: "E001".to_string(),
            },
        ];

        let cmds = render_holograms(&results);
        // clear (2) + title (2) + price (2) + score (2) + particles (2) = 10
        assert!(cmds.len() >= 8);
        assert!(cmds[0].contains("kill"));
        assert!(cmds[2].contains("Rust Programming Book"));
    }
}
