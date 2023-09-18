#![allow(unused)]

use crate::camera::*;
use crate::util::*;
use crate::physics::*;
use crate::collector::*;
use crate::globals::*;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use macroquad::experimental::collections::storage;
use rapier2d::prelude::RigidBodyHandle;
use std::collections::HashMap;
use std::f32::consts::PI;



pub struct Simulation {
    pub world_size: Vec2,
    pub font: Font,
    pub physics: Physics,
    pub camera: Camera2D,
    pub running: bool,
    pub sim_time: f64,
    pub mouse_state: MouseState,
    pub quants: QuantumCollector,
}

impl Simulation {
    
    pub fn new(font: Font) -> Self {
        Self {
            world_size: Vec2 {
                x: f32::NAN,
                y: f32::NAN,
            },
            font,
            physics: Physics::new(),
            camera: create_camera(),
            running: true,
            sim_time: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN },
            quants: QuantumCollector::new(),
        }
    }

    fn reset_sim(&mut self, sim_name: Option<&str>) {
        let settings = get_settings();
        self.world_size = Vec2::new(settings.world_w as f32, settings.world_h as f32);
        self.physics = Physics::new();
        self.quants.quants.clear();
        self.sim_time = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN };
        self.running = true;
        self.init();
    }

    pub fn init(&mut self) {
        let settings = get_settings();
        let quants_num = settings.quant_init_num;
        self.quants.add_many_quants(quants_num, &mut self.physics);
        println!("quants: {}", self.quants.count());
    }

    pub fn autorun_new_sim(&mut self) {
    }

    fn update_quants(&mut self) {
        for (_, quant) in self.quants.get_iter_mut() {
            quant.update(&mut self.physics);
        }
    }

    pub fn update(&mut self) {
        self.update_sim_state();
        self.check_quants_num();
        self.update_quants();
        self.physics.step_physics();
    }

    pub fn draw(&self) {
        //set_default_camera();
        set_camera(&self.camera);
        clear_background(BLACK);
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_grid(50);
        self.draw_quants();
    }

    fn draw_quants(&self) {
        for (id, quant) in self.quants.get_iter() {
            quant.draw();
        }
    }

    fn draw_grid(&self, cell_size: u32) {
        let w = self.world_size.x;
        let h = self.world_size.y;
        let col_num = (w / cell_size as f32).floor() as u32;
        let row_num = (h / cell_size as f32).floor() as u32;
        for x in 0..col_num + 1 {
            for y in 0..row_num + 1 {
                draw_circle((x * cell_size) as f32, (y * cell_size) as f32, 1.0, GRAY);
            }
        }
    }

    pub fn input(&mut self) {
        self.mouse_input();
        control_camera(&mut self.camera);
    }

    fn mouse_input(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            let (mouse_posx, mouse_posy) = mouse_position();
            let mouse_pos = Vec2::new(mouse_posx, mouse_posy);
            let rel_coords = self.camera.screen_to_world(mouse_pos);
        }
    }

    fn update_sim_state(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_state.pos = Vec2::new(mouse_x, mouse_y);
    }

    fn check_quants_num(&mut self) {
        let settings = get_settings();
        if self.quants.count() < settings.quant_min_num {
            self.quants.add_many_quants(1, &mut self.physics);
        }
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }
}

