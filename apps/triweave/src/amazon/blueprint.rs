//! Amazon Room blueprint — mcfunction generator.
//!
//! Generates Minecraft commands to build the Amazon Room structure
//! at Zone 5 (x:100, z:0) in Coherence City.
//!
//! Structure: 32x12x32 obsidian/glass room with:
//! - Central search podium (lectern)
//! - 3x3 hologram pedestal grid (9 result slots)
//! - Redstone comparator WAVE display
//! - Chained chest wall → Blockchain Bank
//!
//! Conservation: α + ω = 15

/// Origin coordinates for the Amazon Room.
pub const ORIGIN_X: i32 = 100;
pub const ORIGIN_Y: i32 = 64;
pub const ORIGIN_Z: i32 = 0;
pub const WIDTH: i32 = 32;
pub const HEIGHT: i32 = 12;
pub const DEPTH: i32 = 32;

/// Generate all RCON commands to build the Amazon Room.
pub fn generate_room_commands() -> Vec<String> {
    let mut cmds = Vec::new();

    let x = ORIGIN_X;
    let y = ORIGIN_Y;
    let z = ORIGIN_Z;

    // ── Floor: obsidian ─────────────────────────────────────────────
    cmds.push(format!(
        "/fill {} {} {} {} {} {} obsidian",
        x, y, z,
        x + WIDTH - 1, y, z + DEPTH - 1
    ));

    // ── Walls: black stained glass ──────────────────────────────────
    // North wall
    cmds.push(format!(
        "/fill {} {} {} {} {} {} black_stained_glass",
        x, y + 1, z,
        x + WIDTH - 1, y + HEIGHT - 1, z
    ));
    // South wall
    cmds.push(format!(
        "/fill {} {} {} {} {} {} black_stained_glass",
        x, y + 1, z + DEPTH - 1,
        x + WIDTH - 1, y + HEIGHT - 1, z + DEPTH - 1
    ));
    // West wall
    cmds.push(format!(
        "/fill {} {} {} {} {} {} black_stained_glass",
        x, y + 1, z,
        x, y + HEIGHT - 1, z + DEPTH - 1
    ));
    // East wall
    cmds.push(format!(
        "/fill {} {} {} {} {} {} black_stained_glass",
        x + WIDTH - 1, y + 1, z,
        x + WIDTH - 1, y + HEIGHT - 1, z + DEPTH - 1
    ));

    // ── Ceiling: sea lanterns ───────────────────────────────────────
    cmds.push(format!(
        "/fill {} {} {} {} {} {} sea_lantern",
        x, y + HEIGHT, z,
        x + WIDTH - 1, y + HEIGHT, z + DEPTH - 1
    ));

    // ── Interior air ────────────────────────────────────────────────
    cmds.push(format!(
        "/fill {} {} {} {} {} {} air",
        x + 1, y + 1, z + 1,
        x + WIDTH - 2, y + HEIGHT - 1, z + DEPTH - 2
    ));

    // ── Central search podium (lectern) ─────────────────────────────
    let cx = x + WIDTH / 2;
    let cz = z + DEPTH / 2;
    cmds.push(format!(
        "/setblock {} {} {} quartz_block",
        cx, y + 1, cz
    ));
    cmds.push(format!(
        "/setblock {} {} {} lectern[has_book=true]",
        cx, y + 2, cz
    ));

    // ── 3x3 Hologram pedestals ──────────────────────────────────────
    for row in 0..3 {
        for col in 0..3 {
            let px = x + 8 + col * 6;
            let pz = z + 8 + row * 6;
            // Pedestal block
            cmds.push(format!(
                "/setblock {} {} {} end_stone_bricks",
                px, y + 1, pz
            ));
            // Light below
            cmds.push(format!(
                "/setblock {} {} {} sea_lantern",
                px, y, pz
            ));
        }
    }

    // ── Amazon Merchant NPC ─────────────────────────────────────────
    cmds.push(format!(
        r#"/summon villager {} {} {} {{CustomName:'{{"text":"Amazon Merchant","color":"gold","bold":true}}',Tags:["reson8_npc","amazon_merchant","omega_component"],Invulnerable:1b,Glowing:1b,NoAI:1b}}"#,
        cx + 2, y + 1, cz
    ));

    // ── Sign: Amazon.com Room ───────────────────────────────────────
    cmds.push(format!(
        r#"/setblock {} {} {} oak_wall_sign[facing=south]{{front_text:{{messages:['"{{"text":"Amazon.com Room","color":"gold"}}"','"{{"text":"Zone 5 | R=144","color":"gray"}}"','"{{"text":"Vectorize Search","color":"aqua"}}"','"{{"text":"α + ω = 15","color":"green"}}"']}}}}"#,
        cx, y + 3, z + 1
    ));

    cmds
}
