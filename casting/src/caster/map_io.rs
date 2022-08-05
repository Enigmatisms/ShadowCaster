/**
 * Author： Enigmatisms
 * Copied from Enigmatisms' LSMv2 (github)
 */

use std::fs;
use nannou::prelude::*;
use std::io::{prelude::*, BufReader};
use serde_derive::{Deserialize, Serialize};
use super::cast_impl::{Vec2_cuda, self};

pub type Mesh = Vec<Point2>;
pub type Meshes = Vec<Mesh>;

#[derive(Deserialize, Serialize, Clone)]
pub struct ScreenConfig {
    pub width: u32,
    pub height: u32
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub screen: ScreenConfig,
    pub map_path: String,
}

pub fn parse_map_file<T>(filepath: T) -> Option<Meshes> where T: AsRef<std::path::Path> {
    if let Some(all_lines) = read_lines(filepath) {
        let mut result: Meshes = Vec::new();
        for line in all_lines.iter() {
            let str_vec: Vec<&str> = line.split(" ").collect();
            let point_num =  str_vec[0].parse::<usize>().unwrap() + 1;
            let mut mesh: Mesh = Vec::new();
            for i in 1..point_num {
                let str1 = str_vec[(i << 1) - 1];
                let str2 = str_vec[i << 1];
                if str1.is_empty() == true {
                    break;
                } else {
                    mesh.push(pt2(
                        str1.parse::<f32>().unwrap() - 600.,
                        str2.parse::<f32>().unwrap() - 450.
                    ));
                }
            }
            result.push(mesh);
        }
        return Some(result);
    } else {
        return None;
    }
}

pub fn meshes_to_segments(meshes: &Meshes, segments: &mut Vec<cast_impl::Vec2_cuda>) -> usize {
    let mut ptr: usize = 0;
    for mesh in meshes.iter() {
        let first = &mesh[0];
        segments.push(Vec2_cuda {x: first.x, y: first.y});
        for i in 1..(mesh.len()) {
            let current = &mesh[i];
            segments.push(Vec2_cuda {x: current.x, y: current.y});
            segments.push(Vec2_cuda {x: current.x, y: current.y});
        }
        segments.push(Vec2_cuda {x: first.x, y: first.y});
        ptr += mesh.len();
    }
    ptr
}

pub fn read_config<T>(file_path: T) -> Config where T: AsRef<std::path::Path> {
    let file: fs::File = fs::File::open(file_path).ok().unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).ok().unwrap()
}

// ========== privates ==========
fn read_lines<T>(filepath: T) -> Option<Vec<String>> where T: AsRef<std::path::Path> {
    if let Ok(file) = fs::File::open(filepath) {
        let reader = BufReader::new(file);
        let mut result_vec: Vec<String> = Vec::new();
        for line in reader.lines() {
            if let Ok(line_inner) = line {
                result_vec.push(line_inner);
            } else {
                return None;
            }
        }
        return Some(result_vec);
    }
    println!("Unable to open file.");
    return None;
}
