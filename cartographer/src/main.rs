use eframe::egui;
use geometry::{Cell, Terrain};

const HEX_RADIUS: f32 = 40.0;
const MAP_RADIUS: u32 = 2;

const WATER_FILL_COLOR: egui::Color32 = egui::Color32::from_rgb(23, 66, 94);
const WATER_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(68, 117, 145);

const ROCK_FILL_COLOR: egui::Color32 = egui::Color32::from_rgb(89, 97, 108);
const ROCK_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(159, 170, 182);

const ISLAND_FILL_COLOR: egui::Color32 = egui::Color32::from_rgb(193, 165, 109);
const ISLAND_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(229, 207, 157);

const STROKE_WIDTH: f32 = 2.0;

fn main() -> eframe::Result {
    eframe::run_ui_native("Cartographer", eframe::NativeOptions::default(), |ui, _frame| {
        egui::CentralPanel::default().show(ui, |ui| {
            let (area, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let center = area.rect.center();

            let origin = Cell { q: 0, r: 0 };

            for cell in origin.disc(MAP_RADIUS) {
                let mut terrain = Terrain::Clear;
                if cell == origin {
                    terrain = Terrain::Rock;
                }
                draw_tile(&painter, center + hex_offset(cell), terrain);
            }
        });
    })
}

fn draw_tile(painter: &egui::Painter, center: egui::Pos2, terrain: Terrain) {
    // Calculate the hexagon's 6 corners' position.
    // Start half a step into the rotation to get a pointy-top hexagon.
    // TAU == 2 * PI, thus dividing it into 6 yields 6 corners.
    let corners = [0.5_f32, 1.5, 2.5, 3.5, 4.5, 5.5].map(|step| {
        let angle = (std::f32::consts::TAU / 6.0) * step;
        center + egui::vec2(HEX_RADIUS * angle.cos(), HEX_RADIUS * angle.sin())
    });

    let (fill_color, stroke_color) = match terrain {
        Terrain::Clear => (WATER_FILL_COLOR, WATER_STROKE_COLOR),
        Terrain::Rock | Terrain::Cliff => (ROCK_FILL_COLOR, ROCK_STROKE_COLOR),
        Terrain::Island(_) => (ISLAND_FILL_COLOR, ISLAND_STROKE_COLOR),
    };

    painter.add(egui::Shape::convex_polygon(
        corners.to_vec(),
        fill_color,
        egui::Stroke::new(STROKE_WIDTH, stroke_color),
    ));
}

#[allow(clippy::cast_precision_loss, reason = "Rendering uses f32 screen coordinates.")]
fn hex_offset(cell: Cell) -> egui::Vec2 {
    let q = cell.q as f32;
    let r = cell.r as f32;

    let width = 3.0_f32.sqrt() * HEX_RADIUS;
    let x = width * (q + r / 2.0);
    let y = r * (1.5 * HEX_RADIUS);

    egui::vec2(x, y)
}
