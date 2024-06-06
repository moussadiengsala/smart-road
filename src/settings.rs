use std::path::Path;

use sdl2::{event::Event, image::{self, InitFlag, LoadTexture}, keyboard::Keycode, pixels::Color, rect::Rect, render::{Texture, TextureCreator}, video::WindowContext};

use crate::{Cross, Itineraire, Vehicle};
use sdl2::render::TextureQuery;

#[derive(Debug, Clone)]
pub struct Statistics {
    pub max_vehicles_passed: usize,
    pub max_velocities: Vec<f64>,
    pub min_velocities: Vec<f64>,
    pub time_to_pass: Vec<f64>,
    pub max_velocity: f64,
    pub min_velocity: f64,
    pub max_time_to_pass: f64,
    pub min_time_to_pass: f64,
    pub close_calls: usize,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            max_vehicles_passed: 0,
            max_velocities: Vec::new(),
            min_velocities: Vec::new(),
            time_to_pass: Vec::new(),
            max_velocity: 0.0,
            min_velocity: 0.0,
            max_time_to_pass: 0.0,
            min_time_to_pass: 0.0,
            close_calls: 0,
        }
    }

    pub fn retrieve(&mut self, vehicle: &Vehicle) {
        self.max_vehicles_passed += 1;
        // Update max and min velocities
        self.max_velocities.push(vehicle.max_vilosity as f64);
        self.min_velocities.push(vehicle.min_vilosity as f64);

        // Update max and min times to pass the intersection
        self.time_to_pass.push(vehicle.time);
        self.max_time_to_pass = self.time_to_pass.iter().cloned().fold(f64::MIN, f64::max);
        self.min_time_to_pass = self.time_to_pass.iter().cloned().fold(f64::MAX, f64::min);

        self.min_velocity = self.min_velocities.iter().cloned().fold(f64::MIN, f64::max);
        self.max_velocity = self.max_velocities.iter().cloned().fold(f64::MAX, f64::min);
    }

    pub fn display_statistics_window(&self, event_pump: &mut sdl2::EventPump) {
        const WIDTH: u32 = 600;
        const HEIGHT: u32 = 400;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem
            .window("Simulation Statistics", WIDTH, HEIGHT)
            .position_centered()
            .build()
            .unwrap();
    
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
    
        let ttf_context = sdl2::ttf::init().unwrap();
        let mut font = ttf_context.load_font("assets/Roboto-Thin.ttf", 30).unwrap();
        font.set_style(sdl2::ttf::FontStyle::BOLD);
    
        let stats_text = vec![
            format!("Statistics"),
            format!("Max vehicles : {}",self.max_vehicles_passed),
            format!("Max velocity : {:.2} m/s", self.max_velocity),
            format!("Min velocity : {:.2} m/s", self.min_velocity),
            format!("Max time to pass : {:.2} s",self.max_time_to_pass),
            format!("Min time to pass : {:.2} s",self.min_time_to_pass),
            format!("Collisions : {}", 0),
            format!("Close calls : {}",self.close_calls)
        ];

        for (i, stat_text) in stats_text.iter().enumerate() {
            let surface = font.render(&stat_text)
                .blended(Color::RGB(255, 255, 255))
                .unwrap();
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    
            let TextureQuery { width, height, .. } = texture.query();
            
            let (x, y) = if i == 0 {
                ((WIDTH - width) as i32 / 2, 24 + 30 * i as i32)
            } else {
                (30, 24 + 40 * i as i32)
            };

            let r = Rect::new(x, y, width, height);
    
            canvas.copy(&texture, None, r).unwrap();
        }

        canvas.present();
    
        // let mut event_pump = sdl_context.event_pump().unwrap();
        'stats_window: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'stats_window,
                    _ => {}
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub width: i32,
    pub height: i32,
    pub vehicle: i32,
    pub gap: i32,
    pub safety_distance: f64,
    pub offset_road: i32,

    pub vertical_key_points: Vec<i32>,
    pub horizontal_key_points: Vec<i32>,
}

impl Settings {
    pub fn new(width: i32, height: i32, vehicle: i32, gap: i32, safety_distance: f64) -> Settings {
        let half_width = width / 2;
        let half_height = height / 2;
        let vehicle_width = 2 * vehicle;
        let offset_road = gap + vehicle_width;
        let offset_road_s = gap + vehicle;

        let get_map_key_points = |dim: i32, half_dim: i32| -> Vec<i32> {
            return vec![
                -0,
                half_dim - 3 * offset_road - offset_road / 2 - offset_road_s / 2, //
                half_dim - 3 * offset_road,
                half_dim - 2 * offset_road - offset_road / 2 - offset_road_s / 2, //
                half_dim - 2 * offset_road,
                half_dim - offset_road - offset_road / 2 - offset_road_s / 2, //
                half_dim - offset_road,
                half_dim - offset_road / 2 - offset_road_s / 2, //
                half_dim,
                half_dim + offset_road / 2 - offset_road_s / 2, //
                half_dim + offset_road,
                half_dim + offset_road + offset_road / 2 - offset_road_s / 2, //
                half_dim + 2 * offset_road,
                half_dim + 2 * offset_road + offset_road / 2 - offset_road_s / 2, //
                half_dim + 3 * offset_road,
                half_dim + 3 * offset_road + offset_road / 2 - offset_road_s / 2, //
                dim,
            ];
        };

        Self {
            width,
            height,
            vehicle,
            gap,
            safety_distance,
            offset_road,

            vertical_key_points: get_map_key_points(width, half_width),
            horizontal_key_points: get_map_key_points(height, half_height)
        }
    }
}


pub fn cars_texture<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Vec<Texture<'a>> {
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let texture_names = vec![
        "BlackOut.png", "WhiteStrip.png", "BlueStrip.png", "GreenStrip.png", "RedStrip.png", "PinkStrip.png",
    ];

    let mut cars_textures = Vec::new();

    for name in texture_names {
        let texture_path = Path::new("assets").join(name);
        match texture_creator.load_texture(texture_path) {
            Ok(texture) => cars_textures.push(texture),
            Err(e) => {
                eprintln!("Failed to load texture {}: {}", name, e);
            }
        }
    };
    cars_textures
}

pub struct BLOCK<'a> {
    pub lane: (Cross, Itineraire),
    pub intersections: &'a [(Cross, Itineraire)],
}

pub(crate) const BLOCKS: &[&BLOCK] = &[

    // North
    &BLOCK{
        lane: (Cross::First, Itineraire::Left),
        intersections: &[
            (Cross::First, Itineraire::Left),
            (Cross::Fourth, Itineraire::Straight),
            (Cross::Fourth, Itineraire::Left),
            (Cross::Third, Itineraire::Left),
            (Cross::Third, Itineraire::Straight),
            (Cross::Second, Itineraire::Left),
        ],
    },
    &BLOCK{
        lane: (Cross::First, Itineraire::Straight),
        intersections: &[
            (Cross::First, Itineraire::Straight),
            (Cross::Fourth, Itineraire::Left),
            (Cross::Third, Itineraire::Straight),
            (Cross::Second, Itineraire::Straight),
            (Cross::Second, Itineraire::Left),
        ],
    },
    // Right
    &BLOCK{
        lane: (Cross::Second, Itineraire::Left),
        intersections: &[
                (Cross::Second, Itineraire::Left),
                (Cross::First, Itineraire::Straight),
                (Cross::First, Itineraire::Left),
                (Cross::Fourth, Itineraire::Left),
                (Cross::Third, Itineraire::Straight),
                (Cross::Third, Itineraire::Left),
            ],
    },
    &BLOCK{
        lane: (Cross::Second, Itineraire::Straight),
        intersections: &[
            (Cross::First, Itineraire::Straight),
            (Cross::Fourth, Itineraire::Straight),
            (Cross::Fourth, Itineraire::Left),
            (Cross::Third, Itineraire::Left),
            (Cross::Second, Itineraire::Straight)
        ],
    },

    // Left
    &BLOCK{
        lane: (Cross::Third, Itineraire::Left),
        intersections: &[
            (Cross::Third, Itineraire::Left),
            (Cross::First, Itineraire::Left),
            (Cross::Fourth, Itineraire::Straight),
            (Cross::Fourth, Itineraire::Left),
            (Cross::Second, Itineraire::Straight),
            (Cross::Second, Itineraire::Left),
        ],
    },
    &BLOCK{
        lane: (Cross::Third, Itineraire::Straight),
        intersections: &[
            (Cross::Third, Itineraire::Straight),
            (Cross::First, Itineraire::Left),
            (Cross::First, Itineraire::Straight),
            (Cross::Fourth, Itineraire::Straight),
            (Cross::Second, Itineraire::Left),
        ],
    },

    // South
    &BLOCK{
        lane: (Cross::Fourth, Itineraire::Left),
        intersections: &[
            (Cross::Fourth, Itineraire::Left),
            (Cross::First, Itineraire::Left),
            (Cross::First, Itineraire::Straight),
            (Cross::Third, Itineraire::Left),
            (Cross::Second, Itineraire::Straight),
            (Cross::Second, Itineraire::Left),
        ],
    },
    &BLOCK{
        lane: (Cross::Fourth, Itineraire::Straight),
        intersections: &[
            (Cross::Fourth, Itineraire::Straight),
            (Cross::First, Itineraire::Left),
            (Cross::Third, Itineraire::Straight),
            (Cross::Third, Itineraire::Left),
            (Cross::Second, Itineraire::Straight),
        ],
    },

  
];

/*
    // block 1

    North - Straight
    North - Left
    South - Left
    East(left) - Straight
    East(left) - Left
    West(right) - Left

    // block 2

    North - Straight
    South - Straight
    South - Left
    East(left) - Left
    West(right) - Straight
----------------------------------------------------------------

    // block 3

    North - Left
    South - Straight
    South - Left
    East - Left
    West(right) - Straight
    West(right) - Left

    // block 4

    North - Straight
    North - Left
    South - Straight
    East - Straight
    West(right) - Left
---------------------------------------------------------------

    // block 5

    North - Straight
    North - Left
    South - Left
    East(left) - Left
    West(right) - Straight
    West(right) - Left

    // block 6

    North - Left
    South - Straight
    East(left) - Straight
    East(left) - Left
    West(right) - Straight

--------------------------------------------------------------------

    // block 7

    North - Left
    South - Straight
    South - Left
    East(left) - Straight
    East(left) - Left
    West(right) - Left

    // block 8

    North - Straight
    South - Left
    East(left) - Straight
    West(right) - Straight
    West(right) - Left


*/
