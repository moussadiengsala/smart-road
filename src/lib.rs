use lane::Stage;
use rand::Rng;
pub use sdl2;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;
pub use std::{rc::Rc, time::Duration};

mod settings;
pub use settings::Settings;

mod map;
pub use map::draw_map;

mod cars;
pub use cars::Vehicle;

mod lane;
pub use lane::{Cross, Lane};

mod routes;
pub use routes::Route;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Itineraire {
    Left,
    Right,
    Straight
}

pub fn handle_keyboard_event(event: &Event, lanes: &mut Vec<Lane>, settings: Rc<Settings>) {
    let mut binding = Lane::new(Cross::First, settings);
    let (lane, route) = match event {
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => (lanes.iter_mut().nth(3).unwrap(), Direction::Up),
        Event::KeyDown {
            keycode: Some(Keycode::Down),
            ..
        } => (lanes.iter_mut().nth(0).unwrap(), Direction::Down),
        Event::KeyDown {
            keycode: Some(Keycode::Left),
            ..
        } => (lanes.iter_mut().nth(2).unwrap(), Direction::Left),
        Event::KeyDown {
            keycode: Some(Keycode::Right),
            ..
        } => (lanes.iter_mut().nth(1).unwrap(), Direction::Right),
        Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
        } => {
            let mut rng = rand::thread_rng();
            match rng.gen_range(0, 4) {
                0 => (lanes.iter_mut().nth(3).unwrap(), Direction::Up),
                1 => (lanes.iter_mut().nth(0).unwrap(), Direction::Down),
                2 => (lanes.iter_mut().nth(2).unwrap(), Direction::Left),
                _ => (lanes.iter_mut().nth(1).unwrap(), Direction::Right),
            }
        }
        _ => (&mut binding, Direction::None),
    };

    if route != Direction::None {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 3) {
            0 => lane.routes.iter_mut().nth(0).unwrap().add_vehicle(route),
            1 => lane.routes.iter_mut().nth(1).unwrap().add_vehicle(route),
            _ => lane.routes.iter_mut().nth(2).unwrap().add_vehicle(route),
        }
    }
}

// pub fn update_traffic_lights(lanes: &mut [Lane]) {
//     // Check if any lane is currently in the Crossing stage
//     if lanes.iter().any(|lane| lane.stage == Stage::Crossing) {
//         return;
//     }

//     let mut next_cross_lane = None;
//     let mut min_distance = f64::MAX;
//     let mut max_vehicle_count = 0;

//     for lane in lanes.iter_mut() {
//         let waiting_vehicles: Vec<&Vehicle> = lane
//             .vehicles
//             .iter()
//             .filter(|v| v.stage == Stage::Waiting)
//             .collect();

//         if !waiting_vehicles.is_empty() {
//             if let Some(distance) = lane.closest_vehicle_distance() {
//                 let vehicle_count = waiting_vehicles.len();
                
//                 if distance < min_distance || (distance == min_distance && vehicle_count > max_vehicle_count) {
//                     min_distance = distance;
//                     max_vehicle_count = vehicle_count;
//                     next_cross_lane = Some(lane);
//                 }
//             }
//         }
//     }

//     if let Some(lane) = next_cross_lane {
//         lane.stage = Stage::Crossing;
//     }
// }
