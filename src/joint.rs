#![allow(unused)]

use std::collections::HashMap;
use std::f32::consts::PI;
use crate::util::*;
use crate::physics::*;
use crate::globals::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Point2;
use rapier2d::na::Vector2;
use rapier2d::prelude::ImpulseJointHandle;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};


pub struct MyJoint {
    pub color: color::Color,
    pub physics_handle: ImpulseJointHandle,
    pos1: Vec2,
    pos2: Vec2,
}



impl MyJoint {
    
    pub fn new(body_handle1: RigidBodyHandle, body_handle2: RigidBodyHandle, anchors: (Point2<f32>, Point2<f32>), physics: &mut Physics) -> Self {
        let color = random_color();
        let handle = physics.add_prismatic_joint(body_handle1, body_handle2, anchors);
        Self {
            color,
            physics_handle: handle,
            pos1: Vec2::ZERO,
            pos2: Vec2::ZERO,
        }
    }

    pub fn draw(&self) {
        draw_line(self.pos1.x, self.pos1.y, self.pos2.x, self.pos2.y, 3.0, self.color);
    }    

    pub fn update(&mut self, physics: &mut Physics) {
        match physics.impulse_joint_set.get(self.physics_handle) {
            Some(joint) => {
                match physics.rigid_bodies.get(joint.body1) {
                    Some(rb1) => {
                        let (pos1, rot1) = iso_to_vec2_rot(rb1.position());
                        self.pos1 = pos1;
                    },
                    None => {

                    },
                }
                match physics.rigid_bodies.get(joint.body2) {
                    Some(rb2) => {
                        let (pos2, rot2) = iso_to_vec2_rot(rb2.position());
                        self.pos2 = pos2;
                    },
                    None => {
                        
                    }
                }
            },
            None => {

            },
        }
    }

}