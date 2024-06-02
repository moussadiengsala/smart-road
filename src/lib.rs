use lane::Stage;
use rand::Rng;
pub use sdl2;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;
use settings::BLOCKS;
use std::{cell::RefCell, slice};
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
    Straight,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Vilosity {
    Slow,
    Medium,
    Fast,
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
        if route == Direction::Down {
            match rng.gen_range(1, 3) {
                1 => lane.routes.iter_mut().nth(1).unwrap().add_vehicle(route),
                _ => lane.routes.iter_mut().nth(0).unwrap().add_vehicle(route),
            }
        } else if route == Direction::Right {
            lane.routes.iter_mut().nth(0).unwrap().add_vehicle(route)
        } else if route == Direction::Left {
            match rng.gen_range(1, 3) {
                1 => lane.routes.iter_mut().nth(1).unwrap().add_vehicle(route),
                _ => lane.routes.iter_mut().nth(0).unwrap().add_vehicle(route),
            }
        } else if route == Direction::Up {
            lane.routes.iter_mut().nth(0).unwrap().add_vehicle(route)
        }

        // let mut rng = rand::thread_rng();
        // match rng.gen_range(0, 3) {
        //     0 => lane.routes.iter_mut().nth(0).unwrap().add_vehicle(route),
        //     1 => lane.routes.iter_mut().nth(1).unwrap().add_vehicle(route),
        //     _ => lane.routes.iter_mut().nth(2).unwrap().add_vehicle(route),
        // }
    }
}

fn extract_routes_mut(lanes: &mut Vec<Lane>) -> Vec<&mut Route> {
    lanes
        .iter_mut()
        .flat_map(|lane| lane.routes.iter_mut())
        .collect()
}

fn chunk_routes<'a>(routes: Vec<&'a mut Route>, block: usize) -> Vec<&'a mut Route> {
    let mut chunks = Vec::new();

    for route in routes {
        for block_def in BLOCKS[block].iter() {
            if block_def == &(route.cross, route.itineraire) {
                chunks.push(&mut *route);
                break;
            }
        }
    }

    chunks
}

pub fn get_blocks(lanes: &mut Vec<Lane>) {
    // for block in 0..BLOCKS.len() {
        let routes: Vec<&mut Route> = extract_routes_mut(lanes);
        let mut routes_chunk = Rc::new(RefCell::new(chunk_routes(routes, 0)));

        if routes_chunk.borrow().iter().all(|r| r.vehicles.len() == 0) {
            return;
        }

        if routes_chunk
            .borrow()
            .iter()
            .any(|r| r.stage == Stage::Crossing) {
                // println!("-------------------a routes is in Crossing mode----------------");
            // the new vehicle that appears in the other road should be slow down
            for route in routes_chunk.borrow_mut().iter_mut() {
                if route.stage != Stage::Crossing  {
                    route.adjust_velocity_vehicle_in_route(Vilosity::Slow);
                }
            }
            return;
        }

        for route in routes_chunk.borrow_mut().iter_mut() {
            let mut vehicle_in_intersection = route
                .vehicles
                .iter()
                .filter(|v| v.stage == Stage::Crossing)
                .collect::<Vec<&Vehicle>>();
            if !vehicle_in_intersection.is_empty() && route.stage != Stage::Crossing {
                route.stage = Stage::Crossing;
                return;
            }
        }

        // if only on routes have vehicle their is no need to accelerate
        // let binding = routes_chunk.borrow();
        // let waiting_vehicles: Vec<&Vehicle> = binding
        //     .iter()
        //     .flat_map(|r| r.vehicles.iter().filter(|v| v.stage == Stage::Waiting))
        //     .collect();
        // // if waiting_vehicles.clone().len() < 2 {
        // //     return;
        // // }

        let min_distance_route_cross_itineraire = {
            let routes_chunk_borrowed = routes_chunk.borrow();
            let min_distance_route = routes_chunk_borrowed.iter()
                .min_by_key(|route| route.distance_to_stop_point())
                .unwrap();
            (min_distance_route.cross, min_distance_route.itineraire)
        };
        
        for route in routes_chunk.borrow_mut().iter_mut() {
            if (route.cross, route.itineraire) == min_distance_route_cross_itineraire {
                route.adjust_velocity_vehicle_in_route(Vilosity::Fast);
                route.stage = Stage::Crossing;
            } else {
                route.adjust_velocity_vehicle_in_route(Vilosity::Slow);
            }
        }
    // }
}

//     // for route in routes_chunk.iter_mut() {
//     //     if (route.clone().cross, route.clone().itineraire) == (a(&routes_chunk).cross, a(&routes_chunk).itineraire) {
//     //         route.adjust_velocity_vehicle_in_route(Vilosity::Fast);
//     //     } else {
//     //         route.adjust_velocity_vehicle_in_route(Vilosity::Slow);
//     //     }
//     // }

// for b in routes_chunk {
//     println!("{:?} {:?}", b.cross, b.itineraire);
// }

// for block in chunk_routes(all_routes_slice, 1).iter() {
//     // the block that has the min distance returned by the fynction distance_to_stop_point
//     // should use adjust_velocity_vehicle_in_route with vilosity_type = Vilosity::Fast the other Vilosity::Slow

// }
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
