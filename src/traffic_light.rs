use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightState {
    Red,
    Green,
}

pub struct TrafficLight {
    rect: Rect,
    pub state: LightState,
}

impl TrafficLight {
    pub fn new(x: i32, y: i32, w: u32, h: u32, state: LightState) -> Self {
        TrafficLight {
            rect: Rect::new(x, y, w, h),
            state,
        }
    }
    pub fn update(&mut self, is_green: bool) {
        self.state = if is_green {
            LightState::Green
        } else {
            LightState::Red
        };
    }
    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(20, 20, 20));
        let housing = Rect::new(
            self.rect.x() - 2,
            self.rect.y() - 2,
            self.rect.width() + 4,
            self.rect.height() + 4,
        );
        let _ = canvas.fill_rect(housing);
        
        let color = match self.state {
            LightState::Red => Color::RGB(220, 20, 20),
            LightState::Green => Color::RGB(20, 180, 20),
        };

        canvas.set_draw_color(color);
        let _ = canvas.fill_rect(self.rect);
        
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let highlight = Rect::new(
            self.rect.x() + 2,
            self.rect.y() + 2,
            4,
            4,
        );
        let _ = canvas.fill_rect(highlight);
    }
}
