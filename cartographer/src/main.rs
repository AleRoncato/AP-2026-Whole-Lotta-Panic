use eframe::egui;

const RADIUS: f32 = 40.0;
const WATER_FILL_COLOR: egui::Color32 = egui::Color32::from_rgb(24, 70, 103);
const WATER_STROKE_COLOR: egui::Color32 = egui::Color32::from_rgb(86, 142, 173);
const STROKE_WIDTH: f32 = 2.0;

fn main() -> eframe::Result {
    eframe::run_ui_native("Cartographer", eframe::NativeOptions::default(), |ui, _frame| {
        egui::CentralPanel::default().show(ui, |ui| {
            let (area, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let center = area.rect.center();

            draw_water_tile(&painter, center);

            let tile_width = 3.0_f32.sqrt() * RADIUS;
            let right_center = center + egui::vec2(tile_width, 0.0);
            draw_water_tile(&painter, right_center);
        });
    })
}

fn draw_water_tile(painter: &egui::Painter, center: egui::Pos2) {
    // Calculate the hexagon's 6 corners' position.
    // Start half a step into the rotation to get a pointy-top hexagon.
    // TAU == 2 * PI, thus dividing it into 6 yields 6 corners.
    let corners = [0.5_f32, 1.5, 2.5, 3.5, 4.5, 5.5].map(|step| {
        let angle = (std::f32::consts::TAU / 6.0) * step;
        center + egui::vec2(RADIUS * angle.cos(), RADIUS * angle.sin())
    });

    painter.add(egui::Shape::convex_polygon(
        corners.to_vec(),
        WATER_FILL_COLOR,
        egui::Stroke::new(STROKE_WIDTH, WATER_STROKE_COLOR),
    ));
}
