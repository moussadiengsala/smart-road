use std::rc::Rc;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::settings::Settings;

pub struct Path {
    pub start: Point,
    pub end: Point,
}

impl Path {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Path {
        Path { start: Point::new(start.0, start.1), end: Point::new(end.0, end.1) }
    }
}

pub fn draw_map(canvas: &mut Canvas<Window>, settings: Rc<Settings>) {
    let routes = create_roads(&settings);

    // canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for route in routes {
        for (i, road) in route.iter().enumerate() {
            if i % 3 == 0 {
                canvas.draw_line(road.start, road.end).unwrap();
            } else {
                draw_dashed_line(canvas, road.start, road.end, 10, 10);
            }
            
        }
    }

    // canvas.present();
}

fn create_roads(settings: &Settings) -> Vec<Vec<Path>> {
    let mut routes = Vec::new();

    // First Route
    let mut first_route = Vec::new();
    for i in 1..=7 {
        first_route.push(Path::new(
            (settings.vertical_key_points[i], settings.horizontal_key_points[0]),
            (settings.vertical_key_points[i], settings.horizontal_key_points[1]),
        ));
    }
    routes.push(first_route);

    // Second Route
    let mut second_route = Vec::new();
    for i in 1..=7 {
        second_route.push(Path::new(
            (settings.vertical_key_points[0], settings.horizontal_key_points[i]),
            (settings.vertical_key_points[1], settings.horizontal_key_points[i]),
        ));
    }
    routes.push(second_route);

    // Third Route
    let mut third_route = Vec::new();
    for i in 1..=7 {
        third_route.push(Path::new(
            (settings.vertical_key_points[8], settings.horizontal_key_points[i]),
            (settings.vertical_key_points[7], settings.horizontal_key_points[i]),
        ));
    }
    routes.push(third_route);

    // Fourth Route
    let mut fourth_route = Vec::new();
    for i in 1..=7 {
        fourth_route.push(Path::new(
            (settings.vertical_key_points[i], settings.horizontal_key_points[8]),
            (settings.vertical_key_points[i], settings.horizontal_key_points[7]),
        ));
    }
    routes.push(fourth_route);

    routes
}


fn draw_dashed_line(
    canvas: &mut Canvas<Window>,
    start: Point,
    end: Point,
    dash_length: i32,
    gap_length: i32,
) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let distance = ((dx * dx + dy * dy) as f32).sqrt() as i32;
    let mut drawn_distance = 0;

    while drawn_distance < distance {
        let dash_start_ratio = drawn_distance as f32 / distance as f32;
        let dash_end_ratio = ((drawn_distance + dash_length).min(distance)) as f32 / distance as f32;

        let dash_start = Point::new(
            start.x + (dx as f32 * dash_start_ratio).round() as i32,
            start.y + (dy as f32 * dash_start_ratio).round() as i32,
        );
        let dash_end = Point::new(
            start.x + (dx as f32 * dash_end_ratio).round() as i32,
            start.y + (dy as f32 * dash_end_ratio).round() as i32,
        );

        canvas.draw_line(dash_start, dash_end).unwrap();

        drawn_distance += dash_length + gap_length;
    }
}

