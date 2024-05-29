use std::rc::Rc;

use rand::Rng;
use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::Canvas,
    video::Window,
};

use crate::{Direction, Itineraire, Settings};

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub position: Point,
    pub route: Direction,
    pub itineraire: Itineraire,
    pub velocity: i32,
    pub is_changed_direction: bool,
    pub is_stopped: bool,
    settings: Rc<Settings>,
}

impl Vehicle {
    pub fn new(route: Direction, itineraire: Itineraire, settings: Rc<Settings>) -> Self {
        Self {
            position: Point::new(0, 0),
            route,
            itineraire,
            velocity: 1,
            is_changed_direction: false,
            is_stopped: false,
            settings,
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

    pub fn spawn(&mut self, direction: Direction) {
        let len = self.settings.vertical_key_points.len();
        let get_position = |vp_idx, hp_idx| Point::new(self.settings.vertical_key_points[vp_idx], self.settings.horizontal_key_points[hp_idx]);
    
        match direction {
            Direction::Up => {
                self.position = match self.itineraire {
                    Itineraire::Right => get_position(9, len - 1),
                    Itineraire::Straight => get_position(11, len - 1),
                    Itineraire::Left => get_position(13, len - 1),
                };
            }
            Direction::Down => {
                self.position = match self.itineraire {
                    Itineraire::Left => get_position(3, 0),
                    Itineraire::Straight => get_position(5, 0),
                    Itineraire::Right => get_position(7, 0),
                };
            }
            Direction::Left => {
                self.position = match self.itineraire {
                    Itineraire::Left => get_position(len - 1, 3),
                    Itineraire::Straight => get_position(len - 1, 5),
                    Itineraire::Right => get_position(len - 1, 7),
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
    

    pub fn move_forward(&mut self) {
        if self.is_stopped {
            return;
        };

        match self.route {
            Direction::Up => {
                if !self.is_changed_direction {
                    self.position.y -= self.velocity
                }
            }
            Direction::Down => {
                if !self.is_changed_direction {
                    self.position.y += self.velocity
                }
            }
            Direction::Left => {
                if !self.is_changed_direction {
                    self.position.x -= self.velocity
                }
            }
            Direction::Right => {
                if !self.is_changed_direction {
                    self.position.x += self.velocity
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

    pub fn move_forward(&mut self) {
        if self.is_stopped {
            return;
        };

        match self.route {
            Direction::Up => {
                if !self.is_changed_direction {
                    self.position.y -= self.velocity
                }
                else {
                    let d = if self.destination == Direction::Left {
                        -1
                    } else {
                        1
                    };
                    self.position.x += d * self.velocity;
                };

                if (self.position.y == self.settings.change_direction_1.y)
                    && (self.destination == Direction::Left)
                    || self.destination == Direction::Right
                        && (self.position.y == self.settings.change_direction_2.y)
                {
                    self.is_changed_direction = true;
                };
            }
            Direction::Down => {
                if !self.is_changed_direction {
                    self.position.y += self.velocity
                } else {
                    let d = if self.destination == Direction::Left {
                        -1
                    } else {
                        1
                    };
                    self.position.x += d * self.velocity;
                };

                if (self.position.y == self.settings.change_direction_2.y)
                    && (self.destination == Direction::Right)
                    || (self.position.y == self.settings.change_direction_1.y)
                        && self.destination == Direction::Left
                {
                    self.is_changed_direction = true;
                };
            }
            Direction::Left => {
                if !self.is_changed_direction {
                    self.position.x -= self.velocity
                } else {
                    let d = if self.destination == Direction::Down {
                        1
                    } else {
                        -1
                    };
                    self.position.y += d * self.velocity;
                };

                if self.destination == Direction::Down
                    && self.position.x == self.settings.change_direction_1.x
                    || self.destination == Direction::Up
                        && self.position.x == self.settings.change_direction_2.x
                {
                    self.is_changed_direction = true;
                };
            }
            Direction::Right => {
                if !self.is_changed_direction {
                    self.position.x += self.velocity
                } else {
                    self.position.y -= self.velocity;
                };

                if self.destination == Direction::Up
                    && self.position.x == self.settings.change_direction_2.x
                {
                    self.is_changed_direction = true;
                };
            }
            _ => (),
        }
    }
*/
