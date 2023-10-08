#![allow(unused)]


use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use crate::util::*;
use crate::physics::*;
use crate::quant::*;
use crate::globals::*;
use macroquad::prelude::*;
use rapier2d::prelude::RigidBodyHandle;

pub trait PhysicsObject {
    fn new() -> Self;
    fn draw(&self, selected: bool, font: &Font);
    fn update(&mut self, dt: f32, physics: &mut Physics) -> bool;
    fn update_physics(&mut self, physics: &mut Physics);
    fn link_physics_handle(&mut self, handle: RigidBodyHandle);
}

pub struct QuantumCollector {
    pub quants: HashMap<RigidBodyHandle, Quant>,
}

impl QuantumCollector {
    pub fn new() -> Self {
        Self {
            quants: HashMap::new(),
        }
    }

    pub fn add_many_quants(&mut self, quants_num: usize, physics: &mut Physics) {
        for _ in 0..quants_num {
            let quant = Quant::new(6.0, 3, WHITE, physics);
            _ = self.add_quant(quant);
        }
    }

    pub fn add_quant(&mut self, quant: Quant) -> RigidBodyHandle {
        let h = quant.physics_handle.clone();
        self.quants.insert(quant.physics_handle, quant);
        return h;
    }

    pub fn get(&self, id: RigidBodyHandle) -> Option<&Quant> {
        return self.quants.get(&id);
    }

    pub fn remove(&mut self, id: RigidBodyHandle) {
        self.quants.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<RigidBodyHandle, Quant> {
        return self.quants.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<RigidBodyHandle, Quant> {
        return self.quants.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.quants.len();
    }

}

pub struct ElementsBox {

}