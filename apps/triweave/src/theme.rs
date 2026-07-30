//! Global theme system for triweave TUI.
//!
//! Maps `ThemeConfig` (from ~/.triweave/config.toml) to ratatui Style values.
//! Built-in presets: coherence-dark, neon-lattice, minimal, forge-amber, ainulindale.
//!
//! Usage:
//!   let theme = Theme::from_config(&config.theme);
//!   let style = theme.border;
//!
//! Conservation: alpha + omega = 15

use ratatui::style::{Color, Modifier, Style};

use crate::saif::ThemeConfig;

/// Resolved theme — all TUI styles derived from config.
#[derive(Debug, Clone)]
pub struct Theme {
    // Panel borders
    pub border: Style,
    pub border_focused: Style,

    // Accent color (used for highlights, sparklines, active elements)
    pub accent: Style,
    pub accent_bold: Style,

    // Strand status
    pub strand_ok: Style,
    pub strand_down: Style,
    pub strand_no_key: Style,

    // WAVE scoring
    pub wave_high: Style,      // >= 0.85
    pub wave_mid: Style,       // >= 0.70
    pub wave_low: Style,       // < 0.70

    // Conservation
    pub conservation_ok: Style,
    pub conservation_fail: Style,

    // ATOM trail gates
    pub gate_kenl: Style,
    pub gate_awi: Style,
    pub gate_atom: Style,
    pub gate_saif: Style,

    // Temperature
    pub temp_cool: Style,      // < 60C
    pub temp_warm: Style,      // 60-80C
    pub temp_hot: Style,       // > 80C

    // Forge telemetry
    pub forge_label: Style,
    pub forge_value: Style,

    // Pipeline
    pub pipeline_active: Style,
    pub pipeline_bar: Style,
    pub pipeline_idle: Style,

    // Log entries
    pub log_info: Style,
    pub log_warn: Style,
    pub log_error: Style,

    // General
    pub text: Style,
    pub text_dim: Style,
    pub title: Style,
    pub version: Style,

    // Border type
    pub border_type: BorderType,
}

/// Border rendering style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BorderType {
    Rounded,
    Double,
    Thick,
    Plain,
}

impl BorderType {
    pub fn to_ratatui(self) -> ratatui::widgets::BorderType {
        match self {
            Self::Rounded => ratatui::widgets::BorderType::Rounded,
            Self::Double => ratatui::widgets::BorderType::Double,
            Self::Thick => ratatui::widgets::BorderType::Thick,
            Self::Plain => ratatui::widgets::BorderType::Plain,
        }
    }
}

impl Theme {
    /// Build a theme from config.
    pub fn from_config(config: &ThemeConfig) -> Self {
        let accent = parse_color(&config.accent);
        let border_type = parse_border(&config.border);

        match config.name.as_str() {
            "neon-lattice" => Self::neon_lattice(accent, border_type),
            "minimal" => Self::minimal(accent, border_type),
            "forge-amber" => Self::forge_amber(accent, border_type),
            "ainulindale" => Self::ainulindale(accent, border_type),
            _ => Self::coherence_dark(accent, border_type), // default
        }
    }

    /// coherence-dark — the default. Cyan accent on dark background.
    fn coherence_dark(accent: Color, border_type: BorderType) -> Self {
        Self {
            border: Style::default().fg(Color::DarkGray),
            border_focused: Style::default().fg(accent),
            accent: Style::default().fg(accent),
            accent_bold: Style::default().fg(accent).add_modifier(Modifier::BOLD),
            strand_ok: Style::default().fg(Color::Green),
            strand_down: Style::default().fg(Color::DarkGray),
            strand_no_key: Style::default().fg(Color::Yellow),
            wave_high: Style::default().fg(Color::Green),
            wave_mid: Style::default().fg(Color::Yellow),
            wave_low: Style::default().fg(Color::Red),
            conservation_ok: Style::default().fg(Color::Green),
            conservation_fail: Style::default().fg(Color::Red),
            gate_kenl: Style::default().fg(Color::Green),
            gate_awi: Style::default().fg(Color::Yellow),
            gate_atom: Style::default().fg(accent),
            gate_saif: Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            temp_cool: Style::default().fg(Color::Green),
            temp_warm: Style::default().fg(Color::Yellow),
            temp_hot: Style::default().fg(Color::Red),
            forge_label: Style::default().fg(Color::DarkGray),
            forge_value: Style::default().fg(Color::White),
            pipeline_active: Style::default().fg(Color::Yellow),
            pipeline_bar: Style::default().fg(Color::Yellow),
            pipeline_idle: Style::default().fg(Color::DarkGray),
            log_info: Style::default().fg(Color::DarkGray),
            log_warn: Style::default().fg(Color::Yellow),
            log_error: Style::default().fg(Color::Red),
            text: Style::default().fg(Color::White),
            text_dim: Style::default().fg(Color::DarkGray),
            title: Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            version: Style::default().fg(Color::DarkGray),
            border_type,
        }
    }

    /// neon-lattice — high contrast, magenta/green neon on black.
    fn neon_lattice(_accent: Color, border_type: BorderType) -> Self {
        let neon_green = Color::Rgb(0, 255, 128);
        let neon_magenta = Color::Rgb(255, 0, 200);
        let neon_cyan = Color::Rgb(0, 255, 255);
        let neon_yellow = Color::Rgb(255, 255, 0);

        Self {
            border: Style::default().fg(neon_green),
            border_focused: Style::default().fg(neon_magenta),
            accent: Style::default().fg(neon_cyan),
            accent_bold: Style::default().fg(neon_cyan).add_modifier(Modifier::BOLD),
            strand_ok: Style::default().fg(neon_green),
            strand_down: Style::default().fg(Color::Rgb(80, 80, 80)),
            strand_no_key: Style::default().fg(neon_yellow),
            wave_high: Style::default().fg(neon_green),
            wave_mid: Style::default().fg(neon_yellow),
            wave_low: Style::default().fg(Color::Rgb(255, 50, 50)),
            conservation_ok: Style::default().fg(neon_green),
            conservation_fail: Style::default().fg(neon_magenta),
            gate_kenl: Style::default().fg(neon_green),
            gate_awi: Style::default().fg(neon_yellow),
            gate_atom: Style::default().fg(neon_cyan),
            gate_saif: Style::default().fg(neon_magenta).add_modifier(Modifier::BOLD),
            temp_cool: Style::default().fg(neon_green),
            temp_warm: Style::default().fg(neon_yellow),
            temp_hot: Style::default().fg(neon_magenta),
            forge_label: Style::default().fg(Color::Rgb(100, 100, 100)),
            forge_value: Style::default().fg(neon_cyan),
            pipeline_active: Style::default().fg(neon_yellow),
            pipeline_bar: Style::default().fg(neon_green),
            pipeline_idle: Style::default().fg(Color::Rgb(60, 60, 60)),
            log_info: Style::default().fg(Color::Rgb(120, 120, 120)),
            log_warn: Style::default().fg(neon_yellow),
            log_error: Style::default().fg(neon_magenta),
            text: Style::default().fg(Color::White),
            text_dim: Style::default().fg(Color::Rgb(100, 100, 100)),
            title: Style::default().fg(neon_cyan).add_modifier(Modifier::BOLD),
            version: Style::default().fg(Color::Rgb(80, 80, 80)),
            border_type,
        }
    }

    /// minimal — grayscale with single accent color. Low visual noise.
    fn minimal(accent: Color, border_type: BorderType) -> Self {
        Self {
            border: Style::default().fg(Color::Rgb(60, 60, 60)),
            border_focused: Style::default().fg(accent),
            accent: Style::default().fg(accent),
            accent_bold: Style::default().fg(accent).add_modifier(Modifier::BOLD),
            strand_ok: Style::default().fg(Color::Rgb(180, 180, 180)),
            strand_down: Style::default().fg(Color::Rgb(80, 80, 80)),
            strand_no_key: Style::default().fg(Color::Rgb(180, 150, 80)),
            wave_high: Style::default().fg(Color::Rgb(180, 180, 180)),
            wave_mid: Style::default().fg(Color::Rgb(140, 140, 100)),
            wave_low: Style::default().fg(Color::Rgb(180, 80, 80)),
            conservation_ok: Style::default().fg(Color::Rgb(180, 180, 180)),
            conservation_fail: Style::default().fg(Color::Rgb(200, 80, 80)),
            gate_kenl: Style::default().fg(Color::Rgb(160, 160, 160)),
            gate_awi: Style::default().fg(Color::Rgb(140, 140, 140)),
            gate_atom: Style::default().fg(accent),
            gate_saif: Style::default().fg(accent).add_modifier(Modifier::BOLD),
            temp_cool: Style::default().fg(Color::Rgb(160, 160, 160)),
            temp_warm: Style::default().fg(Color::Rgb(180, 150, 80)),
            temp_hot: Style::default().fg(Color::Rgb(200, 80, 80)),
            forge_label: Style::default().fg(Color::Rgb(100, 100, 100)),
            forge_value: Style::default().fg(Color::Rgb(160, 160, 160)),
            pipeline_active: Style::default().fg(accent),
            pipeline_bar: Style::default().fg(accent),
            pipeline_idle: Style::default().fg(Color::Rgb(60, 60, 60)),
            log_info: Style::default().fg(Color::Rgb(100, 100, 100)),
            log_warn: Style::default().fg(Color::Rgb(180, 150, 80)),
            log_error: Style::default().fg(Color::Rgb(200, 80, 80)),
            text: Style::default().fg(Color::Rgb(200, 200, 200)),
            text_dim: Style::default().fg(Color::Rgb(100, 100, 100)),
            title: Style::default().fg(Color::Rgb(220, 220, 220)).add_modifier(Modifier::BOLD),
            version: Style::default().fg(Color::Rgb(80, 80, 80)),
            border_type,
        }
    }

    /// forge-amber — warm amber tones, industrial feel.
    fn forge_amber(_accent: Color, border_type: BorderType) -> Self {
        let amber = Color::Rgb(255, 176, 0);
        let ember = Color::Rgb(200, 80, 20);
        let steel = Color::Rgb(140, 140, 160);

        Self {
            border: Style::default().fg(Color::Rgb(80, 60, 20)),
            border_focused: Style::default().fg(amber),
            accent: Style::default().fg(amber),
            accent_bold: Style::default().fg(amber).add_modifier(Modifier::BOLD),
            strand_ok: Style::default().fg(amber),
            strand_down: Style::default().fg(Color::Rgb(80, 60, 40)),
            strand_no_key: Style::default().fg(ember),
            wave_high: Style::default().fg(amber),
            wave_mid: Style::default().fg(ember),
            wave_low: Style::default().fg(Color::Rgb(180, 40, 40)),
            conservation_ok: Style::default().fg(amber),
            conservation_fail: Style::default().fg(Color::Rgb(200, 40, 40)),
            gate_kenl: Style::default().fg(amber),
            gate_awi: Style::default().fg(ember),
            gate_atom: Style::default().fg(steel),
            gate_saif: Style::default().fg(amber).add_modifier(Modifier::BOLD),
            temp_cool: Style::default().fg(steel),
            temp_warm: Style::default().fg(amber),
            temp_hot: Style::default().fg(ember),
            forge_label: Style::default().fg(Color::Rgb(100, 80, 40)),
            forge_value: Style::default().fg(amber),
            pipeline_active: Style::default().fg(amber),
            pipeline_bar: Style::default().fg(ember),
            pipeline_idle: Style::default().fg(Color::Rgb(60, 50, 30)),
            log_info: Style::default().fg(Color::Rgb(100, 80, 40)),
            log_warn: Style::default().fg(amber),
            log_error: Style::default().fg(ember),
            text: Style::default().fg(Color::Rgb(220, 200, 160)),
            text_dim: Style::default().fg(Color::Rgb(100, 80, 40)),
            title: Style::default().fg(amber).add_modifier(Modifier::BOLD),
            version: Style::default().fg(Color::Rgb(80, 60, 30)),
            border_type,
        }
    }

    /// ainulindale — the Tolkien creation-music theme.
    /// Three Voices (Manwe=silver, Ulmo=deep-blue, Aule=gold) mapped to
    /// Three Strands (Claude, Grok, Gemini). Melkor's discord = conservation violation.
    fn ainulindale(_accent: Color, border_type: BorderType) -> Self {
        let manwe = Color::Rgb(192, 210, 230);      // silver-blue (Claude/structure)
        let ulmo = Color::Rgb(30, 100, 180);         // deep ocean blue (Grok/pulse)
        let aule = Color::Rgb(218, 165, 32);         // gold (Gemini/craft)
        let varda = Color::Rgb(220, 220, 255);       // starlight
        let melkor = Color::Rgb(180, 30, 30);        // discord red
        let shadow = Color::Rgb(40, 35, 50);         // void shadow

        Self {
            border: Style::default().fg(Color::Rgb(60, 55, 70)),
            border_focused: Style::default().fg(manwe),
            accent: Style::default().fg(manwe),
            accent_bold: Style::default().fg(varda).add_modifier(Modifier::BOLD),
            strand_ok: Style::default().fg(varda),
            strand_down: Style::default().fg(shadow),
            strand_no_key: Style::default().fg(aule),
            wave_high: Style::default().fg(varda),
            wave_mid: Style::default().fg(manwe),
            wave_low: Style::default().fg(melkor),
            conservation_ok: Style::default().fg(varda),
            conservation_fail: Style::default().fg(melkor),
            gate_kenl: Style::default().fg(aule),       // craft begins
            gate_awi: Style::default().fg(ulmo),        // awareness deepens
            gate_atom: Style::default().fg(manwe),      // structure emerges
            gate_saif: Style::default().fg(varda).add_modifier(Modifier::BOLD), // Ea!
            temp_cool: Style::default().fg(ulmo),
            temp_warm: Style::default().fg(aule),
            temp_hot: Style::default().fg(melkor),
            forge_label: Style::default().fg(Color::Rgb(80, 75, 90)),
            forge_value: Style::default().fg(manwe),
            pipeline_active: Style::default().fg(aule),
            pipeline_bar: Style::default().fg(manwe),
            pipeline_idle: Style::default().fg(shadow),
            log_info: Style::default().fg(Color::Rgb(90, 85, 100)),
            log_warn: Style::default().fg(aule),
            log_error: Style::default().fg(melkor),
            text: Style::default().fg(varda),
            text_dim: Style::default().fg(Color::Rgb(90, 85, 100)),
            title: Style::default().fg(varda).add_modifier(Modifier::BOLD),
            version: Style::default().fg(Color::Rgb(60, 55, 70)),
            border_type,
        }
    }
}

/// Parse a color name or hex value.
fn parse_color(s: &str) -> Color {
    match s.to_lowercase().as_str() {
        "cyan" => Color::Cyan,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "red" => Color::Red,
        "magenta" => Color::Magenta,
        "blue" => Color::Blue,
        "white" => Color::White,
        "gray" | "grey" => Color::Gray,
        s if s.starts_with('#') && s.len() == 7 => {
            let r = u8::from_str_radix(&s[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[5..7], 16).unwrap_or(0);
            Color::Rgb(r, g, b)
        }
        _ => Color::Cyan,
    }
}

/// Parse border type from config string.
fn parse_border(s: &str) -> BorderType {
    match s.to_lowercase().as_str() {
        "double" => BorderType::Double,
        "thick" => BorderType::Thick,
        "plain" => BorderType::Plain,
        _ => BorderType::Rounded,
    }
}

/// List available theme names.
pub fn available_themes() -> Vec<&'static str> {
    vec!["coherence-dark", "neon-lattice", "minimal", "forge-amber", "ainulindale"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_presets_build() {
        for name in available_themes() {
            let config = ThemeConfig {
                name: name.to_string(),
                accent: "cyan".to_string(),
                border: "rounded".to_string(),
            };
            let theme = Theme::from_config(&config);
            // Smoke test — all fields should be accessible
            let _ = theme.border;
            let _ = theme.accent;
            let _ = theme.wave_high;
            let _ = theme.conservation_ok;
            let _ = theme.border_type.to_ratatui();
        }
    }

    #[test]
    fn parse_color_hex() {
        assert_eq!(parse_color("#ff8800"), Color::Rgb(255, 136, 0));
        assert_eq!(parse_color("cyan"), Color::Cyan);
        assert_eq!(parse_color("unknown"), Color::Cyan); // fallback
    }

    #[test]
    fn parse_border_variants() {
        assert_eq!(parse_border("rounded"), BorderType::Rounded);
        assert_eq!(parse_border("double"), BorderType::Double);
        assert_eq!(parse_border("thick"), BorderType::Thick);
        assert_eq!(parse_border("plain"), BorderType::Plain);
        assert_eq!(parse_border("???"), BorderType::Rounded); // fallback
    }
}
