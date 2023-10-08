#![allow(unused)]
use macroquad::experimental::collections::storage;


pub const SCREEN_W: f32 = 900.0;
pub const SCREEN_H: f32 = 700.0;
pub const WORLD_W: f32 = 900.0;
pub const WORLD_H: f32 = 700.0;

pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_W / SCREEN_H;
pub const GRAV: f32 = -2500.0;


pub fn init_global_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return *storage::get::<Settings>();
}

pub fn mod_settings() -> Settings {
    return *storage::get_mut::<Settings>();
}

pub fn init_global_signals(signals: Signals) {
    storage::store(signals);
}

pub fn get_signals() -> Signals {
    return *storage::get::<Signals>();
}

pub fn mod_signals() -> Signals {
    return *storage::get_mut::<Signals>();
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
    pub force: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            world_w: WORLD_W as i32,
            world_h: WORLD_H as i32,
            quant_init_num: 2,
            quant_min_num: 2,
            quant_rotate: 2.0,
            quant_speed: 100.0,
            quant_size_min: 12,
            quant_size_max: 12,
            force: 500.0,
       }
    }
}

#[derive(Clone, Copy)]
pub struct Signals {
    pub add_single_quant: bool,
    pub add_some_quants: bool,
    pub add_hex_quant: bool,
    pub add_state_matter: bool,
    pub reset_all: bool,
}

impl Default for Signals {
    fn default() -> Self {
        Self {
            add_single_quant: false,
            add_some_quants: false,
            add_hex_quant: false,
            add_state_matter: false,
            reset_all: false,
        }
    }
}