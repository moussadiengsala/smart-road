use std::rc::Rc;

use rand::Rng;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{lane::Stage, Direction, Itineraire, Settings, Vilosity};

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub position: Point,
    pub route: Direction,
    pub itineraire: Itineraire,
    pub direction: f32,
    pub velocity: f32,
    pub is_changed_direction: bool,
    pub is_stopped: bool,
    pub stage: Stage,
    pub velosity_type: Vec<f32>,

    pub distance_traveled: f64,
    pub time: f64,

    accumulated_x: f32,
    accumulated_y: f32,

    settings: Rc<Settings>,
}

impl Vehicle {
    pub fn new(route: Direction, itineraire: Itineraire, settings: Rc<Settings>) -> Self {
        let velosity_type = vec![0.1, 0.5, 2.0, 3.0];
        let mut rng = rand::thread_rng();
        
        Self {
            position: Point::new(0, 0),
            route,
            itineraire,
            direction: match route {
                Direction::Up | Direction::Left => -1.0,
                Direction::Down | Direction::None | Direction::Right => 1.0,
            },
            velocity: velosity_type[rng.gen_range(2,4)],
            velosity_type,
            is_changed_direction: false,
            is_stopped: false,
            distance_traveled: 0.0,
            stage: Stage::Waiting,
            time: 0.0,
            settings,
            accumulated_x: 0.0,
            accumulated_y: 0.0,
        }
    }

    pub fn set_vilosity(&mut self, vehicle_type: Vilosity) {
        let i = match vehicle_type {
            Vilosity::Reduce => 0,
            Vilosity::Slow => 1,
            Vilosity::Medium => 2,
            Vilosity::Fast => 3,
            
        };
        self.velocity = self.velosity_type[i];
    }

    pub fn adjust_velocity(&mut self, vehicles: &Vehicle) {
        // if you are at safty distance behind a vehicle you shoud have his velocity.
        if self.distance(vehicles) < self.settings.safety_distance + 10.0 {
            self.velocity = vehicles.velocity;
        }
    }

    pub fn has_reached_end(&self) -> bool {
        let border_x = self.position.x < -self.settings.vehicle
            || self.position.x > self.settings.width + self.settings.vehicle;
        let border_y = self.position.y < -self.settings.vehicle
            || self.position.y > self.settings.height + self.settings.vehicle;

        border_x || border_y
    }

    pub fn distance(&self, other: &Self) -> f64 {
        let dx = self.position.x as f64 - other.position.x as f64;
        let dy = self.position.y as f64 - other.position.y as f64;
        ((dx * dx) + (dy * dy)).sqrt()
    }

    pub fn distance_to(&self, point: Point) -> f64 {
        let dx = self.position.x as f64 - point.x as f64;
        let dy = self.position.y as f64 - point.y as f64;
        ((dx * dx) + (dy * dy)).sqrt()
    }

    pub fn stop(&mut self) {
        self.is_stopped = true;
    }

    pub fn resume(&mut self) {
        self.is_stopped = false;
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>) {
        if self.velocity == 0.1 {
            println!("jjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjj");
        }
        if !self.is_stopped {
            self.move_forward();
        }

        canvas.set_draw_color(Color::GREEN);
        let rect = Rect::new(
            self.position.x,
            self.position.y,
            self.settings.vehicle as u32,
            self.settings.vehicle as u32,
        );
        canvas.fill_rect(rect).unwrap();
    }

    fn set_stage(&mut self) {
        let len = self.settings.horizontal_key_points.len();
        let y = &self.settings.horizontal_key_points;
        let x = &self.settings.vertical_key_points;

        match self.route {
            Direction::Up => {
                match self.itineraire {
                    Itineraire::Left => {
                        if self.position.y < y[len - 2] && self.position.x > x[4] {
                            self.stage = Stage::Crossing;
                        } else if self.position.x < x[4]  {
                            self.stage = Stage::Crossed;
                        }
                    },
                    Itineraire::Straight => {
                        if self.position.y < y[len - 2] && self.position.y > y[4] {
                            self.stage = Stage::Crossing;
                        } else if self.position.y < y[4] {
                            self.stage = Stage::Crossed;
                        }
                    },
                    _ => (),
                }
            },
            Direction::Down => {
                match self.itineraire {
                    Itineraire::Left => {
                        if self.position.y > y[2] && self.position.x < x[len - 4] {
                            self.stage = Stage::Crossing;
                        } else if self.position.x > x[len - 4]{
                            self.stage = Stage::Crossed;
                        }
                    },
                    Itineraire::Straight => {
                        if self.position.y > y[2] && self.position.y < y[len - 4] {
                            self.stage = Stage::Crossing;
                        } else if self.position.y > y[len - 4] {
                            self.stage = Stage::Crossed;
                        }
                    },
                    _ => (),
                }
            },
            Direction::Left => {
                match self.itineraire {
                    Itineraire::Left => {
                        if self.position.x < x[len - 2] && self.position.y < y[len - 4] {
                            self.stage = Stage::Crossing;
                        } else if  self.position.y > y[len - 4] {
                            self.stage = Stage::Crossed;
                        }
                    },
                    Itineraire::Straight => {
                        if self.position.x < x[len - 2] && self.position.x > x[4] {
                            self.stage = Stage::Crossing;
                        } else if self.position.x < x[4] {
                            self.stage = Stage::Crossed;
                        }
                    },
                    _ => (),
                }
            },
            Direction::Right => {
                match self.itineraire {
                    Itineraire::Left => {
                        if self.position.x > x[2] && self.position.y > y[4] {
                            self.stage = Stage::Crossing;
                        } else if self.position.y < y[4] {
                            self.stage = Stage::Crossed;
                        }
                    },
                    Itineraire::Straight => {
                        if self.position.x > x[2] && self.position.x < x[len - 4] {
                            self.stage = Stage::Crossing;
                        } else if self.position.x > x[len - 4] {
                            self.stage = Stage::Crossed;
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    }

    pub fn spawn(&mut self, direction: Direction) {
        let len = self.settings.vertical_key_points.len();
        let get_position = |vp_idx, hp_idx| {
            Point::new(
                self.settings.vertical_key_points[vp_idx],
                self.settings.horizontal_key_points[hp_idx],
            )
        };

        match direction {
            Direction::Up => {
                self.position = match self.itineraire {
                    Itineraire::Left => get_position(9, len - 1),
                    Itineraire::Straight => get_position(11, len - 1),
                    Itineraire::Right => get_position(13, len - 1),
                };
            }
            Direction::Down => {
                self.position = match self.itineraire {
                    Itineraire::Right => get_position(3, 0),
                    Itineraire::Straight => get_position(5, 0),
                    Itineraire::Left => get_position(7, 0),
                };
            }
            Direction::Left => {
                self.position = match self.itineraire {
                    Itineraire::Right => get_position(len - 1, 3),
                    Itineraire::Straight => get_position(len - 1, 5),
                    Itineraire::Left => get_position(len - 1, 7),
                };
            }
            Direction::Right => {
                self.position = match self.itineraire {
                    Itineraire::Left => get_position(0, 9),
                    Itineraire::Straight => get_position(0, 11),
                    Itineraire::Right => get_position(0, 13),
                };
            }
            _ => (),
        }
    }

    fn move_in_direction(&mut self) {
        if !self.is_changed_direction {
            match self.route {
                Direction::Up | Direction::Down => {
                    self.accumulated_y += self.direction * self.velocity;
                    if self.accumulated_y.abs() >= 1.0 {
                        let integer_part = self.accumulated_y.trunc() as i32;
                        self.position.y += integer_part;
                        self.accumulated_y -= integer_part as f32;
                    }
                }
                Direction::Left | Direction::None | Direction::Right => {
                    self.accumulated_x += self.direction * self.velocity;
                    if self.accumulated_x.abs() >= 1.0 {
                        let integer_part = self.accumulated_x.trunc() as i32;
                        self.position.x += integer_part;
                        self.accumulated_x -= integer_part as f32;
                    }
                }
            };
        } else {
            match self.route {
                Direction::Up | Direction::Down => {
                    let d = if self.itineraire == Itineraire::Left {
                        self.direction
                    } else {
                        -self.direction
                    };
                    self.accumulated_x += d * self.velocity;
                    if self.accumulated_x.abs() >= 1.0 {
                        let integer_part = self.accumulated_x.trunc() as i32;
                        self.position.x += integer_part;
                        self.accumulated_x -= integer_part as f32;
                    }
                }
                Direction::Left | Direction::None | Direction::Right => {
                    let d = if self.itineraire == Itineraire::Right {
                        self.direction
                    } else {
                        -self.direction
                    };
                    self.accumulated_y += d * self.velocity;
                    if self.accumulated_y.abs() >= 1.0 {
                        let integer_part = self.accumulated_y.trunc() as i32;
                        self.position.y += integer_part;
                        self.accumulated_y -= integer_part as f32;
                    }
                }
            };
        };
    }

    pub fn move_forward(&mut self) {
        if self.is_stopped {
            return;
        }
        self.set_stage();

        // Previous position before moving
        let prev_position = self.position;

        // Move in the current direction
        self.move_in_direction();

        // Calculate distance traveled
        let distance = self.distance_to(prev_position);
        self.distance_traveled += distance;

        // Calculate time increment based on distance and velocity
        let time_increment = distance / self.velocity as f64;
        self.time += time_increment;
        

        match self.route {
            Direction::Up => {
                if (prev_position.y > self.settings.horizontal_key_points[13]
                    && self.position.y <= self.settings.horizontal_key_points[13]
                    && self.itineraire == Itineraire::Right)
                    || (prev_position.y > self.settings.horizontal_key_points[7]
                        && self.position.y <= self.settings.horizontal_key_points[7]
                        && self.itineraire == Itineraire::Left)
                {
                    self.is_changed_direction = true;
                }
            }
            Direction::Down => {
                if (prev_position.y < self.settings.horizontal_key_points[3]
                    && self.position.y >= self.settings.horizontal_key_points[3]
                    && self.itineraire == Itineraire::Right)
                    || (prev_position.y < self.settings.horizontal_key_points[9]
                        && self.position.y >= self.settings.horizontal_key_points[9]
                        && self.itineraire == Itineraire::Left)
                {
                    self.is_changed_direction = true;
                }
            }
            Direction::Left => {
                if (prev_position.x > self.settings.vertical_key_points[13]
                    && self.position.x <= self.settings.vertical_key_points[13]
                    && self.itineraire == Itineraire::Right)
                    || (prev_position.x > self.settings.vertical_key_points[7]
                        && self.position.x <= self.settings.vertical_key_points[7]
                        && self.itineraire == Itineraire::Left)
                {
                    self.is_changed_direction = true;
                }
            }
            Direction::Right => {
                if (prev_position.x < self.settings.vertical_key_points[9]
                    && self.position.x >= self.settings.vertical_key_points[9]
                    && self.itineraire == Itineraire::Left)
                    || (prev_position.x < self.settings.vertical_key_points[3]
                        && self.position.x >= self.settings.vertical_key_points[3]
                        && self.itineraire == Itineraire::Right)
                {
                    self.is_changed_direction = true;
                }
            }
            _ => (),
        }
    }
}

// Yellow
/*
    route: UP => destination: TurnLeft
    route: Down => destination: TurnRight
    route: Left => destination: TurnDown
    route: Right => destination: TurnUp
*/

// Blue: still forward
/*
    route: UP => destination: GoUP
    route: Down => destination: GoDown
    route: Left => destination: goLeft
    route: Right => destination: GoRight
*/

// GREEN
/*
    route: Up => destination: TurnRight
    route: Down => destination: GoLeft
    route: Left => destination: goUp
    route: Right => destination: GoRight
*/

/*
 pub fn s(&mut self) {
        match self.lane {
            Cross::First => {
                match self.color {
                    Color::BLUE => {
                        if self.position == self.settings.dis_vehicle_fourth {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::YELLOW => {
                        if self.position == self.settings.dis_vehicle_third {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::GREEN => {
                        if self.position == self.settings.dis_vehicle_second {
                            self.stage = Stage::Crossing;
                        }
                    },
                    _ => todo!(),

                }
            },
            Cross::Second => {
                match self.color {
                    Color::BLUE | Color::GREEN  => {
                        if self.position == self.settings.dis_vehicle_third {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::YELLOW => {
                        if self.position == self.settings.dis_vehicle_first {
                            self.stage = Stage::Crossing;
                        }
                    },
                    _ => todo!(),

                }
            },
            Cross::Third => {
                match self.color {
                    Color::BLUE => {
                        if self.position == self.settings.dis_vehicle_second {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::YELLOW => {
                        if self.position == self.settings.dis_vehicle_fourth {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::GREEN => {
                        if self.position == self.settings.dis_vehicle_first {
                            self.stage = Stage::Crossing;
                        }
                    },
                    _ => todo!(),

                }
            },
            Cross::Fourth => {
                match self.color {
                    Color::BLUE => {
                        if self.position == self.settings.dis_vehicle_first {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::YELLOW => {
                        if self.position == self.settings.dis_vehicle_second {
                            self.stage = Stage::Crossing;
                        }
                    },
                    Color::GREEN => {
                        if self.position == self.settings.dis_vehicle_third {
                            self.stage = Stage::Crossing;
                        }
                    },
                    _ => todo!(),

                }
            },
        }
    }

*/
