use std::cell::RefCell;

use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};
use smart_road::*;

fn render(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
) -> Result<(), String> {
    let (width, height) = canvas.output_size()?;

    // Treat the center of the screen as the (0, 0) coordinate
    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
    canvas.copy(texture, sprite, screen_rect)?;

    Ok(())
}

pub fn main() {
    let settings = Rc::new(Settings::new(1000, 1000, 30, 1, 100.0));
    let mut statistic: Statistics = Statistics::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("smart road", settings.width as u32, settings.height as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let lanes: Rc<RefCell<Vec<Lane>>> = Rc::new(RefCell::new(vec![
        Lane::new(Cross::First, settings.clone()),
        Lane::new(Cross::Second, settings.clone()),
        Lane::new(Cross::Third, settings.clone()),
        Lane::new(Cross::Fourth, settings.clone()),
    ]));

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/1.jpg").unwrap();
    let (width, height, half_width, half_height) = (
        settings.vertical_key_points[2],
        settings.horizontal_key_points[2],
        settings.width / 2,
        settings.height / 2,
    );
  
    let positions_and_sprite = vec![
        (Point::new(-half_width + width / 2, -half_height + height / 2) , Rect::new(0, height, width as u32, height as u32)),
        (Point::new(half_width - width / 2, -half_height + height / 2) , Rect::new(100, height - 100, width as u32, height as u32)),
        (Point::new(-half_width + width / 2, half_height - height / 2) , Rect::new(width, 100, width as u32, height as u32)),
        (Point::new(half_width - width / 2, half_height - height / 2) , Rect::new(width, height, width as u32, height as u32)),
    ];

    let a: Vec<Texture> = cars_texture(&texture_creator);
    canvas.present();
    let mut event_pump: sdl2::EventPump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(80, 80, 80));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {
                    handle_keyboard_event(&event, &mut lanes.borrow_mut(), settings.clone());
                }
            }
        }

        canvas.clear();
        for (position, sprite) in positions_and_sprite.iter() {
            render(&mut canvas, &texture, *position, *sprite).unwrap();
        }

        // map
        draw_map(&mut canvas, settings.clone());

        {
            let mut lanes_borrowed = lanes.borrow_mut();
            for lane in lanes_borrowed.iter_mut() {
                lane.update(&mut canvas, &a, &mut statistic);
            }
        }

        // the smart road algorithm to avoid collisions
        smart_intersection(&mut lanes.borrow_mut());

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    statistic.display_statistics_window(&mut event_pump);
}
