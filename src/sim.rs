#![allow(unused)]

use crate::camera::*;
use crate::joint::MyJoint;
use crate::quant::Quant;
use crate::util::*;
use crate::physics::*;
use crate::collector::*;
use crate::globals::*;
use crate::ui::*;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use macroquad::experimental::collections::storage;
use rapier2d::na::Point2;
use rapier2d::na::Vector2;
use rapier2d::prelude::ColliderSet;
use rapier2d::prelude::FixedJointBuilder;
use rapier2d::prelude::GenericJoint;
use rapier2d::prelude::ImpulseJoint;
use rapier2d::prelude::ImpulseJointHandle;
use rapier2d::prelude::ImpulseJointSet;
use rapier2d::prelude::JointAxis;
use rapier2d::prelude::PrismaticJointBuilder;
use rapier2d::prelude::RigidBodyHandle;
use rapier2d::prelude::RigidBodySet;
use rapier2d::prelude::UnitVector;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::process::exit;



pub struct Simulation {
    pub world_size: Vec2,
    pub font: Font,
    pub physics: Physics,
    pub camera: Camera2D,
    pub running: bool,
    pub sim_time: f64,
    pub mouse_state: MouseState,
    pub quants: QuantumCollector,
    ui: UI,
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
            ui: UI::new(),
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
        //for _ in 0..24 {
        //    self.add_triplet(); 
        //}  
    }

    pub fn autorun_new_sim(&mut self) {
    }

    fn update_quants(&mut self) {
        for (_, quant) in self.quants.get_iter_mut() {
            quant.update(&mut self.physics);
        }
    }

    pub fn update(&mut self) {
        self.ui.process();
        self.process_signals();
        self.update_sim_state();
        self.check_quants_num();
        self.update_quants();
        //self.update_motors();
        self.physics.step_physics();
    }

    fn update_motors(&mut self) {
        for (handle, joint) in self.physics.impulse_joint_set.iter_mut() {
            if rand::gen_range(0, 100) == 100 {
                joint.data.set_motor_velocity(JointAxis::AngX, random_unit()*10000.0, 0.2);
            }
        }
    }

    pub fn draw(&self) {
        //set_default_camera();
        set_camera(&self.camera);
        clear_background(BLACK);
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_grid(50);
        self.draw_joints();
        self.draw_quants();
        self.ui.draw();
    }

    fn draw_joints(&self) {
        for (handle, joint) in self.physics.impulse_joint_set.iter() {
            let rbh1 = joint.body1;
            let rbh2 = joint.body2;
            let rb1 = self.physics.rigid_bodies.get(rbh1).unwrap() ;
            let rb2 = self.physics.rigid_bodies.get(rbh2).unwrap() ;
            let (p1, r1) = iso_to_vec2_rot(rb1.position());
            let (p2, r2) = iso_to_vec2_rot(rb2.position());
            draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, BLUE);
        }
    }

    fn process_signals(&mut self) {
        let mut signals = mod_signals();
        if signals.add_single_quant {
            signals.add_single_quant = false;
            self.add_triplet();
        }
        if signals.add_some_quants {
            signals.add_some_quants = false;
            for _ in 0..12 {
                //self.add_triplet();
                //self.add_hex();
                self.add_custom(rand::gen_range(1, 6) as usize);
            }
        }
        if signals.add_hex_quant {
            signals.add_hex_quant = false;
            self.add_hex();
        }
        if signals.reset_all {
            signals.reset_all = false;
            self.reset();
        }
        init_global_signals(signals);
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
        control_camera(&mut self.camera);
        if is_key_pressed(KeyCode::Escape) {
            exit(0);
        }
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

    fn add_triplet(&mut self) {
        let mut vc = Vec2::new(400.0, 300.0);
        vc = random_position(100.0, 100.0) + Vec2::new(WORLD_W/2.0-50.0, WORLD_H/2.0-50.0);
        let q = Quant::new_custom(vc, 7.0, 3, BLUE, &mut self.physics);
        let step = 2.0 * PI / 3.0;
        let prev_quant: Option<RigidBodyHandle> = None;
        //for m in 0..5 {
            let major_quant = self.quants.add_quant(q);

            for i in 0..3 {
                let a = i as f32 * step;
                let vd = Vec2::from_angle(a)*14.0;
                let vq = vc + vd;
                let q = Quant::new_custom(vq, 7.0, 3, GREEN, &mut self.physics);
                let minor_quant = self.quants.add_quant(q);
                let vr = (vq-vc)/2.0;
                let bound = PrismaticJointBuilder::new(UnitVector::new_normalize(Vector2::new(vr.x, vr.y))).local_anchor1(Point2::new(vr.x, vr.y)).local_anchor2(Point2::new(-vr.x, -vr.y))
                .limits([0.0, 1.0]).build();
            let bound_handle = self.physics.impulse_joint_set.insert(major_quant, minor_quant, bound, true);
            }
        //}
    }

    fn add_hex(&mut self) {
        let mut vc = Vec2::new(400.0, 300.0);
        vc = random_position(100.0, 100.0) + Vec2::new(WORLD_W/2.0-50.0, WORLD_H/2.0-50.0);
        let q = Quant::new_custom(vc, 7.0, 3, BLUE, &mut self.physics);
        let step = 2.0 * PI / 6.0;
        let prev_quant: Option<RigidBodyHandle> = None;
        //for m in 0..5 {
            let major_quant = self.quants.add_quant(q);

            for i in 0..6 {
                let a = i as f32 * step;
                let vd = Vec2::from_angle(a)*14.0;
                let vq = vc + vd;
                let q = Quant::new_custom(vq, 7.0, 3, GREEN, &mut self.physics);
                let minor_quant = self.quants.add_quant(q);
                let vr = (vq-vc)/2.0;
                let bound = PrismaticJointBuilder::new(UnitVector::new_normalize(Vector2::new(vr.x, vr.y))).local_anchor1(Point2::new(vr.x, vr.y)).local_anchor2(Point2::new(-vr.x, -vr.y))
                .limits([0.0, 1.0]).build();
            let bound_handle = self.physics.impulse_joint_set.insert(major_quant, minor_quant, bound, true);
            }
        //}
    }

    fn add_custom(&mut self, n: usize) {
        let mut vc = Vec2::new(400.0, 300.0);
        vc = random_position(100.0, 100.0) + Vec2::new(WORLD_W/2.0-50.0, WORLD_H/2.0-50.0);
        let q = Quant::new_custom(vc, 9.0, 3, RED, &mut self.physics);
        let step = 2.0 * PI / n as f32;
        let prev_quant: Option<RigidBodyHandle> = None;
        //for m in 0..5 {
            let major_quant = self.quants.add_quant(q);

            for i in 0..n {
                let a = i as f32 * step;
                let vd = Vec2::from_angle(a)*14.0;
                let vq = vc + vd;
                let q = Quant::new_custom(vq, 6.0, 3, GREEN, &mut self.physics);
                let minor_quant = self.quants.add_quant(q);
                let vr = (vq-vc)/2.0;
                let bound = PrismaticJointBuilder::new(UnitVector::new_normalize(Vector2::new(vr.x, vr.y))).local_anchor1(Point2::new(vr.x, vr.y)).local_anchor2(Point2::new(-vr.x, -vr.y))
                .limits([0.0, 1.0]).build();
            let bound_handle = self.physics.impulse_joint_set.insert(major_quant, minor_quant, bound, true);
            }
        //}
    }

    fn reset(&mut self) {
        self.physics.colliders = ColliderSet::new();
        self.physics.impulse_joint_set = ImpulseJointSet::new();
        self.physics.rigid_bodies = RigidBodySet::new();
        self.quants = QuantumCollector::new();
    }

}

