use std::rc::Rc;

use crate::{Itineraire, Route};
use crate::{cars::Vehicle, Direction};
use crate::settings::Settings;
use rand::Rng;
use sdl2::{rect::Point, render::Canvas, video::Window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cross {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Crossing,
    Waiting,
}

#[derive(Debug, Clone)]
pub struct Lane {
    pub routes: Vec<Route>,
    pub cross: Cross,
    pub stage: Stage,
    pub stop_point: Point,
    pub settings: Rc<Settings>,
}

impl Lane {
    pub fn new(cross: Cross, settings: Rc<Settings>) -> Lane {
        Lane {
            routes: vec![
                Route::new(Itineraire::Left, settings.clone()),
                Route::new(Itineraire::Straight, settings.clone()),
                Route::new(Itineraire::Right, settings.clone()),
            ],
            cross,
            stage: Stage::Waiting,
            stop_point: match cross {
                Cross::First => settings.stop_point_first,
                Cross::Second => settings.stop_point_second,
                Cross::Third => settings.stop_point_third,
                Cross::Fourth => settings.stop_point_fourth,
            },
            settings,
        }
    }

    // pub fn closest_vehicle_distance(&self) -> Option<f64> {
    //     self.vehicles
    //         .iter()
    //         .map(|vehicle| vehicle.distance_to(self.stop_point))
    //         .min_by(|a, b| a.partial_cmp(b).unwrap())
    // }

    // pub fn stop_vehicules(&mut self) {
    //     let stop_point = match self.cross {
    //         Cross::First => self.settings.stop_point_first,
    //         Cross::Second => self.settings.stop_point_second,
    //         Cross::Third => self.settings.stop_point_third,
    //         Cross::Fourth => self.settings.stop_point_fourth,
    //     };

    //     let mut vehicles = self.vehicles.iter_mut().collect::<Vec<&mut Vehicle>>();
    //     for i in 0..vehicles.len() {
    //         let can_move = if let Some(next_vehicle) = vehicles.iter().nth((i as i32 - 1) as usize)
    //         {
    //             vehicles[i].distance(next_vehicle) > self.settings.safety_distance
    //         } else {
    //             true
    //         };

    //         if (vehicles[i].position == stop_point && self.stage == Stage::Waiting) || !can_move {
    //             vehicles[i].is_stopped = true;
    //         }

    //         if self.stage == Stage::Crossing && vehicles[i].is_stopped {
    //             vehicles[i].is_stopped = false;
    //         }
    //     }
    // }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        for i in (0..self.routes.len()).rev() {
            self.routes[i].update(canvas);
        }
    }

   
}
