use sdl2::rect::Point;

use crate::{Cross, Itineraire};

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

    pub change_direction_1: Point,
    pub change_direction_2: Point,

    pub stop_point_first: Point,
    pub stop_point_second: Point,
    pub stop_point_third: Point,
    pub stop_point_fourth: Point,

    pub dis_vehicle_first: Point,
    pub dis_vehicle_second: Point,
    pub dis_vehicle_third: Point,
    pub dis_vehicle_fourth: Point,
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
            horizontal_key_points: get_map_key_points(height, half_height),

            change_direction_1: Point::new(
                half_width - 3 * offset_road_s / 2,
                half_height - 3 * offset_road_s / 2,
            ),
            change_direction_2: Point::new(
                half_width + (offset_road_s / 2),
                half_height + offset_road_s / 2,
            ),

            stop_point_first: Point::new(
                half_width - 3 * offset_road_s / 2,
                half_height - offset_road - vehicle,
            ),
            stop_point_second: Point::new(
                half_width - offset_road - vehicle,
                half_height + offset_road_s / 2,
            ),
            stop_point_third: Point::new(
                half_width + offset_road,
                half_height - 3 * offset_road_s / 2,
            ),
            stop_point_fourth: Point::new(
                half_width + (offset_road_s / 2),
                half_height + offset_road,
            ),

            dis_vehicle_first: Point::new(
                half_width + (offset_road_s / 2),
                half_height - offset_road - vehicle,
            ),
            dis_vehicle_second: Point::new(
                half_width - offset_road - vehicle,
                half_height - 3 * offset_road_s / 2,
            ),
            dis_vehicle_third: Point::new(
                half_width + offset_road,
                half_height + offset_road_s / 2,
            ),
            dis_vehicle_fourth: Point::new(
                half_width - 3 * offset_road_s / 2,
                half_height + offset_road,
            ),
        }
    }
}

pub struct BLOCK<'a> {
    pub lane: (Cross, Itineraire),
    pub intersections: &'a [(Cross, Itineraire)],
}

pub(crate) const BLOCKS: &[&BLOCK] = &[
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
