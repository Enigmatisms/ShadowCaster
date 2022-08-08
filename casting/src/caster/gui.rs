use nannou::prelude::*;
use nannou_egui::{self, egui};

use super::model::Model;
use super::map_io::load_map_file;
use super::viz::initialize_cuda_end;

pub fn update_gui(model: &mut Model, update: &Update) {
    let Model {
        ref mut map_points,
        ref mut caster,
        ref mut egui,
        ..
    } = model;
    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();
    let window = egui::Window::new("Control Panel").default_width(200.);
    window.show(&ctx, |ui| {
        egui::Grid::new("slide_bars")
            .striped(true)
        .show(ui, |ui| {
            ui.label("Radius");
            ui.add(egui::Slider::new(&mut caster.radius, (500.)..=10000.));
            ui.end_row();
        });
        ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
            if ui.button("Load map file").clicked() {
                let mut raw_points: Vec<Vec<Point2>> = Vec::new();
                let _ = load_map_file(&mut raw_points);
                let mut total_pt_num = 0;
                initialize_cuda_end(&raw_points, &mut total_pt_num, true);
                caster.total_pt_num = total_pt_num;
                *map_points = raw_points;           // replacing map points
            }
        });
    });
}
