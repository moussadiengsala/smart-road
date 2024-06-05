use std::rc::Rc;

use sdl2::{pixels::Color, rect::Point, render::Canvas, video::Window};

use crate::settings::Settings;

pub struct Path {
    pub start: Point,
    pub end: Point,
}

impl Path {
    pub fn new(start: (i32, i32), end: (i32, i32)) -> Path {
        Path {
            start: Point::new(start.0, start.1),
            end: Point::new(end.0, end.1),
        }
    }
}



pub fn draw_map(canvas: &mut Canvas<Window>, settings: Rc<Settings>) {
    let (routes, xy) = create_roads(&settings);

    // canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    
    for (i, route) in routes.iter().enumerate() {
        let mut y = 0;
        for (j, road) in route.iter().enumerate() {
            if j % 3 == 0 {
                canvas.draw_line(road.start, road.end).unwrap();
            } else {
                draw_dashed_line(canvas, road.start, road.end, 10, 8);
            }

            // draw the arrow
            let k = j * 2 + 1; 
            if j > 0 && j < routes.len() && i%2 == 0 || i%2 != 0&& j > routes.len() - 1 && j < 2 * routes.len() - 1 {
                let direction = &get_direction(i)[y];
                y += 1;
                if i % 3 == 0 {
                    draw_arrow(canvas, Point::new(settings.vertical_key_points[k], xy[i]), direction, 12.0);
                } else {
                    draw_arrow(canvas, Point::new(xy[i], settings.horizontal_key_points[k]), direction, 12.0);
                }
            }
        }
    }

    // canvas.present();
}

fn create_roads(settings: &Settings) -> (Vec<Vec<Path>>, Vec<i32>) {
    let mut routes = Vec::new();
    let len = settings.vertical_key_points.len();

    let xy = vec![
        settings.horizontal_key_points[1], 

        settings.vertical_key_points[1], 
        settings.vertical_key_points[len - 2], 

        settings.horizontal_key_points[len - 2],
    ];

    // First Route
    let mut first_route = Vec::new();
    let mut i = 2;
    while i < settings.vertical_key_points.len() {
        first_route.push(Path::new(
            (
                settings.vertical_key_points[i],
                settings.horizontal_key_points[0],
            ),
            (
                settings.vertical_key_points[i],
                settings.horizontal_key_points[2],
            ),
        ));
        i += 2;
    }
    routes.push(first_route);

    // Second Route
    let mut second_route = Vec::new();
    let mut i: usize = 2;
    while i < settings.vertical_key_points.len() {
        second_route.push(Path::new(
            (
                settings.vertical_key_points[0],
                settings.horizontal_key_points[i],
            ),
            (
                settings.vertical_key_points[2],
                settings.horizontal_key_points[i],
            ),
        ));
        i += 2;
    }
    routes.push(second_route);

    // // Third Route
    let mut third_route = Vec::new();
    let mut i = 2;
    while i < len {
        third_route.push(Path::new(
            (
                settings.vertical_key_points[len - 1],
                settings.horizontal_key_points[i],
            ),
            (
                settings.vertical_key_points[len - 3],
                settings.horizontal_key_points[i],
            ),
        ));
        i += 2;
    }
    routes.push(third_route);

    // // Fourth Route
    let mut fourth_route = Vec::new();
    let mut i = 2;
    while i < len {
        fourth_route.push(Path::new(
            (
                settings.vertical_key_points[i],
                settings.horizontal_key_points[len - 1],
            ),
            (
                settings.vertical_key_points[i],
                settings.horizontal_key_points[len - 3],
            ),
        ));
        i += 2;
    }
    routes.push(fourth_route);

    return (routes, xy);
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
        let dash_end_ratio =
            ((drawn_distance + dash_length).min(distance)) as f32 / distance as f32;

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

fn draw_arrow(canvas: &mut Canvas<Window>, end: Point, direction: &str, size: f32) {
    // Calculate the start point based on the direction and size
    let (start, end) = match direction {
        "right" => (Point::new(end.x, end.y + size as i32), Point::new(end.x + 2*size as i32, end.y + size as i32)),
        "left" => (Point::new(end.x + 2 * size as i32, end.y + size as i32), Point::new(end.x, end.y + size as i32)),
        "up" => (Point::new(end.x + size as i32, end.y + 2 * size as i32), Point::new(end.x + size as i32, end.y)),
        "down" => (Point::new(end.x + size as i32, end.y), Point::new(end.x + size as i32, end.y + 2*size as i32)),
        _ => (end, end), // Default to the end point if direction is invalid
    };

    // Draw the main line of the arrow
    draw_dashed_line(canvas, start, end, 2, 3);

    // Calculate the direction vector
    let dx = (end.x - start.x) as f32;
    let dy = (end.y - start.y) as f32;
    let length = (dx * dx + dy * dy).sqrt();
    let unit_dx = dx / length;
    let unit_dy = dy / length;

    // Arrowhead points
    let tip = end;
    let base1 = Point::new(
        tip.x - (unit_dx * size - unit_dy * size / 2.0) as i32,
        tip.y - (unit_dy * size + unit_dx * size / 2.0) as i32,
    );
    let base2 = Point::new(
        tip.x - (unit_dx * size + unit_dy * size / 2.0) as i32,
        tip.y - (unit_dy * size - unit_dx * size / 2.0) as i32,
    );

    // Draw the arrowhead
    canvas.draw_line(tip, base1).unwrap();
    canvas.draw_line(tip, base2).unwrap();
}

fn get_direction(index: usize) -> Vec<String> {
    let directions = vec![
        vec!["left", "down", "right"],
        vec!["up", "right", "down"],
        vec!["up", "left", "down"],
        vec!["left", "up", "right"],
    ];

    directions[index].iter().map(|&s| s.to_string()).collect()
}

