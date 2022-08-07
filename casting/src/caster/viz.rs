use nannou::prelude::*;
use super::cast_impl;
use super::model::Model;
use super::map_io;
use super::fragment_shader::{monotone_triangles, get_triangles};

const BOUNDARIES: [(f32, f32); 4] = [(-570., -420.), (570., -420.), (570., 420.), (-570., 420.)];
const BOUNDARY_IDS: [i8; 4] = [3, 0, 0, -3];

pub fn key_pressed(_app: &App, _model: &mut Model, _key: Key) {
    match _key {
        Key::P => {println!("Current pos: ({}, {})", _model.pose.x, _model.pose.y);},
        Key::Escape => {
            (_model.wctrl.exit_func)(_app);
        },
        _ => {},
    }
}

pub fn mouse_moved(_: &App, _model: &mut Model, _pos: Point2) {
    _model.pose.x = _pos.x;
    _model.pose.y = _pos.y;
}

pub fn model(app: &App) -> Model {
    let config: map_io::Config = map_io::read_config("../config/config.json");

    let window_id = app
        .new_window()
        .event(event)
        .key_pressed(key_pressed)
        .mouse_moved(mouse_moved)
        .size(config.screen.width, config.screen.height)
        .view(view)
        .build()
        .unwrap();

    app.set_exit_on_escape(false);
    let meshes: map_io::Meshes = map_io::parse_map_file(config.map_path.as_str()).unwrap();

    let mut total_pt_num = 0;
    initialize_cuda_end(&meshes, &mut total_pt_num, false);

    Model::new(window_id, &config, meshes, total_pt_num)
}

fn initialize_cuda_end(new_pts: &map_io::Meshes, total_pt_num: &mut usize, initialized: bool) {
    let mut points: Vec<cast_impl::Vec2_cuda> = Vec::new();
    let mut next_ids: Vec<i8> = Vec::new();
    for mesh in new_pts.iter() {
        for pt in mesh.iter() {
            points.push(cast_impl::Vec2_cuda{x: pt.x, y: pt.y});
        }
        let length = mesh.len();
        let offset: i8 = (length as i8) - 1;
        let mut ids: Vec<i8> = vec![0; length];
        ids[0] = offset;
        ids[length - 1] = -offset;
        next_ids.extend(ids.into_iter());
    }
    for i in 0..4 {                                                 // add boundaries
        let (x, y) = BOUNDARIES[i];
        points.push(cast_impl::Vec2_cuda{x: x, y: y});
        next_ids.push(BOUNDARY_IDS[i]);
    }
    // TODO: pose need not to be Vec3
    *total_pt_num = points.len();
    let point_num = *total_pt_num as libc::c_int;
    unsafe {
        cast_impl::updatePointInfo(points.as_ptr(), next_ids.as_ptr(), point_num, initialized);
    }
} 

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    let pose = cast_impl::Vec3_cuda {x:_model.pose.x, y:_model.pose.y, z:_model.pose.z};
    let mut raw_pts: Vec<cast_impl::Vec2_cuda> = vec![cast_impl::Vec2_cuda{x: 0., y:0.}; _model.caster.total_pt_num << 1];
    let mut valid_pnum = 0;
    unsafe {
        cast_impl::shadowCasting(&pose, raw_pts.as_mut_ptr(), &mut valid_pnum);
    }
    _model.caster.viz_pts.clear();
    for i in 0..(valid_pnum as usize) {
        let pt = &raw_pts[i];
        _model.caster.viz_pts.push(pt2(pt.x, pt.y));
    }
}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let (bg_r, bg_g, bg_b) = model.color.bg_color;
    draw.background().rgba(bg_r, bg_g, bg_b, 1.0);
    
    
    let viz_pts = &model.caster.viz_pts;
    // let (shade_r, shade_g, shade_b, shade_a) = model.color.shade_color;
    let raw_tris = get_triangles(viz_pts);
    let tris = monotone_triangles(raw_tris, pt2(model.pose.x, model.pose.y));
    draw.mesh().tris_colored(tris);
    // draw.polygon()
    //     .rgba(shade_r, shade_g, shade_b, shade_a)
    //     .points((0..viz_pts.len()).map(|i| {viz_pts[i]}));

    let (r, g, b, a) = model.color.shape_color;
    for mesh in model.map_points.iter() {
        let points = (0..mesh.len()).map(|i| {
            mesh[i]
        });
        draw.polygon()
            .rgba(r, g, b, a)
            .points(points);
    }
    
    draw.ellipse()
        .w(15.)
        .h(15.)
        .x(model.pose.x)
        .y(model.pose.y)
        .color(STEELBLUE);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
