use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::traffic_light::LightState;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Turn {
    Straight,
    Right,
    Left,
}
#[derive(Clone)]
pub struct Vehicle {
    pub rect: Rect,
    pub direction: Direction,
    velocity: i32,
    color: Color,
    turn: Turn,
    pub has_turned: bool,
    pub in_intersection: bool,
}

impl Vehicle {
    pub fn new(direction: Direction) -> Self {
        let velocity = 5;
        let rect = match direction {
            Direction::North => Rect::new(415, 800, 20, 40),
            Direction::South => Rect::new(365, -40, 20, 40),
            Direction::East => Rect::new(-40, 415, 40, 20),
            Direction::West => Rect::new(800, 365, 40, 20),
        };
        let mut rng = rand::rng();
        let (turn, color) = match rng.random_range(0..3) {
            0 => (Turn::Straight, Color::RGB(180, 180, 200)),  // Silver/gray for straight
            1 => (Turn::Right, Color::RGB(200, 60, 60)),       // Deep red for right turn
            2 => (Turn::Left, Color::RGB(60, 100, 180)),       // Deep blue for left turn
            _ => unreachable!(),
        };
        Vehicle {
            rect,
            direction,
            velocity,
            color,
            turn,
            has_turned: false,
            in_intersection: false,
        }
    }

    pub fn update(&mut self, light_state: LightState) {
        if !self.in_intersection && !self.has_turned && self.should_stop_at_light(light_state) {
            return;
        }
        if !self.has_turned && !self.in_intersection {
            // Detect entry into intersection bounds (e.g., 350 <= x/y <= 450)
            let in_intersection = match self.direction {
                Direction::North => self.rect.y() <= 450,
                Direction::South => self.rect.y() + self.rect.height() as i32 >= 350,
                Direction::East => self.rect.x() + self.rect.width() as i32 >= 350,
                Direction::West => self.rect.x() <= 450,
            };

            if in_intersection {
                self.in_intersection = true;
            }
        }
        if !self.has_turned {
            let turn_point = match (self.direction, self.turn) {
                // Left turns
                (Direction::West, Turn::Left) => (355, 375),
                (Direction::North, Turn::Left) => (425, 355),
                (Direction::South, Turn::Left) => (375, 445),
                (Direction::East, Turn::Left) => (445, 425),

                // Right turns
                (Direction::West, Turn::Right) => (405, 375),
                (Direction::North, Turn::Right) => (420, 405),
                (Direction::South, Turn::Right) => (365, 395),
                (Direction::East, Turn::Right) => (395, 415),

                // Straight - use center of intersection
                (Direction::West, Turn::Straight) => (400, 400),
                (Direction::East, Turn::Straight) => (400, 400),
                (Direction::North, Turn::Straight) => (400, 400),
                (Direction::South, Turn::Straight) => (400, 400),
            };

            let point_rect = Rect::new(turn_point.0, turn_point.1, 1, 1);
            if self.rect.has_intersection(point_rect) {
                self.apply_turn();
                self.has_turned = true;
            }
        }

        // Continue movement
        match self.direction {
            Direction::North => self.rect.offset(0, -self.velocity),
            Direction::South => self.rect.offset(0, self.velocity),
            Direction::East => self.rect.offset(self.velocity, 0),
            Direction::West => self.rect.offset(-self.velocity, 0),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        let outline = Rect::new(
            self.rect.x() - 1,
            self.rect.y() - 1,
            self.rect.width() + 2,
            self.rect.height() + 2,
        );
        let _ = canvas.fill_rect(outline);
        
        canvas.set_draw_color(self.color);
        let _ = canvas.fill_rect(self.rect);
        
        let mut window_color = self.color;
        window_color.r = (window_color.r as f32 * 1.3).min(255.0) as u8;
        window_color.g = (window_color.g as f32 * 1.3).min(255.0) as u8;
        window_color.b = (window_color.b as f32 * 1.3).min(255.0) as u8;
        
        canvas.set_draw_color(window_color);
        let window = match self.direction {
            Direction::North => Rect::new(
                self.rect.x() + 2,
                self.rect.y() + 2,
                self.rect.width() - 4,
                8,
            ),
            Direction::South => Rect::new(
                self.rect.x() + 2,
                self.rect.y() + 30,
                self.rect.width() - 4,
                8,
            ),
            Direction::East => Rect::new(
                self.rect.x() + 30,
                self.rect.y() + 2,
                8,
                self.rect.height() - 4,
            ),
            Direction::West => Rect::new(
                self.rect.x() + 2,
                self.rect.y() + 2,
                8,
                self.rect.height() - 4,
            ),
        };
        let _ = canvas.fill_rect(window);
    }

    fn should_stop_at_light(&mut self, light_state: LightState) -> bool {
        if self.in_intersection || self.has_turned {
            return false;
        }
        if light_state == LightState::Green {
            return false;
        }

        // Calculate distance to stop line
        let val = match self.direction {
            Direction::North => self.rect.y() > 350 && self.rect.y() <= 450,
            Direction::South => {
                (self.rect.y() + self.rect.height() as i32) < 450
                    && (self.rect.y() + self.rect.height() as i32) >= 350
            }
            Direction::East => {
                (self.rect.x() + self.rect.width() as i32) < 450
                    && (self.rect.x() + self.rect.width() as i32) >= 350
            }
            Direction::West => self.rect.x() > 350 && self.rect.x() <= 450,
        };
        val
    }
    pub fn apply_turn(&mut self) {
        self.direction = match (self.direction, self.turn) {
            // Go straight: no change
            (dir, Turn::Straight) => dir,

            // Right turns
            (Direction::North, Turn::Right) => Direction::East,
            (Direction::East, Turn::Right) => Direction::South,
            (Direction::South, Turn::Right) => Direction::West,
            (Direction::West, Turn::Right) => Direction::North,

            // Left turns
            (Direction::North, Turn::Left) => Direction::West,
            (Direction::West, Turn::Left) => Direction::South,
            (Direction::South, Turn::Left) => Direction::East,
            (Direction::East, Turn::Left) => Direction::North,
        };
        // Swap width/height if we flipped between vertical<->horizontal
        let (w, h) = (self.rect.width(), self.rect.height());
        if (w > h && matches!(self.direction, Direction::North | Direction::South))
            || (h > w && matches!(self.direction, Direction::East | Direction::West))
        {
            let center = self.rect.center();
            // build a new Rect centered the same place
            let new_w = h;
            let new_h = w;
            self.rect = Rect::new(
                center.x - (new_w as i32) / 2,
                center.y - (new_h as i32) / 2,
                new_w,
                new_h,
            );
        }
    }
}
