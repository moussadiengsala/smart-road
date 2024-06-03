use std::{rc::Rc, sync::Arc, time::{Duration, Instant}};

use sdl2::{rect::Point, render::Canvas, video::Window};

use crate::{lane::Stage, settings, Cross, Direction, Itineraire, Settings, Vehicle, Vilosity};

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
        // println!("------------------------- {:?} {:?}", self.cross, self.itineraire);
        
        let mut vehicle_in_intersection = self
            .vehicles
            .iter_mut()
            .filter(|v| v.stage == Stage::Crossing)
            .collect::<Vec<&mut Vehicle>>();

        for v in vehicle_in_intersection.iter_mut() {
            if v.velocity != 3.0 {
                v.set_vilosity(Vilosity::Fast);
            }
        }

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
    

    pub fn adjust_velocity_vehicle_in_route(&mut self, vilosity_type: Vilosity) {
        it's better to control all velocity here

        - filter the vehicle that are already cross and random their vilosity.
        - if Waiting don't touch their vilosity until:
            - at 2 * safty_distance this should be Slow. 
            - at safty_distance this should be Reduce. 
        - if Crossing accelerate the vehicle that are in 2 * safty_distance and those in the intersection too. 

        with this you only controle when to set Crossing a route. 
        // println!("------------  ------------------------{:?}", vilosity_type);
        // for i in 0..self.vehicles.len() {
        //     match self.cross {
        //         Cross::First => {
        //             if self.vehicles[i].position.y < self.stop_point.y {
        //                 println!("Cross ------ {}", self.vehicles[i].distance_to(self.stop_point) < self.settings.safety_distance);
        //                 if self.vehicles[i].distance_to(self.stop_point) < self.settings.safety_distance && vilosity_type == Vilosity::Slow {
        //                     self.vehicles[i].set_vilosity(Vilosity::Reduce);
        //                 } else {
        //                     self.vehicles[i].set_vilosity(vilosity_type);
        //                 }
        //                 break;
        //             }
        //         }
        //         Cross::Second => {
        //             if self.vehicles[i].position.x < self.stop_point.x {
        //                 if self.vehicles[i].distance_to(self.stop_point) < self.settings.safety_distance && vilosity_type == Vilosity::Slow {
        //                     self.vehicles[i].set_vilosity(Vilosity::Reduce);
        //                 } else {
        //                     self.vehicles[i].set_vilosity(vilosity_type);
        //                 }
        //                 break;
        //             }
        //         }
        //         Cross::Third => {
        //             if self.vehicles[i].position.x > self.stop_point.x {
        //                 if self.vehicles[i].distance_to(self.stop_point) < self.settings.safety_distance && vilosity_type == Vilosity::Slow {
        //                     self.vehicles[i].set_vilosity(Vilosity::Reduce);
        //                 } else {
        //                     self.vehicles[i].set_vilosity(vilosity_type);
        //                 }
        //                 break;
        //             }
        //         }
        //         Cross::Fourth => {
        //             if self.vehicles[i].position.y > self.stop_point.y {
        //                 if self.vehicles[i].distance_to(self.stop_point) < self.settings.safety_distance && vilosity_type == Vilosity::Slow {
        //                     self.vehicles[i].set_vilosity(Vilosity::Reduce);
        //                 } else {
        //                     self.vehicles[i].set_vilosity(vilosity_type);
        //                 }
        //                 break;
        //             }
        //         }
        //     }
        // }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        self.set_stage();

        for i in (0..self.vehicles.len()).rev() {
            if i > 0 {
                if let Some(other) = &self.vehicles.clone().get(i - 1) {
                    self.vehicles[i].adjust_velocity(other);
                }
            }
            self.vehicles[i].update(canvas);

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