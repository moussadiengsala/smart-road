use std::rc::Rc;

use sdl2::{render::Canvas, video::Window};

use crate::{settings, Direction, Itineraire, Settings, Vehicle};

#[derive(Debug, Clone)]
pub struct Route {
    pub vehicles: Vec<Vehicle>,
    pub itineraire: Itineraire,
    pub settings: Rc<Settings>,
}

impl Route {
    pub fn new(itineraire: Itineraire, settings: Rc<Settings>) -> Route {
        Self { vehicles: Vec::new(), itineraire, settings }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        // self.stop_vehicules();

        for i in (0..self.vehicles.len()).rev() {
            self.vehicles[i].update(canvas);

            // Remove vehicles that have reached the end of the lane
            if self.vehicles[i].has_reached_end() {
                self.vehicles.remove(i);
            }
        }
    }

    pub fn add_vehicle(&mut self, route: Direction) {
        let mut vehicle =
            Vehicle::new(route, self.itineraire, self.settings.clone());
        vehicle.spawn(route);

        if let Some(last) = self.vehicles.clone().last() {
            if self.settings.safety_distance < vehicle.distance(last) {
                self.vehicles.push(vehicle);
            }
        } else {
            self.vehicles.push(vehicle);
        }
        println!("-------------- {:?} {}", self.itineraire, self.vehicles.len());
    }
}