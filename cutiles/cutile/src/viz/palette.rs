//! reson8Labs TQEC visual language — navy void, cyan-gold core, purple HUP, amber survivors.

/// RGB triplet for palette colours.
#[derive(Debug, Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

pub mod tqec {
    use super::Rgb;

    pub const VOID: Rgb = Rgb(6, 10, 22);
    pub const CORE_CYAN: Rgb = Rgb(0, 200, 255);
    pub const CORE_GOLD: Rgb = Rgb(255, 210, 80);
    pub const HUP_PURPLE: Rgb = Rgb(140, 60, 220);
    pub const SURVIVOR_AMBER: Rgb = Rgb(255, 180, 0);
    pub const SYNDROME_CYAN: Rgb = Rgb(80, 220, 240);
    pub const MIRROR_OBSERVER: Rgb = Rgb(180, 120, 255);
    pub const SRAC_BRAID: Rgb = Rgb(255, 140, 60);
}

/// Map normalized entropy in `[0, 1]` to HUP purple intensity blended into void.
pub fn entropy_to_hup(entropy_norm: f32) -> Rgb {
    let t = entropy_norm.clamp(0.0, 1.0);
    let void = tqec::VOID;
    let hup = tqec::HUP_PURPLE;
    Rgb(
        lerp_u8(void.0, hup.0, t),
        lerp_u8(void.1, hup.1, t),
        lerp_u8(void.2, hup.2, t),
    )
}

/// Surge flag: amber survivor when stable, gold flash when surge detected.
pub fn surge_color(surge: f32) -> Rgb {
    if surge > 0.5 {
        tqec::CORE_GOLD
    } else {
        tqec::SURVIVOR_AMBER
    }
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (f32::from(a) + t * f32::from(b.saturating_sub(a))).round() as u8
}