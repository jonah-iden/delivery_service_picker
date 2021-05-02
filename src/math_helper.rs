extern crate vecmath;
use vecmath::*;


pub fn angle(v1: [f64; 2], v2: [f64; 2]) -> f64 {
    return vec2_cross(v1, v2).atan2(vec2_dot(v1, v2))
}

pub fn len_points(p1: [f64; 2], p2: [f64; 2]) -> f64 {
    return ((p1[0] - p2[0]).powf(2.0) + (p1[1] - p2[1]).powf(2.0)).sqrt()
}

pub fn len_vec(v: [f64; 2]) -> f64 {
    return (v[0].powf(2.0) + v[0].powf(2.0)).sqrt()
}

pub fn vec(p1: [f64; 2], p2: [f64; 2]) -> [f64; 2] {
    return [p1[0] -p2[0], p1[1] -p2[1]]
}