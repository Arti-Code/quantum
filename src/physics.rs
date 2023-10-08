#![allow(unused)]

use crate::util::*;
use crate::globals::*;
use macroquad::prelude::*;
use rapier2d::na::Isometry2;
use rapier2d::na::{Point2, Vector2};
use rapier2d::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::consts::PI;
use crossbeam;

pub struct PhysicsProperities {
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

impl Default for PhysicsProperities {
    
    fn default() -> Self {
        Self { friction: 1.0, restitution: 0.0, density: 1.0, linear_damping: 0.0, angular_damping: 0.0 }
    }
}

impl PhysicsProperities {
    
    pub fn new(friction: f32, restitution: f32, density: f32, linear_damping: f32, angular_damping: f32) -> Self {
        Self { friction, restitution, density, linear_damping, angular_damping }
    }

    pub fn bounce() -> Self {
        Self { friction: 0.0, restitution: 1.0, density: 1.0, linear_damping: 0.1, angular_damping: 0.1 }
    }

    pub fn free() -> Self {
        Self { friction: 0.0, restitution: 1.0, density: 0.03, linear_damping: 0.0, angular_damping: 0.0 }
    }
}


pub struct Physics {
    pub attract_num: u32,
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    physics_hooks: (),
    event_handler: (),
    grav_time: f32,
}

impl Physics {

    pub fn new() -> Self {
        Self {
            attract_num: 0,
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity: Vector2::new(0.0, 0.0),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            physics_hooks: (),
            event_handler: (),
            grav_time: 0.0,
        }
    }

    pub fn step_physics(&mut self) {
        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        self.attract_num = 0;
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &event_handler,
            //&self.event_handler,
        );



/*         while let Ok(collision_event) = collision_recv.try_recv() {
            match collision_event {
                CollisionEvent::Started(_, _, CollisionEventFlags::SENSOR) => {
                    println!("sensor");
                },
                CollisionEvent::Started(_, _, CollisionEventFlags::REMOVED) => {
                    println!("removed");
                },
                CollisionEvent::Started(c1, c2, _) => {
                    let collider1 = self.colliders.get(c1).unwrap();
                    let collider2 = self.colliders.get(c2).unwrap();
                    let body1 = self.rigid_bodies.get(collider1.parent().unwrap()).unwrap();
                    let body2 = self.rigid_bodies.get(collider2.parent().unwrap()).unwrap();
                    let eng = body1.kinetic_energy() + body2.kinetic_energy();
                    let txt = match eng {
                        e if e < 1000.0 => {
                            format!("{}J", e.round())
                        },
                        e if e < 1000_000.0 => {
                            format!("{}kJ", (e/1000.0).round())
                        },
                        e if e < 1000_000_000.0 => {
                            format!("{}MJ", (e/1000_000.0).round())
                        },
                        e => {
                            format!("{}GJ", (e/1000_000_000.0).round())
                        },
                    };
                    println!("[COLLISION]: energy: {}", txt);
                },
                CollisionEvent::Stopped(_, _, _) => {

                },
            }
            //println!("Received collision event: {:?}", collision_event);
        } */

        //self.update_grav();
    }

    pub fn remove_physics_object(&mut self, body_handle: RigidBodyHandle) {
        _ = self.rigid_bodies.remove(body_handle, &mut self.island_manager, &mut self.colliders, &mut self.impulse_joint_set, &mut self.multibody_joint_set, true);
    }

    pub fn get_physics_obj_num(&self) -> usize {
        let body_num = self.rigid_bodies.len();
        return body_num;
    }

    fn get_body_handle_from_collider(&self, collider_handle: ColliderHandle) -> Option<RigidBodyHandle> {
        let collider: &Collider;
        match self.colliders.get(collider_handle) {
            Some(col) => {
                collider = col;
            }
            None => {
                return None;
            }
        };
        match collider.parent() {
            Some(rbh) => {
                return Some(rbh);
            }
            None => {
                return None;
            }
        }
    }

    fn add_dynamic_rigidbody(&mut self, key: u64, position: &Vec2, rotation: f32, linear_damping: f32, angular_damping: f32) -> RigidBodyHandle {
        let pos = Isometry2::new(Vector2::new(position.x, position.y), rotation);
        let dynamic_body = RigidBodyBuilder::dynamic().position(pos)
            .linear_damping(linear_damping).angular_damping(angular_damping).can_sleep(false).build();
        return self.rigid_bodies.insert(dynamic_body);
    }

    pub fn add_collider(&mut self, body_handle: RigidBodyHandle, rel_position: &Vec2, rotation: f32, shape: SharedShape, physics_props: PhysicsProperities) -> ColliderHandle {
        let iso = make_isometry(rel_position.x, rel_position.y, rotation);
        let collider = match shape.shape_type() {
            ShapeType::Ball => {
                let radius = shape.0.as_ball().unwrap().radius;
                ColliderBuilder::new(shape).position(iso).density(physics_props.density).friction(physics_props.friction).restitution(physics_props.restitution)
                    .active_collision_types(ActiveCollisionTypes::DYNAMIC_DYNAMIC).active_events(ActiveEvents::COLLISION_EVENTS).build()
            },
            ShapeType::ConvexPolygon => {
                ColliderBuilder::new(shape).density(physics_props.density).friction(physics_props.friction).restitution(physics_props.restitution)
                .active_collision_types(ActiveCollisionTypes::default()).active_events(ActiveEvents::COLLISION_EVENTS).build()
            },
            ShapeType::Compound => {
                ColliderBuilder::new(shape).density(physics_props.density).friction(physics_props.friction).restitution(physics_props.restitution)
                .active_collision_types(ActiveCollisionTypes::default()).active_events(ActiveEvents::COLLISION_EVENTS).build()
            },
            _ => {
                ColliderBuilder::ball(5.0).position(iso).build()
            },
        };
        return self.colliders.insert_with_parent(collider, body_handle, &mut self.rigid_bodies);
    }

    pub fn add_dynamic(&mut self, key: u64, position: &Vec2, rotation: f32, shape: SharedShape, physics_props: PhysicsProperities) -> RigidBodyHandle {
        let rbh = self.add_dynamic_rigidbody(key, position, rotation, physics_props.linear_damping, physics_props.angular_damping);
        _ = self.add_collider(rbh, &Vec2::ZERO, 0.0, shape, physics_props);
        //match self.rigid_bodies.get_mut(rbh) {
        //    Some(body) => {
        //        let settings = get_settings();
        //        let v = random_unit_vec2();
        //        let mut f = random_unit() * settings.force * v;
        //        body.apply_impulse(Vector2::new(f.x, f.y), true);
        //    },
        //    None => {},
        //}
        return rbh;
    }

    pub fn add_prismatic_joint(&mut self, body_handle1: RigidBodyHandle, body_handle2: RigidBodyHandle, anchors: (Point2<f32>, Point2<f32>)) -> ImpulseJointHandle {
        let p1 = anchors.0;
        let p2 = anchors.1;
        let joint = PrismaticJointBuilder::new(Vector2::x_axis())
            .local_anchor1(p1).local_anchor2(p2)
            .motor_velocity(0.0, 0.1)
            .build();
        let joint_handle = self.impulse_joint_set.insert(body_handle1, body_handle2, joint, true);
        return joint_handle;
    }

    pub fn get_physics_data(&self, handle: RigidBodyHandle) -> PhysicsData {
        if let Some(rb) = self.rigid_bodies.get(handle) {
            let iso = rb.position();
            let (pos, rot) = iso_to_vec2_rot(iso);
            let force = Vec2::new(rb.user_force().data.0[0][0], rb.user_force().data.0[0][1]);
            let data = PhysicsData {
                position: pos,
                rotation: rot,
                mass: rb.mass(),
                kin_eng: Some(rb.kinetic_energy()),
                force: Some(force),
            };
            return data;
        } else {
            return PhysicsData {
                position: Vec2::new(WORLD_W / 2., WORLD_H / 2.),
                rotation: 0.0,
                mass: 0.0,
                kin_eng: Some(0.0),
                force: None,
            };
        }
    }

    pub fn get_object_position(&self, handle: RigidBodyHandle) -> Option<Vec2> {
        let rb = self.rigid_bodies.get(handle);
        match rb {
            Some(body) => {
                let pos = Vec2::new(body.position().translation.x, body.position().translation.y);
                return Some(pos);
            }
            None => {
                return None;
            }
        }
    }

    pub fn get_object_size(&self, handle: RigidBodyHandle) -> Option<f32> {
        let rb = self.rigid_bodies.get(handle);
        match rb {
            Some(body) => {
                match body.colliders().first() {
                    Some(colh) => {
                        match self.colliders.get(*colh) {
                            Some(collider) => {
                                return Some(collider.shape().as_ball().unwrap().radius);
                            },
                            None => {
                                return None;
                            },
                        }
                    },
                    None => { 
                      return None; 
                    },
                }
            },
            None => {
                return None;
            },
        }
    }

    pub fn get_contacts_set(&mut self, agent_body_handle: RigidBodyHandle, radius: f32) -> HashSet<RigidBodyHandle> {
        let mut contacts: HashSet<RigidBodyHandle> = HashSet::new();
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if collider.is_sensor() {
                continue;
            }
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), &rapier2d::geometry::Ball::new(radius), filter,
                |collided| {
                    let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                    contacts.insert(rb2_handle);
                    return true;
                },
            );
        }
        return contacts;
    }

    pub fn get_closed_agent(&self, agent_body_handle: RigidBodyHandle, detection_range: f32) -> Option<RigidBodyHandle> {
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let pos1 = matrix_to_vec2(rb.position().translation);
        let mut dist = f32::INFINITY;
        let mut target: RigidBodyHandle = RigidBodyHandle::invalid();
        let detector = ColliderBuilder::ball(detection_range).sensor(true).density(0.0).build();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_collider: None,
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), detector.shape(), filter,
            |collided| {
                let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                let rb2 = self.rigid_bodies.get(rb2_handle).unwrap();
                let pos2 = matrix_to_vec2(rb2.position().translation);
                let new_dist = pos1.distance(pos2);
                if new_dist < dist {
                    dist = new_dist;
                    target = rb2_handle;
                }
                return true;
            },
        );
        if dist < f32::INFINITY {
            return Some(target);
        } else {
            return None;
        }
    }

    fn update_grav(&mut self) {
        self.grav_time += get_frame_time();
        if self.grav_time >= 0.25 {
            self.grav_time -= 0.25;
            let mut gravity_map: HashMap<RigidBodyHandle, Vec2> = HashMap::new();
            for (id1, body1) in self.rigid_bodies.iter() {
                let mut gforce = Vec2::ZERO;
                let pos1 = Vec2::new(body1.position().translation.x, body1.position().translation.y);
                let coll1 = self.colliders.get(*body1.colliders().first().unwrap()).unwrap();
                let size1 = coll1.shape().as_ball().unwrap().radius;
                for (id2, body2) in self.rigid_bodies.iter() { 
                    if id1 == id2 {
                        continue;
                    }
                    let pos2 = Vec2::new(body2.position().translation.x, body2.position().translation.y);
                    let dist = pos2.distance_squared(pos1);
                    if dist >= 10000.0 { continue; }
                    let coll2 = self.colliders.get(*body2.colliders().first().unwrap()).unwrap();
                    let size2 = coll2.shape().as_ball().unwrap().radius;
                    let dir = (pos1-pos2).normalize_or_zero();
                    gforce += (GRAV*dir * (size1+size2)) / dist; 
                }
                gravity_map.insert(id1, gforce);
            }
            for (rbh, gforce) in gravity_map.iter() {
                let rb = self.rigid_bodies.get_mut(*rbh).unwrap();
                rb.reset_forces(true);
                rb.add_force(vector![gforce.x, gforce.y], true);
            }
        }
    }
}

pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
    pub mass: f32,
    pub kin_eng: Option<f32>,
    pub force: Option<Vec2>,
}
