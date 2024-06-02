
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

        // map
        draw_map(&mut canvas, settings.clone());

        {
            let mut lanes_borrowed = lanes.borrow_mut();
            for lane in lanes_borrowed.iter_mut() {
                lane.update(&mut canvas);
            }
        }

        smart_intersection(&mut lanes.borrow_mut());

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
