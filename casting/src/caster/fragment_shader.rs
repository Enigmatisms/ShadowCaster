/**
 * Tessellated visible area
 * Fragment shader
 * This fragment shader is a false one (not applicable to non-convex polygon)
 * 本人设计的tessellation 算法，很可惜，此算法只适用于凸多边形的三角化
 */
use nannou::prelude::*;

pub fn get_triangles(viz_pts: &Vec<Point2>) -> Vec<geom::Tri> {
    recurvise_reduce(viz_pts, viz_pts.len())
}

pub fn monotone_triangles(tri_pts: Vec<geom::Tri>, pos: Point2) -> impl Iterator<Item = geom::Tri<(Vec3, Srgba)>> {
    (0..tri_pts.len()).map(move |i| {
        tri_pts[i].map_vertices(|v| {
            let diff = (pt2(v.x, v.y) - pos).length();
            let coeff = (1.0 - diff / 300.).max(0.);
            let color: Srgba = srgba(coeff, coeff, coeff, 1.0);
            (v, color)
        })
    })
}

#[inline(always)]
fn order_check(p1: &Point2, p2: &Point2, p3: &Point2) -> bool {
    let vec1 = *p2 - *p1;
    let vec2 = *p3 - *p2;
    (vec1.x * vec2.y - vec2.x * vec1.y) > 0.
}

#[inline(always)]
fn push_tri(result: &mut Vec<geom::Tri>, pts: &Vec<Point2>, i0: usize, i1:usize, i2: usize) {
    result.push(geom::Tri([pts[i0].extend(0.), pts[i1].extend(0.), pts[i2].extend(0.)]));
}

fn recurvise_reduce(pts: &Vec<Point2>, last_len: usize) -> Vec<geom::Tri> {
    let length = pts.len();
    let mut result: Vec<geom::Tri> = Vec::new();
    if length > 3 {
        let mut next_pts: Vec<Point2> = Vec::new();
        // process all points, produce pts for next recurvise iteration and some triangles
        let mut s_ptr = 0;
        let mut last_dir = pts[s_ptr + 1] - pts[s_ptr]; 
        let mut now_ptr: usize = 2;
        loop {
            next_pts.push(pts[s_ptr]);
            if now_ptr < length {
                while (pts[now_ptr] - pts[now_ptr - 1]).perp_dot(last_dir).abs() < 1e-6 {
                    now_ptr += 1;           // 共线点跳过
                    if now_ptr == length {
                        result.extend(recurvise_reduce(&next_pts, length).into_iter());
                        return result;
                    }
                }
                if order_check(&pts[s_ptr], &pts[now_ptr - 1], &pts[now_ptr]) {
                    push_tri(&mut result, pts, s_ptr, now_ptr - 1, now_ptr);
                    s_ptr = now_ptr;
                    now_ptr += 2;
                } else {
                    s_ptr = now_ptr - 1;
                    now_ptr += 1;
                }
                last_dir = pts[(s_ptr + 1) % length] - pts[s_ptr];
            } else if now_ptr == length {
                if order_check(&pts[s_ptr], &pts[now_ptr - 1], &pts[0]) {
                    push_tri(&mut result, pts, s_ptr, now_ptr - 1, 0);
                } else {
                    next_pts.push(pts[now_ptr - 1]);
                }
                break;
            } else {
                break;
            }
        }
        if last_len == next_pts.len() {
            next_pts.reverse();
        }
        result.extend(recurvise_reduce(&next_pts, length).into_iter());
        return result;
    } else {
        if pts.len() == 3 {
            push_tri(&mut result, pts, 0, 1, 2);
        }
        return result;
    }
}