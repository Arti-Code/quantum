#![allow(unused)]

use std::f32::consts::PI;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::prelude::*;
use rapier2d::parry::query::contact; 
use rapier2d::na::{Isometry2, Vector2, Translation, Point2, Const};
use crate::globals::*;

pub fn generate_key64() -> u64 {
    return gen_range(u64::MIN, u64::MAX);
}

pub fn random_unit() -> f32 {
    return rand::gen_range(-1.0, 1.0);
}

pub fn random_position(x_max: f32, y_max: f32) -> Vec2 {
    let x = rand::gen_range(0.0, x_max);
    let y = rand::gen_range(0.0, y_max);
    return Vec2::new(x, y);
}

pub fn random_rotation() -> f32 {
    let rot = rand::gen_range(0.0, PI * 2.0);
    return rot;
}

pub fn random_unit_vec2() -> Vec2 {
    let x = rand::gen_range(-1.0, 1.0);
    let y = rand::gen_range(-1.0, 1.0);
    return Vec2::new(x, y).normalize_or_zero();
}

pub fn random_color() -> color::Color {
    let colors = vec![
        LIGHTGRAY, GRAY, DARKGRAY, YELLOW, GOLD, ORANGE, PINK, RED, 
        MAROON, GREEN, LIME, DARKGREEN, SKYBLUE, BLUE, DARKBLUE, PURPLE, 
        VIOLET, DARKPURPLE, BEIGE, BROWN, DARKBROWN, WHITE, MAGENTA
    ];
    //let colors = vec![RED, GREEN, BLUE, YELLOW, ORANGE, GRAY, SKYBLUE, LIME, ];
    let num = colors.len();
    let c = rand::gen_range(0, num);
    return colors[c];
}

pub fn random_color5() -> color::Color {
    let colors = [RED, BLUE, GREEN, YELLOW, WHITE];
    let c = rand::gen_range(0, 5);
    return colors[c];
}

pub fn angle2vec2(angle: f32) -> Vec2 {
    let (x, y) = angle.sin_cos();
    let mut v = Vec2::new(x, y).normalize_or_zero();
    return v;
}

pub fn wrap_around(v: &Vec2) -> Vec2 {
    let tolerance = 5.0;
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > WORLD_W + tolerance {
        vr.x = 0.0 - tolerance;
    } else if vr.x < 0.0 - tolerance {
        vr.x = WORLD_W + tolerance;
    }
    if vr.y > WORLD_H + tolerance {
        vr.y = 0.0 - tolerance;
    } else if vr.y < 0.0 - tolerance {
        vr.y = WORLD_H + tolerance;
    }
    return vr;
}

pub fn iso_to_vec2_rot(isometry: &Isometry<Real>) -> (Vec2, f32) {
    let pos = Vec2::new(isometry.translation.x, isometry.translation.y);
    let rot = isometry.rotation.angle() + PI;
    return (pos, rot);
}

pub fn make_isometry(posx: f32, posy: f32, rotation: f32) -> Isometry2<f32> {
    let iso = Isometry2::new(Vector2::new(posx, posy), rotation);
    return iso;
}

pub fn matrix_to_vec2(translation: Translation<f32, 2>) -> Vec2 {
    return Vec2::new(translation.x, translation.y);
}

pub fn map_polygon(n: usize, r: f32, dev: f32) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = vec![];
    let s = 2.0 * PI / (n as f32);
    let mut a = 2.0 * PI;
    for i in 0..n {
        a = s * i as f32;
        let x = a.sin();
        let y = a.cos();
        let deviation = rand::gen_range(-dev, dev);
        let radius = r + r * deviation;
        let v = Vec2::new(x, y)*radius;
        points.push(v);
    }
    return points;
}

fn vec2_to_point2(v: &Vec2) -> Point2<f32> {
    return Point2::new(v.x, v.y);
}

pub fn vec2_to_point2_collection(vec2_list: &Vec<Vec2>) -> Vec<Point2<f32>> {
    let mut points: Vec<Point2<f32>> = vec![];
    for v in vec2_list.iter() {
        let p = Point2::new(v.x, v.y);
        points.push(p);
    }
    return points;
}

pub fn vec2_to_point2_array(vec2_list: &Vec<Vec2>) -> Matrix<Point2<f32>> {
    let l = vec2_list.len();
    let mut points: Matrix<Point2<f32>>;
    let vecs = vec2_to_point2_collection(vec2_list);
    points = Matrix::from_vec(vecs);
    return points;
}

pub fn contact_mouse(mouse_pos: Vec2, target_pos: Vec2, target_rad: f32) -> bool {
    let v1 = Vec2::new(mouse_pos.x, mouse_pos.y);
    let v2 = Vec2::new(target_pos.x, target_pos.y);
    let pos1 = make_isometry(v1.x, v1.y, 0.0);
    let pos2 = make_isometry(v2.x, v2.y, 0.0);
    let ball1 = Ball::new(2.0);
    let ball2 = Ball::new(target_rad);
    match contact(&pos1, &ball1, &pos2, &ball2, 0.0).unwrap() {
        Some(_) => true,
        None => false,
    }
}

pub fn make_regular_poly(n: usize, r: f32, dev: Option<f32>) -> Vec<Vec2> {
    let s = 2.0*PI/n as f32;
    let mut verts: Vec<Vec2> = vec![];
    for i in 0..n {
        let d = match dev {
            Some(deviation) => rand::gen_range(-deviation, deviation),
            None => 0.0,
        };

        let a = s * i as f32;
        let x = a.cos();
        let y = a.sin();
        let v = Vec2::new(x, y)*r + r*d;
        verts.push(v);
    }
    return verts;
}

pub fn make_regular_poly_indices(n: usize, r: f32) -> (Vec<Vec2>, Vec<[u32; DIM]>) {
    let s = 2.0*PI/n as f32;
    let mut verts: Vec<Vec2> = vec![];
    let mut indices: Vec<[u32; DIM]> = vec![];
    for i in 0..n {
        let a = s * i as f32;
        let x = a.cos()*r;
        let y = a.sin()*r;
        let v = Vec2::new(x, y);
        if i == 0 {
            indices.push([(n-1) as u32, i as u32]);
        } else {
            indices.push([(i-1) as u32, i as u32]);
        }
        verts.push(v);
    }
    return (verts, indices);
}

pub fn create_name(num: usize) -> String {
    let names_list: Vec<&str> = vec![
        "am","af", "ax", "ar", "av", "al", "aq", "ak", "ar", "at",
        "cu", "ca", "co", "cy", "cu", "ce", "co", "cv", "ce", "cd", "cf", "cf", "ct", "ci", "cj", "ck", "cl", "cr", "cs", "cz", "cw", "cm", "cu", "cp",
        "mu", "ma", "mo", "my", "mu", "me", "mo", "mv", "me", "md", "mf", "mf", "mt", "mi", "mj", "mk", "ml", "mr", "ms", "mz", "mw", "mm", "mu", "mp",
        "ju", "ja", "jo", "jy", "ju", "je", "jo", "jv", "je", "jd", "jf", "jf", "jt", "ji", "jj", "jk", "jl", "jr", "js", "jz", "jw", "jj", "ju", "jp",
        "du", "da", "do", "dy", "du", "de", "do", "dv", "de", "dd", "df", "df", "dt", "di", "dj", "dk", "dl", "dr", "ds", "dz", "dw", "dd", "du", "dp",
        "so", "su", "sa", "si", "se", "sy", "sl", "sj", "ss", "sk", "sr", "st", "sq", "sf", "sn",
        "nu", "na", "no", "ny", "nu", "ne", "no", "nv", "ne", "nd", "nf", "nf", "nt", "ni", "nj", "nk", "nl", "nr", "ns", "nz", "nw", "nn", "nu", "np",
        "vu", "va", "vo", "vy", "vu", "ve", "vo", "vv", "ve", "vd", "vf", "vf", "vt", "vi", "vj", "vk", "vl", "vr", "vs", "vz", "vw", "vv", "vu", "vp",
        "xu", "xa", "xo", "xy", "xu", "xe", "xo", "xv", "xe", "xd", "xf", "xf", "xt", "xi", "xj", "xk", "xl", "xr", "xs", "xz", "xw", "xx", "xu", "xp",
        "pu", "pa", "po", "py", "pu", "pe", "po", "pv", "pe", "pd", "pf", "pf", "pj", "pi", "pj", "pk", "pl", "pr", "ps", "pz", "pw", "pp", "pu", "pt",
        "lu", "la", "lo", "ly", "lu", "le", "lo", "lv", "le", "ld", "lf", "lf", "lt", "li", "lj", "lk", "ll", "lr", "ls", "lz", "lw", "ll", "lu", "lp", 
        "ku", "ka", "ko", "ky", "ku", "ke", "ko", "kv", "ke", "kd", "kf", "kf", "kt", "ki", "kj", "kk", "kl", "kr", "ks", "kz", "kw", "kk", "ku", "kp",
        "ru", "ra", "ro", "ry", "ru", "re", "ro", "rv", "re", "rd", "rf", "rf", "rt", "ri", "rj", "rk", "rl", "rr", "rs", "rz", "rw", "rr", "ru", "rp",
        "fu", "fa", "fo", "fy", "fu", "fe", "fo", "fv", "fe", "fd", "ff", "ff", "ft", "fi", "fj", "fk", "fl", "fr", "fs", "fz", "fw", "ff", "fu", "fp", 
        "ol", "oi", "oj", "od", "os", "ot", "ok", "on", "om", "oc", "ox", "oz", "op",
        "iu", "ia", "io", "iy", "iu", "ie", "io", "iv", "ie", "id", "if", "if", "it", "ii", "ij", "ik", "il", "ir", "is", "iz", "iw", "ii", "iu", "ip",
        "wu", "wa", "wo", "wy", "wu", "we", "wo", "wv", "we", "wd", "wf", "wf", "wt", "wi", "wj", "wk", "wl", "wr", "ws", "wz", "ww", "ww", "wu", "wp",
        "bu", "ba", "bo", "by", "bu", "be", "bo", "bv", "be", "bd", "bf", "bf", "bt", "bi", "bj", "bk", "bl", "br", "bs", "bz", "bw", "bb", "bu", "bp",
        "qu", "qa", "qo", "qy", "qu", "qe", "qo", "qv", "qe", "qd", "qf", "qf", "qt", "qi", "qj", "qk", "ql", "qr", "qs", "qz", "qw", "qq", "qu", "qp", 
        "uo", "ui", "ua", "us", "ud", "uf", "ug", "ug", "uj", "uk", "ul",
        "hu", "ha", "ho", "hy", "hu", "he", "ho", "hv", "he", "hd", "hf", "hf", "ht", "hi", "hj", "hk", "hl", "hr", "hs", "hz", "hw", "hh", "hu", "hp", 
        "su", "sa", "so", "sy", "su", "se", "so", "sv", "se", "sd", "sf", "sf", "st", "si", "sj", "sk", "sl", "sr", "ss", "sz", "sw", "ss", "su", "sp",
        "tu", "ta", "to", "ty", "tu", "te", "to", "tv", "te", "td", "tf", "tf", "th", "ti", "tj", "tk", "tl", "tr", "ts", "tz", "tw", "tt", "tu", "tp",
        "zu", "za", "zo", "zy", "zu", "ze", "zo", "zv", "ze", "zd", "zf", "zf", "zt", "zi", "zj", "zk", "zl", "zr", "zs", "zz", "zw", "zz", "zu", "zp"
    ];
    let mut name = String::new();
    let size = names_list.len();
    for locus in 0..num {
        let i = rand::gen_range(0, size);
        let voice = names_list.get(i).unwrap();
        name.insert_str(locus*2, voice);
    }
    return name;

}


pub struct MouseState {
    pub pos: Vec2,
}
