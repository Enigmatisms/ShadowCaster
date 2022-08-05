extern crate libc;

#[repr(C)]
pub struct Vec3_cuda {
    pub x: libc::c_float,
    pub y: libc::c_float,
    pub z: libc::c_float
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec2_cuda {
    pub x: libc::c_float,
    pub y: libc::c_float
}

#[link(name = "cast_impl", kind = "static")]
extern {
    pub fn deallocatePoints();
    // const Vec2* const meshes, const char* const nexts, int point_num, bool initialized
    pub fn updatePointInfo(meshes: *const Vec2_cuda, nexts: *const libc::c_char, point_num: libc::c_int, initialized: bool);
    // void shadowCasting(const Vec3& pose, Vec2* const host_output, int& point_num) {
    pub fn shadowCasting(pose: &Vec3_cuda, host_output: *const Vec2_cuda, point_num: &libc::c_int);
}