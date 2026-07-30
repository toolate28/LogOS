//! TQEC Braid Cascade — minimal egui prototype (reson8Labs visual language).
//!
//! Run: `cargo run -p cutile --example tqec_braid_viz --features viz`

use cutile::viz::{palette::tqec, BraidLayout, SyndromeField};
use cutile::{
    betti_tomczak_lift_check, srac_correct_if_needed, Backend, CubicalHIT, DefaultSurgeDetector,
    EntropyParams, SurgeDetector, TriWeavonHIT,
};
use eframe::egui::{self, Color32, Pos2, Stroke, Vec2};

fn rgb(c: cutile::viz::Rgb) -> Color32 {
    Color32::from_rgb(c.0, c.1, c.2)
}

struct TqecBraidViz {
    hit: TriWeavonHIT,
    backend: Backend,
    entropy_field: Vec<f32>,
    last_w: f32,
    last_surge: f32,
    last_betti: f32,
    filtration_depth: usize,
    srac_message: Option<String>,
    syndrome: SyndromeField,
    step: u32,
}

impl TqecBraidViz {
    fn new() -> Self {
        let mut hit = TriWeavonHIT::new();
        let p1 = hit.add_point();
        let p2 = hit.add_point();
        let p3 = hit.add_point();
        hit.weave(p1, p2);
        hit.weave(p2, p3);
        let _ = hit.hcomp_edge(p1, p3, 0.5);

        let backend = Backend::auto();
        let mut app = Self {
            hit,
            backend,
            entropy_field: vec![0.0; 128],
            last_w: 0.0,
            last_surge: 0.0,
            last_betti: 0.0,
            filtration_depth: 2,
            srac_message: None,
            syndrome: SyndromeField::from_entropy_result(vec![0.0; 256], 0.0, 0.0, 16, 16),
            step: 0,
        };
        app.refresh_entropy();
        app
    }

    fn refresh_entropy(&mut self) {
        let n = 64usize;
        let omega: Vec<f32> = (0..n)
            .map(|i| 0.1 + 0.02 * (i as f32 + self.step as f32).sin())
            .collect();
        let grad: Vec<f32> = (0..16)
            .map(|i| {
                if (i + self.step as usize) % 5 == 0 {
                    0.25
                } else {
                    0.01
                }
            })
            .collect();
        let rho = vec![0.05f32; n];
        let strain = vec![0.2f32; 16];

        let params = EntropyParams {
            omega_tilde: omega.clone(),
            d_perp_rho_sq: grad.clone(),
            rho,
            strain_norms: strain,
            tau: 1.0,
            nu: 0.01,
            surge_threshold: 0.05,
            prev_w_avg: self.last_w,
        };

        if let Ok(result) = self.backend.compute_entropy_v2(&params) {
            self.last_w = result.w;
            self.last_surge = result.surge;
            self.last_betti = result.betti_proxy;
            self.entropy_field = omega
                .iter()
                .enumerate()
                .map(|(i, &o)| {
                    let g = grad.get(i % grad.len()).copied().unwrap_or(0.0);
                    (o + g).min(1.0)
                })
                .collect();
            self.syndrome = SyndromeField::from_entropy_result(
                self.entropy_field.clone(),
                result.surge,
                result.betti_proxy,
                16,
                16,
            );
        }
    }

    fn step_srac(&mut self) {
        self.step = self.step.wrapping_add(1);
        self.refresh_entropy();

        let detector = DefaultSurgeDetector;
        let surge = detector.detect_surge(self.last_w, self.last_w * 0.9, 0.05);
        let lift_ok = betti_tomczak_lift_check(
            f64::from(self.last_betti),
            4.0,
            self.last_surge < 0.5,
        );

        if let Some(correction) =
            srac_correct_if_needed(surge, lift_ok, self.filtration_depth)
        {
            self.filtration_depth = correction.suggested_depth;
            self.srac_message = Some(correction.reason);
            let p = self.hit.add_point();
            let q = self.hit.add_point();
            let _ = self.hit.weave(p, q);
        } else {
            self.srac_message = Some("Coherence stable — no SRAC correction".into());
        }
    }

    fn draw_braids(&self, ui: &mut egui::Ui) {
        let rect = ui.available_rect_before_wrap();
        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 4.0, rgb(tqec::VOID));

        let layout = BraidLayout::from_hit(
            &self.hit,
            rect.width(),
            rect.height(),
            &self.entropy_field,
        );

        let pos_map: std::collections::HashMap<u64, Pos2> = layout
            .node_positions
            .iter()
            .map(|(id, x, y)| (*id, Pos2::new(rect.left() + x, rect.top() + y)))
            .collect();

        for (a, b) in &layout.edges {
            if let (Some(&pa), Some(&pb)) = (pos_map.get(a), pos_map.get(b)) {
                painter.line_segment(
                    [pa, pb],
                    Stroke::new(2.0, rgb(tqec::SRAC_BRAID)),
                );
            }
        }

        for (i, (id, _, _)) in layout.node_positions.iter().enumerate() {
            if let Some(&pos) = pos_map.get(id) {
                let color = layout
                    .node_colors
                    .get(i)
                    .copied()
                    .unwrap_or(tqec::CORE_CYAN);
                painter.circle_filled(pos, 5.0, rgb(color));
            }
        }

        // Syndrome mini-map (prototype 2 overlay, CPU path)
        let map_size = Vec2::new(96.0, 96.0);
        let map_rect = egui::Rect::from_min_size(
            Pos2::new(rect.right() - map_size.x - 8.0, rect.top() + 8.0),
            map_size,
        );
        let pixels = cutile::viz::render_syndrome_field_cpu(&self.syndrome);
        let pw = self.syndrome.width.max(1);
        let ph = self.syndrome.height.max(1);
        let cell_w = map_size.x / pw as f32;
        let cell_h = map_size.y / ph as f32;
        for y in 0..ph {
            for x in 0..pw {
                let c = pixels.get(y * pw + x).copied().unwrap_or(tqec::VOID);
                let r = egui::Rect::from_min_size(
                    Pos2::new(
                        map_rect.left() + x as f32 * cell_w,
                        map_rect.top() + y as f32 * cell_h,
                    ),
                    Vec2::new(cell_w, cell_h),
                );
                painter.rect_filled(r, 0.0, rgb(c));
            }
        }
        painter.rect_stroke(map_rect, 2.0, Stroke::new(1.0, rgb(tqec::MIRROR_OBSERVER)));
    }
}

impl eframe::App for TqecBraidViz {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("TQEC Braid Cascade — cutile v0.3");
            ui.label(format!(
                "backend={}  W={:.4}  surge={:.1}  betti={:.1}  depth={}",
                self.backend.name(),
                self.last_w,
                self.last_surge,
                self.last_betti,
                self.filtration_depth
            ));
            if let Some(msg) = &self.srac_message {
                ui.colored_label(rgb(tqec::CORE_GOLD), msg);
            }
            self.draw_braids(ui);
            ui.separator();
            if ui.button("Step SRAC Correction").clicked() {
                self.step_srac();
            }
            if ui.button("Refresh Entropy").clicked() {
                self.refresh_entropy();
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 640.0])
            .with_title("TQEC Braid Cascade"),
        ..Default::default()
    };
    eframe::run_native(
        "tqec_braid_viz",
        options,
        Box::new(|_cc| Ok(Box::new(TqecBraidViz::new()))),
    )
}