use lane::Stage;
use rand::Rng;
pub use sdl2;
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;
use settings::BLOCKS;
use std::{cell::RefCell, time::Instant};
pub use std::{rc::Rc, time::Duration};

mod settings;
pub use settings::{Settings, Statistics, cars_texture};

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
    Reduce,
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
        match rng.gen_range(0, 3) {
            0 => lane.routes.iter_mut().nth(0).unwrap().add_vehicle(route),
            1 => lane.routes.iter_mut().nth(1).unwrap().add_vehicle(route),
            _ => lane.routes.iter_mut().nth(2).unwrap().add_vehicle(route),
        }
    }
}

fn extract_routes_mut(lanes: &mut Vec<Lane>) -> Vec<&mut Route> {
    lanes
        .iter_mut()
        .flat_map(|lane| lane.routes.iter_mut())
        .collect()
}

fn chunk_routes<'a>(routes: Vec<&'a mut Route>, block: &[(Cross, Itineraire)]) -> Vec<&'a mut Route> {
    let mut chunks = Vec::new();

    for route in routes {
        for block_def in block.iter() {
            if block_def == &(route.cross, route.itineraire) {
                chunks.push(&mut *route);
                break;
            }
        }
    }

    chunks
}

pub fn smart_intersection(lanes: &mut Vec<Lane>) {
    for block in BLOCKS.iter() {
        let routes: Vec<&mut Route> = extract_routes_mut(lanes);
        let routes_chunk = Rc::new(RefCell::new(chunk_routes(routes, block.intersections)));

        // there is nothing to do if any of the intersection road has a vehicle.
        if routes_chunk.borrow().iter().any(|r| (r.cross, r.itineraire) == block.lane && r.vehicles.len() == 0) {
            continue;
        }

        if routes_chunk.borrow().iter().any(|r| r.stage == Stage::Crossing) {
            for r in routes_chunk.borrow_mut().iter_mut() {
                if (r.cross, r.itineraire) == block.lane && r.stage != Stage::Crossing {
                    r.other_route_crossed = true;
                    break;
                }
            }
            continue;
        }

        if let Some(c) = routes_chunk.borrow_mut().iter_mut()
            .max_by_key(|route| route.vehicles
                .iter()
                .filter(|v: &&Vehicle| v.stage == Stage::Crossing)
                .collect::<Vec<&Vehicle>>().len()
        ) {
            if (c.cross, c.itineraire) == block.lane {
                c.time = Instant::now();
                c.stage = Stage::Crossing;
                continue;
            }
        }

        let mut b = routes_chunk.borrow_mut();
        if let Some(c) = b.iter_mut()
            .filter(|r| r.vehicles.len() != 0)
            .min_by_key(|route| {
                route.distance_to_stop_point()
            }) {
                if (c.cross, c.itineraire) == block.lane {
                    c.time = Instant::now();
                    c.stage = Stage::Crossing;
                    continue;
                }
        } else if let Some(c) = b.iter_mut()
            .max_by_key(|r| r.vehicles.len()) {
            if (c.cross, c.itineraire) == block.lane {
                c.time = Instant::now();
                c.stage = Stage::Crossing;
                continue;
            }
        }
    
    }
}


/*
 pub fn smart_intersection(lanes: &mut Vec<Lane>) {
    for block in BLOCKS.iter() {
        let routes: Vec<&mut Route> = extract_routes_mut(lanes);
        let routes_chunk = Rc::new(RefCell::new(chunk_routes(routes, block.intersections)));

        // there is nothing to do if any of the intersection road has a vehicle.
        if routes_chunk.borrow().iter().any(|r| r.stage == Stage::Crossing) 
        || routes_chunk.borrow().iter().any(|route| (route.cross, route.itineraire) == block.lane && route.vehicles.len() == 0) {
            continue;
        }

        if let Some(c) = routes_chunk.borrow_mut().iter_mut()
            .max_by_key(|route| route.vehicles
                .iter()
                .filter(|v: &&Vehicle| v.stage == Stage::Crossing)
                .collect::<Vec<&Vehicle>>().len()
        ) {
            if (c.cross, c.itineraire) == block.lane {
                c.stage = Stage::Crossing;
                continue;
            }
        }

        let mut b = routes_chunk.borrow_mut();
        if let Some(c) = b.iter_mut()
            .filter(|r| r.vehicles.len() != 0)
            .min_by_key(|route| {
                route.distance_to_stop_point()
            }) {
                if (c.cross, c.itineraire) == block.lane {
                    c.stage = Stage::Crossing;
                    continue;
                }
        } else if let Some(c) = b.iter_mut()
            .max_by_key(|r| r.vehicles.len()) {
            if (c.cross, c.itineraire) == block.lane {
                c.stage = Stage::Crossing;
                continue;
            }
        }
    
    }
}
 */