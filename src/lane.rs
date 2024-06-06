
use std::rc::Rc;

use crate::{Itineraire, Route, Statistics};
use crate::settings::Settings;
use sdl2::render::Texture;
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
    Crossed,
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
        let  stop_point = match cross {
            Cross::First => Point::new(0, settings.horizontal_key_points[2]),
            Cross::Second => Point::new(settings.vertical_key_points[2], 0),
            Cross::Third => Point::new(settings.vertical_key_points[settings.vertical_key_points.len() - 2], 0),
            Cross::Fourth => Point::new(0, settings.horizontal_key_points[settings.horizontal_key_points.len() - 2]),
        };

        Lane {
            routes: vec![
                Route::new(Itineraire::Left, cross, stop_point, settings.clone()),
                Route::new(Itineraire::Straight, cross, stop_point, settings.clone()),
                Route::new(Itineraire::Right, cross, stop_point, settings.clone()),
            ],
            cross,
            stage: Stage::Waiting,
            stop_point,
            settings,
        }
    }

    pub fn update(&mut self, canvas: &mut Canvas<Window>, texture: &Vec<Texture>, statistic: &mut Statistics) {
        for i in (0..self.routes.len()).rev() {
            self.routes[i].update(canvas, &texture, statistic);
        }
    }
}