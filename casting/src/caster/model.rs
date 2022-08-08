use nannou::prelude::*;
use std::path::PathBuf;
use super::cast_impl;
use nannou_egui::Egui;

use super::map_io;
use super::color::{EditorColor};

pub struct WindowCtrl {
    pub window_id: WindowId,
    pub win_w: f32,
    pub win_h: f32,
    pub gui_visible: bool,
    pub exit_func: fn(app: &App)
}

impl WindowCtrl {
    pub fn new(win_id: WindowId, win_w: f32, win_h: f32, exit_f: fn(app: &App)) -> WindowCtrl {
        WindowCtrl {window_id: win_id, win_w: win_w, win_h: win_h, gui_visible: true, exit_func: exit_f}
    }
}

pub struct CastCtrl {
    pub viz_pts: Vec<Point2>,
    pub total_pt_num: usize, 
    pub radius: f32,
}

impl CastCtrl {
    pub fn new(_total_pt_num: usize) -> Self {
        CastCtrl {viz_pts: Vec::new(), total_pt_num: _total_pt_num, radius: 1000.}
    }
}

pub struct Model {
    pub map_points: Vec<Vec<Point2>>,
    pub caster: CastCtrl,
    pub wctrl: WindowCtrl,
    pub pose: Point3,
    pub color: EditorColor,
    pub texture: wgpu::Texture,
    pub egui: Egui,
}

impl Model {
    pub fn new(app: &App, window_id: WindowId, config: &map_io::Config, meshes: map_io::Meshes, pt_num: usize) -> Model {
        let img_path = PathBuf::from("../maps/texture-1.png");
        let texture = wgpu::Texture::from_path(app, img_path).unwrap();
        Model {
            map_points: meshes, 
            caster: CastCtrl::new(pt_num),
            wctrl: WindowCtrl::new(window_id, config.screen.width as f32, config.screen.height as f32, exit),
            pose: pt3(0., 0., 0.),
            color: EditorColor::new(),
            texture: texture,
            egui: Egui::from_window(&app.window(window_id).unwrap())
        }
    }
}

fn exit(app: &App) {
    unsafe {
        cast_impl::deallocatePoints();
    }
    app.quit();
}