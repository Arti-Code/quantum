#![allow(unused)]
use macroquad::experimental::collections::storage;


pub const SCREEN_W: f32 = 1400.0;
pub const SCREEN_H: f32 = 950.0;
pub const WORLD_W: f32 = 1400.0;
pub const WORLD_H: f32 = 950.0;

pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_W / SCREEN_H;
pub const GRAV: f32 = 50.0;


pub fn init_global_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return *storage::get::<Settings>();
}

pub fn mod_settings() -> Settings {
    return *storage::get_mut::<Settings>();
}


#[derive(Clone, Copy)]
pub struct Settings {
    pub world_w: i32,
    pub world_h: i32,
    pub quant_min_num: usize,
    pub quant_init_num: usize,
    pub quant_speed: f32,
    pub quant_rotate: f32,
    pub quant_size_min: i32,
    pub quant_size_max: i32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            world_w: WORLD_W as i32,
            world_h: WORLD_H as i32,
            quant_init_num: 1024,
            quant_min_num: 512,
            quant_rotate: 2.0,
            quant_speed: 100.0,
            quant_size_min: 3,
            quant_size_max: 3,
       }
    }
}