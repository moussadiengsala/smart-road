# Smart Road
## Overview

This project simulates traffic control at a road intersection in a city. The primary objective is to manage the flow of vehicles through the intersection using traffic lights and ensure that traffic congestion is minimized while avoiding collisions.


## Explaination
you'll find this followings files

### main file
```rust
// some impportation here ...

pub fn main() {
   // this is settings of the game I stroe diffrent dimensions like the canvas width ... and keys coordonates and more ...
   let settings = Rc::new(Settings::new(1000, 1000, 30, 1, 100.0));
   // tracking here diffrentes statistics of a vehicles so that to display it at the end ...
   let mut statistic: Statistics = Statistics::new();

    // sdl2 settings here ...

    // go see lane file for more infomation ...
    /*know each lane has 6 routes:
      * 3 for vehicles that go out of the canvas.
      * 3 for vehicles that come in of the canvas: but for this lane three there 3 routes:
         - one route Left: the vehicles that come into this route should turn at their left at some point.
         - one route Straight: the vehicles that come into this route should never turn.
         - one route Right: the vehicles that come into this route should turn at their right at some point.
    */
    let lanes: Rc<RefCell<Vec<Lane>>> = Rc::new(RefCell::new(vec![
        Lane::new(Cross::First, settings.clone()), // represent the (North) lane
        Lane::new(Cross::Second, settings.clone()), // represent the (West) lane
        Lane::new(Cross::Third, settings.clone()), // represent the (East) lane
        Lane::new(Cross::Fourth, settings.clone()), // represent the (South) lane
    ]));

    let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();
    'running: loop {
        // listening the event keyboard here ... 
        // drawing the map here ...
        draw_map(&mut canvas, settings.clone());

        // updates here diffrentes lanes ...
        {
            let mut lanes_borrowed = lanes.borrow_mut();
            for lane in lanes_borrowed.iter_mut() {
                lane.update(&mut canvas, &a, &mut statistic);
            }
        }

        // the smart road algorithm to avoid collisions
        smart_intersection(&mut lanes.borrow_mut());

        // ...
   }

   // dispply the statistics at the end ...
   statistic.display_statistics_window(&mut event_pump);
}

```

### lib file 
this is where reside the function to listen the keyboard and the smart road algorithm

```ruat
// some importations ...
// pub enum Direction with values Up, Down, Left, Right, None 
// pub enum Itineraire with values Left, Right, Straight cause for a lane these is only those 3 possiblities.
// pub enum Vilosity Reduce, Slow, Medium, Fast my diffrentes types of velosities in the game.

// this listen the keyboard user
pub fn handle_keyboard_event(event: &Event, lanes: &mut Vec<Lane>, settings: Rc<Settings>) {
    let mut binding = Lane::new(Cross::First, settings);
    let (lane, route) = match event {

        // if key Up choose the lane at 3 position meaning the (South) lane.
        Event::KeyDown {
            keycode: Some(Keycode::Up),
            ..
        } => (lanes.iter_mut().nth(3).unwrap(), Direction::Up),
        // ...
        // choose one randomly if key is (R)
    };

    // after chosing a lane you chose route Left or route Straight or route right.
    if route != Direction::None {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0, 3) {
            0 => lane.routes.iter_mut().nth(0).unwrap().add_vehicle(route),
            // ...
        }
    }
}

// the smart road road algorithm
pub fn smart_intersection(lanes: &mut Vec<Lane>) {
    // ...
}
```

### map file
nothing to say here it's just for drawing the map game.

### settings file 
this is like I say in the main the game settings let me give litte more infomations about it.

```rust 
   // some importations ...

   struct Statistics {
      max_vehicles_passed // the totale vehicle
      max_velocities // max velosity of a vehicle during his travels
      min_velocities // min velosity of a vehicle during his travels
      time_to_pass // the travels time of a vehicle
      max_velocity // the max of max_velocities
      min_velocity // the min of min_velocities
      max_time_to_pass // the max of time_to_pass
      min_time_to_pass // the min of time_to_pass
      close_calls // everytime a collision is avoid this is incremented
   }

   // some methods of Statistics here

   struct Settings {
      width // canvas width
      height // canvas height
      vehicle // vehicle size
      safety_distance // safety distance
      vertical_key_points // all key point in axis x
      horizontal_key_points // all key point in axis y

      // ...
   }
   // some methods of Statistics here

   // this function load the diffrentes texture in assets and return a vec of them.
   // the struct vehicle has field name texture of type usize represent the position of a texture in the vec return by this function.
   pub fn cars_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Vec<Texture<'a>> {
      // ...

      let texture_names = vec!["BlackOut.png", "WhiteStrip.png", "BlueStrip.png", "GreenStrip.png", "RedStrip.png", "PinkStrip.png"];
      let mut cars_textures = Vec::new();
      // ..
      cars_textures
   }

   // this is a part of the smart road algorithm
   pub struct BLOCK<'a> {
      lane // a given route
      intersections // all route that could be in intersection with lane.
   }

   // go on lib file to the smart road algorithm to see his use case
   /*
      this variable is the key of the smart road algorithm let me break it down to make sure you know what i mean here.

      evertime I iterate over field intersections slice in BLOCK I look these:
      - among the routes in intersections who has the most vehicle in the 'smart road intersections' or
      - among the routes in intersections who has the closest vehicle to the 'stop point (the stop point is the point before the "smart road intersections")' 

      if the resulted route of thes two is the same as lane field in BLOCK I let his vehicle cross otherwise I reduce the vilosity of his vehicles. 
   */
   pub(crate) const BLOCKS: &[&BLOCK] = &[
      // example: (North)
      &BLOCK{
         lane: (Cross::First, Itineraire::Left), // North-Left route
         // the North-Left route is in intersection with these followings routes.
         intersections: &[
               (Cross::First, Itineraire::Left), // North-Left itself
               (Cross::Fourth, Itineraire::Straight), // South-Straight
               (Cross::Fourth, Itineraire::Left), // South-Left
               (Cross::Third, Itineraire::Left), // East-Left
               (Cross::Third, Itineraire::Straight), // East-Straight
               (Cross::Second, Itineraire::Left), // West-Left
         ],
      },
      // ...
  
   ];

```

### lane file
```rust

   // ...

   struct Lane {
      routes // his diffrentes routes (Left, Straight, Right)
      cross // values First(North), Second(West), Three(East), Fourth(South)
      stage // values Waiting(his vehicles should reduce velosity), Crosing(...)
      stop_point // point before enter the intersection
      settings // the game settings
   }

   // some method of Lane
```

### route file
```rust 
   // ...
   struct Route {
      vehicles // vehicle in the route
      itineraire // which route this is Left, Straight or Right.
      // ...
   }
   // some method of Route
```
### cars file

## Notions

   - Documentation for SDL2.

## Getting Started
### Prerequisites

   - Rust programming language installed
   - SDL2 library

### Installation

    1. Clone the repository:
   ```sh
   git clone https://github.com/moussadiengsala/road_intersection.git
   cd road_intersection
   ```

    2. Install dependencies:

   ```sh
    cargo build
   ```

    3. Usage
   ```sh
    cargo run
   ```

   Use the keyboard controls to spawn vehicles and observe the traffic flow.


## Demo

[video demo](media/demo.mp4)

## Contributing
Contributions are welcome! Please open an issue or submit a pull request.

## License
This project is licensed under the MIT License.

## Authors

- [@Moussa Dieng](https://www.moussa-dieng.dev)