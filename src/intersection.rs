use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn draw(canvas: &mut Canvas<Window>) {
    let crossing_color = Color::RGB(45, 45, 50);
    canvas.set_draw_color(crossing_color);
    // Intersection square
    let square = Rect::new(350, 350, 100, 100);
    let _ = canvas.fill_rect(square);

    canvas.set_draw_color(Color::RGB(220, 220, 220));

    // Horizontal crosswalk stripes (top and bottom of intersection)
    for i in 0..5 {
        let stripe_top = Rect::new(355 + i * 18, 345, 12, 10);
        let stripe_bottom = Rect::new(355 + i * 18, 445, 12, 10);
        let _ = canvas.fill_rect(stripe_top);
        let _ = canvas.fill_rect(stripe_bottom);
    }

    // Vertical crosswalk stripes (left and right of intersection)
    for i in 0..5 {
        let stripe_left = Rect::new(345, 355 + i * 18, 10, 12);
        let stripe_right = Rect::new(445, 355 + i * 18, 10, 12);
        let _ = canvas.fill_rect(stripe_left);
        let _ = canvas.fill_rect(stripe_right);
    }
}
