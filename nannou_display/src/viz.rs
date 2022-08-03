use nannou::prelude::*;
use array2d::Array2D;

use super::cuda_helper;
use super::model::Model;
use super::map_io;

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
        .raw_event(raw_window_event)
        .mouse_moved(mouse_moved)
        .size(config.screen.width, config.screen.height)
        .view(view)
        .build()
        .unwrap();

    app.set_exit_on_escape(false);
    let meshes: map_io::Meshes = map_io::parse_map_file(config.map_path.as_str()).unwrap();

    let lidar_param = cuda_helper::Vec3_cuda{x: config.lidar.amin, y: config.lidar.amax, z:config.lidar.ainc};
    let ray_num = map_io::get_ray_num(&lidar_param);
    let mut total_pt_num = 0;
    initialize_cuda_end(&meshes, &mut total_pt_num, false);

    Model::new(app, window_id, &config, meshes, total_pt_num)
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}

pub fn initialize_cuda_end(new_pts: &map_io::Meshes, total_pt_num: &mut usize, initialized: bool) {
    let mut points: Vec<cuda_helper::Vec2_cuda> = Vec::new();
    let mut next_ids: Vec<char> = Vec::new();
    for mesh in new_pts.iter() {
        for pt in mesh.iter() {
            points.push(cuda_helper::Vec2_cuda{x: pt.x, y: pt.y});
        }
        let length = points.len();
        let offset: char = (length as char) - 1;
        let mut ids: Vec<char> = vec![0; length];
        ids[0] = offset;
        ids[length - 1] = -offset;
        next_ids.extend(ids.into_iter());
    }
    // TODO: pose need not to be Vec3
    *total_pt_num = points.len();
    let point_num = *total_pt_num as libc::c_int;
    unsafe {
        cuda_helper::updatePointInfo(seg_point_arr.as_ptr(), next_ids.as_ptr(), point_num, initialized);
    }
} 

pub fn update(_app: &App, _model: &mut Model, _update: Update) {
    let pose = cuda_helper::Vec3_cuda {x:_model.pose.x, y:_model.pose.y, z:_model.pose.z};
    let mut raw_pts: Vec<cuda_helper::Vec2_cuda> = vec![cuda_helper::Vec2_cuda{x: 0., y:0.}; _model.caster.total_pt_num << 1];
    let mut valid_pnum = 0;
    unsafe {
        cuda_helper::shadowCasting(&pose, raw_pts.as_mut_ptr(), &mut valid_pnum);
    }
    _model.caster.viz_pts.clear();
    for i in 0..(valid_pnum as usize) {
        let pt = &raw_pts[i];
        _model.caster.viz_pts.push(pt2(pt.x, pt.y));
    }
}

fn event(_app: &App, _model: &mut Model, _event: WindowEvent) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = plot::window_transform(app.draw(), &model.wtrans);

    let (bg_r, bg_g, bg_b) = model.color.bg_color;
    draw.background().rgba(bg_r, bg_g, bg_b, 1.0);
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

    let viz_pts = &model.caster.viz_pts;
    draw.polygon()
        .rgba(0.8, 0.8, 0.8, 0.8)
        .points(0..viz_pts.len().map(|i| {viz_pts[i]}));

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

/// TODO: possible debug function: when drawer renders false results, draw single line (line by line)
fn visualize_rays(
    draw: &Draw, ranges: &Vec<libc::c_float>, pose: &Point3, 
    lidar_param: &cuda_helper::Vec3_cuda, color: &[f32; 4], ray_num: usize) {
    // let cur_angle_min = pose.z + lidar_param.x + lidar_param.z;
    // for i in 0..ray_num {
    //     let r = ranges[i];
    //     // if r > 1e5 {continue;}
    //     let cur_angle = cur_angle_min + lidar_param.z * 3. * (i as f32);
    //     let dir = pt2( cur_angle.cos(), cur_angle.sin());
    //     let start_p = pt2(pose.x, pose.y);
    //     let end_p = start_p + dir * r;
    //     draw.line()
    //         .start(start_p)
    //         .end(end_p)
    //         .weight(1.)
    //         .rgba(color[0], color[1], color[2], color[3]);
    // }
}
