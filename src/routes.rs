use std::{rc::Rc, sync::Arc, time::{Duration, Instant}};

use rand::Rng;
use sdl2::{rect::Point, render::{Canvas, Texture}, video::Window};

use crate::{lane::Stage, settings, Cross, Direction, Itineraire, Settings, Statistics, Vehicle, Vilosity};

#[derive(Debug, Clone)]
pub struct Route {
    pub vehicles: Vec<Vehicle>,
    pub itineraire: Itineraire,
    pub cross: Cross,
    pub stop_point: Point,
    pub settings: Rc<Settings>,
    pub stage: Stage,
    pub waiting_since: Option<Instant>,
    pub is_vehicle_in_intersection: bool,
}

impl Route {
    pub fn new(
        itineraire: Itineraire,
        cross: Cross,
        stop_point: Point,
        settings: Rc<Settings>,
    ) -> Route {
        Self {
            vehicles: Vec::new(),
            itineraire,
            settings,
            cross,
            stage: Stage::Waiting,
            stop_point,
            is_vehicle_in_intersection: false,
            waiting_since: None,
        }
    }

    fn set_stage(&mut self) {
        if self.stage == Stage::Waiting {
            return;
        }

        if self.vehicles.is_empty() {
            self.stage = Stage::Waiting;
            self.waiting_since = None;
            return;
        }
        
        let vehicle_in_intersection = self
            .vehicles
            .iter_mut()
            .filter(|v| v.stage == Stage::Crossing)
            .collect::<Vec<&mut Vehicle>>();

        if vehicle_in_intersection.is_empty() && self.is_vehicle_in_intersection {
            self.stage = Stage::Waiting;
            self.is_vehicle_in_intersection = false;
        }

        if !vehicle_in_intersection.is_empty() {
            self.is_vehicle_in_intersection = true;
        }
 
    }

    pub fn distance_to_stop_point(&self) -> usize {
        for i in 0..self.vehicles.len() {
            match self.cross {
                Cross::First => {
                    if self.vehicles[i].position.y < self.stop_point.y {
                        return (self.stop_point.y - self.vehicles[i].position.y).abs() as usize;
                    }
                }
                Cross::Second => {
                    if self.vehicles[i].position.x < self.stop_point.x {
                        return (self.stop_point.x - self.vehicles[i].position.x).abs() as usize;
                    }
                }
                Cross::Third => {
                    if self.vehicles[i].position.x > self.stop_point.x {
                        return (self.vehicles[i].position.x - self.stop_point.x).abs() as usize;
                    }
                }
                Cross::Fourth => {
                    if self.vehicles[i].position.y > self.stop_point.y {
                        return (self.vehicles[i].position.y - self.stop_point.y).abs() as usize;
                    }
                }
            }
        }

        10000
    }
    

    pub fn adjust_velocity_vehicle_in_route(&mut self) {
        if self.itineraire == Itineraire::Right {
            return;
        }

        // test all use case of cmp
        // check everywhere you use self.stop_point
        let cmp = |point_1: Point, point_2: Point| -> f64 {
            if point_1.x == 0 {
                return (point_1.y - point_2.y).abs().into();
            } else {
                return (point_1.x - point_2.x).abs().into();;
            }
        };
        
        // if self.stage == Stage::Crossing {
        //     // println!("Crossing {:?} {:?}", self.cross, self.itineraire);
        // }
        // it's better to control all velocity here

        
        // - if Waiting don't touch their vilosity until:
        //     - at 2 * safty_distance this should be Slow. 
        //     - at safty_distance this should be Reduce. 
        // - if Crossing accelerate the vehicle that are in 2 * safty_distance and those in the intersection too. 

        // with this you only controle when to set Crossing a route. 

        // - filter the vehicle that are already cross and random their vilosity.
        let mut rng = rand::thread_rng();
        for vehicle in self.vehicles.iter_mut().filter(|v| v.stage == Stage::Crossed) {
            vehicle.velocity = vehicle.velosity_type[rng.gen_range(2,4)]
        } 

        if self.stage == Stage::Crossing && self.vehicles.len() != 0 {
            // && cmp(self.stop_point, v.position) < 2.0 * self.settings.safety_distance)
            for vehicle in self.vehicles.iter_mut()
                .filter(|v| v.stage == Stage::Crossing || 
                    (v.stage == Stage::Waiting)) {
                vehicle.set_vilosity(Vilosity::Fast);
            }
        } else if self.stage == Stage::Waiting && self.vehicles.len() != 0 {
             // Ralentir les véhicules qui sont à moins de 2 * safety_distance mais à plus de safety_distance
            // for vehicle in self.vehicles.iter_mut()
            //     .filter(|v| cmp(self.stop_point, v.position) < 2.0 * self.settings.safety_distance && 
            //                 cmp(self.stop_point, v.position) > self.settings.safety_distance) {
            //     vehicle.set_vilosity(Vilosity::Slow);
            // }

            // Réduire la vitesse des véhicules qui sont à moins de safety_distance
            for vehicle in self.vehicles.iter_mut()
                .filter(|v| v.stage == Stage::Crossing ||  cmp(self.stop_point, v.position) < self.settings.safety_distance) {
                vehicle.set_vilosity(Vilosity::Reduce);
            }
        }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>, texture: &Vec<Texture>, statistic: &mut Statistics) {
        self.set_stage();
        self.adjust_velocity_vehicle_in_route();

        for i in (0..self.vehicles.len()).rev() {
            if i > 0 {
                if let Some(other) = &self.vehicles.clone().get(i - 1) {
                    self.vehicles[i].adjust_velocity(other);
                }
            }
            self.vehicles[i].update(canvas, &texture);

            // Remove vehicles that have reached the end of the lane
            if self.vehicles[i].has_reached_end() {
        
                self.vehicles.remove(i);
            }
        }
    }

    pub fn add_vehicle(&mut self, route: Direction) {
        let mut vehicle = Vehicle::new(route, self.itineraire, self.settings.clone());
        vehicle.spawn(route);

        if let Some(last) = self.vehicles.clone().last() {
            if self.settings.safety_distance < vehicle.distance(last) {
                self.vehicles.push(vehicle);
            }
        } else {
            self.vehicles.push(vehicle);
        }
        println!(
            "--------------* {:?} {}",
            self.itineraire,
            self.vehicles.len()
        );
    }
}