#![allow(unused)]

use std::collections::HashMap;
use std::f32::consts::PI;
use crate::util::*;
use crate::physics::*;
use crate::globals::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::ImpulseJointHandle;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};


pub struct Quant {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub mass: f32,
    pub vel: f32,
    pub ang_vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub shape: SharedShape,
    pub physics_handle: RigidBodyHandle,
    pub bounds_num: usize,
    pub bounds: Vec<ImpulseJointHandle>,
}



impl Quant {
    
    pub fn new(size: f32, bounds_num: usize, color: Color, physics: &mut Physics) -> Self {
        let settings = get_settings();
        let key = gen_range(u64::MIN, u64::MAX);
        //let size = rand::gen_range(settings.quant_size_min, settings.quant_size_max) as f32;
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let shape = SharedShape::ball(size*0.75);
        let rbh = physics.add_dynamic(key, &pos, 0.0, shape.clone(), PhysicsProperities::default());
        //let color = random_color();
        Self {
            key: generate_key64(),
            pos,
            rot: random_rotation(),
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            color,
            shape,
            physics_handle: rbh,
            bounds_num,
            bounds: vec![],
        }
    }

    pub fn new_custom(position: Vec2, size: f32, bounds_num: usize, color: Color, physics: &mut Physics) -> Self {
        let settings = get_settings();
        let key = gen_range(u64::MIN, u64::MAX);
        let shape = SharedShape::ball(size*0.9);
        let rbh = physics.add_dynamic(key, &position, 0.0, shape.clone(), PhysicsProperities::default());
        Self {
            key: generate_key64(),
            pos: position,
            rot: random_rotation(),
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            color,
            shape,
            physics_handle: rbh,
            bounds_num,
            bounds: vec![],
        }
    }


    pub fn draw(&self) {
        let settings = get_settings();
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle(x0, y0, self.size, self.color);
    }    

    pub fn update(&mut self, physics: &mut Physics) {
        self.update_physics(physics);
    }

    fn draw_circle(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle_lines(x0, y0, self.size, 4.0, self.color);
    }

    fn update_physics(&mut self, physics: &mut Physics) {
        let settings = get_settings();
        let physics_data = physics.get_physics_data(self.physics_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        self.mass = physics_data.mass;
        //match physics.rigid_bodies.get_mut(self.physics_handle) {
        //    Some(body) => {
        //        //self.check_edges(body);
        ////        let dir = Vec2::from_angle(self.rot);
        ////        let v = dir * self.vel * settings.quant_speed;
        ////        let rot = self.ang_vel * settings.quant_rotate;
        ////        body.set_linvel(Vector2::new(v.x, v.y), true);
        ////        body.set_angvel(rot, true);
        //    }
        //    None => {}
        //}
    }

    fn check_edges(&mut self, body: &mut RigidBody) {
        let settings = get_settings();
        let mut raw_pos = matrix_to_vec2(body.position().translation);
        let mut out_of_edge = false;
        if raw_pos.x < -5.0 {
            raw_pos.x = settings.world_w as f32;
            out_of_edge = true;
        } else if raw_pos.x > settings.world_w as f32 + 5.0 {
            raw_pos.x = 0.0;
            out_of_edge = true;
        }
        if raw_pos.y < -5.0 {
            raw_pos.y = settings.world_h as f32;
            out_of_edge = true;
        } else if raw_pos.y > settings.world_h as f32 + 5.0 {
            raw_pos.y = 0.0;
            out_of_edge = true;
        }
        if out_of_edge {
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, self.rot), true);
            //body.set_linvel([0.0, 0.0].into(), true);
            //self.vel = 0.0;
        }
    }
}