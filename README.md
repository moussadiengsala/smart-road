# smart-road

// lane.rs
 pub fn g(&self, lanes: Vec<Self>) {
        let all_route: Vec<&Route> = lanes.iter()
            .flat_map(|lane| lane.routes.iter())
            .collect();
        
        for route in self.routes.iter() {
            // route.can_cross(all_route.clone());
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



// routes.rs

 pub fn can_cross(&mut self, routes: Vec<&Self>)  {

    }

// lib.rs



// fn extract_routes_mut(lanes: &mut Vec<Lane>) -> Vec<&mut Route> {
//     lanes.iter_mut()
//         .flat_map(|lane| lane.routes.iter_mut())
//         .collect()
// }


// fn chunk_routes<'a>(
//     routes: Vec<&'a mut Route>,
//     block: usize,
// ) -> Vec<&'a mut Route> {
//     let mut chunks = Vec::new();

//     for route in routes {
//         for block_def in BLOCKS[block].iter() {
//             if block_def == &(route.cross, route.itineraire) {
//                 chunks.push(&mut *route);
//                 break;
//             }
//         }
//     }
    
//     chunks
// }


// pub fn get_blocks(lanes: &mut Vec<Lane>) {
//     let routes: Vec<&mut Route> = extract_routes_mut(lanes);
//     let routes_chunk: Vec<&mut Route> = chunk_routes(routes, 1);

//     if routes_chunk.iter().any(|r| r.stage == Stage::Crossing) {
//         return;
//     }

//     let min_distance_route = routes_chunk.iter()
//         .min_by_key(|&route| route.distance_to_stop_point())
//         .unwrap();

//     // Adjust velocity based on the found route
//     for i in 0..routes_chunk.len() {
//         let route = &mut routes_chunk[i].clone();
//         if route.cross == min_distance_route.cross {
//             route.adjust_velocity_vehicle_in_route(Vilosity::Fast);
//             route.stage = Stage::Crossing;
//         } else {
//             route.adjust_velocity_vehicle_in_route(Vilosity::Slow);
//         }
//     }
// }


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


// main.rs

use std::cell::RefCell;

use sdl2::image::{self, InitFlag, LoadTexture};
use smart_road::*;

pub fn main() {
    let settings = Rc::new(Settings::new(1000, 1000, 30, 1, 60.0));

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // Initialize SDL2_image
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let window = video_subsystem
        .window("smart road", settings.width as u32, settings.height as u32)
        .position_centered()
        .build()
        .unwrap();
    
    let mut canvas = window.into_canvas().build().unwrap();

    
    // Load an image as a texture
    // let texture_creator = canvas.texture_creator();
    // let texture = texture_creator
    //     .load_texture("./unnamed.png")
    //     .unwrap();


    // canvas.clear();
    let lanes: Rc<RefCell<Vec<Lane>>> = Rc::new(RefCell::new(vec![
        Lane::new(Cross::First, settings.clone()),
        Lane::new(Cross::Second, settings.clone()),
        Lane::new(Cross::Third, settings.clone()),
        Lane::new(Cross::Fourth, settings.clone()),
    ]));
    
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(55, 64, 5));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {
                    handle_keyboard_event(&event,&mut lanes.borrow_mut(), settings.clone());
                }
            }
        }

        canvas.clear();
        // The rest of the game loop goes here...

        // load the image
        // canvas.copy(&texture, None, None).unwrap();

        // map
        draw_map(&mut canvas, settings.clone());

        {
            let mut lanes_borrowed = lanes.borrow_mut();
            for lane in lanes_borrowed.iter_mut() {
                lane.update(&mut canvas);
            }
        }

        {
            let lanes_borrowed = lanes.borrow();
            for lane in lanes_borrowed.iter() {
                lane.g(lanes_borrowed.clone());
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
