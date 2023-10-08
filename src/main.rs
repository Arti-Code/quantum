//#![allow(unused)]

mod camera;
mod sim;
mod util;
mod physics;
mod collector;
mod globals;
mod quant;
mod joint;
mod ui;

use macroquad::prelude::*;
use crate::sim::*;
use crate::globals::*;


fn app_configuration() -> Conf {
    Conf {
        window_title: env!("CARGO_PKG_NAME").to_string().to_uppercase(),
        window_width: SCREEN_W as i32,
        window_height: SCREEN_H as i32,
        sample_count: 16,
        window_resizable: false,
        ..Default::default()
    }
}

fn setup() {
    init_global_settings(Settings::default());
    init_global_signals(Signals::default());
}

#[macroquad::main(app_configuration)]
async fn main() {
    setup();
    let font = load_ttf_font("assets/fonts/firacode.ttf").await.expect("can't load font resource!");
    let mut sim = Simulation::new(font.clone());
    sim.init();
    loop {
        sim.input();
        if sim.is_running() {
            sim.update();
            sim.draw();
        }
        next_frame().await;
    }
}
