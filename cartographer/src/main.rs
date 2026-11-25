use eframe::egui;
use geometry::{Cell, DefaultMap, Map, Terrain};
use shared::IslandId;

const HEX_RADIUS: f32 = 40.0;
const SHIP_RADIUS: f32 = 20.0;

const MAP_RADIUS: u32 = 3;

const WATER_FILL_COLOR: egui::Color32 = egui::Color32::from_rgb(23, 66, 94);
const WATER_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(68, 117, 145);

const ROCK_FILL_COLOR: egui::Color32 = egui::Color32::from_rgb(89, 97, 108);
const ROCK_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(159, 170, 182);

const CLIFF_FILL_COLOR: egui::Color32 = egui::Color32::from_rgb(100, 72, 52);
const CLIFF_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(171, 132, 94);

const ISLAND_FILL_COLOR: egui::Color32 = egui::Color32::from_rgb(193, 165, 109);
const ISLAND_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(229, 207, 157);

const SHIPS_COLORS: [(egui::Color32, egui::Color32); 6] = [
    (
        egui::Color32::from_rgb(255, 100, 100),
        egui::Color32::from_rgb(111, 25, 36),
    ), // Red
    (
        egui::Color32::from_rgb(255, 165, 70),
        egui::Color32::from_rgb(125, 52, 14),
    ), // Orange
    (
        egui::Color32::from_rgb(255, 225, 80),
        egui::Color32::from_rgb(112, 82, 12),
    ), // Yellow
    (
        egui::Color32::from_rgb(100, 230, 145),
        egui::Color32::from_rgb(19, 91, 54),
    ), // Green
    (
        egui::Color32::from_rgb(90, 200, 255),
        egui::Color32::from_rgb(17, 66, 114),
    ), // Blue
    (
        egui::Color32::from_rgb(205, 135, 255),
        egui::Color32::from_rgb(76, 34, 119),
    ), // Purple
];

const STROKE_WIDTH: f32 = 2.0;

fn main() -> eframe::Result {
    let origin = Cell { q: 0, r: 0 };
    let cliff = Cell { q: 1, r: 0 };
    let island = Cell { q: -1, r: 0 };

    let map = DefaultMap::try_new(
        MAP_RADIUS,
        origin.disc(MAP_RADIUS).map(|cell| {
            if cell == origin {
                (cell, Terrain::Rock)
            } else if cell == cliff {
                (cell, Terrain::Cliff)
            } else if cell == island {
                (cell, Terrain::Island(IslandId(0)))
            } else {
                (cell, Terrain::Clear)
            }
        }),
    )
    .map_err(|error| eframe::Error::AppCreation(Box::new(error)))?;

    eframe::run_ui_native("Cartographer", eframe::NativeOptions::default(), move |ui, _frame| {
        egui::CentralPanel::default().show(ui, |ui| {
            let (area, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let center = area.rect.center();

            for cell in origin.disc(MAP_RADIUS) {
                if let Some(terrain) = map.terrain(cell) {
                    draw_tile(&painter, center + hex_offset(cell), terrain);
                }
            }

            draw_corner_ships(&painter, center, &map);
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
        Terrain::Rock => (ROCK_FILL_COLOR, ROCK_STROKE_COLOR),
        Terrain::Cliff => (CLIFF_FILL_COLOR, CLIFF_STROKE_COLOR),
        Terrain::Island(_) => (ISLAND_FILL_COLOR, ISLAND_STROKE_COLOR),
    };

    painter.add(egui::Shape::convex_polygon(
        corners.to_vec(),
        fill_color,
        egui::Stroke::new(STROKE_WIDTH, stroke_color),
    ));
}

fn draw_corner_ships(painter: &egui::Painter, center: egui::Pos2, map: &DefaultMap) {
    let half_base = 3.0_f32.sqrt() * SHIP_RADIUS / 2.0;

    for (cell, (fill, stroke_color)) in map.corners().into_iter().zip(SHIPS_COLORS) {
        let ship_center = center + hex_offset(cell);
        let corners = vec![
            ship_center + egui::vec2(0.0, -SHIP_RADIUS),
            ship_center + egui::vec2(half_base, SHIP_RADIUS / 2.0),
            ship_center + egui::vec2(-half_base, SHIP_RADIUS / 2.0),
        ];

        let stroke = egui::Stroke::new(STROKE_WIDTH, stroke_color);
        painter.add(egui::Shape::convex_polygon(corners, fill, stroke));
    }
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
