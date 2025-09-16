use macroquad::prelude::*;
mod chaikin;

use chaikin::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut animation = false;

    loop {
        clear_background(BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            points.push((x, y));
        }

        for (x, y) in points.iter() {
            draw_circle_lines(*x, *y, 3., 1., WHITE);
        }

        if is_key_pressed(KeyCode::Enter) {
           animation = true;
        }

        if animation == true {
            chaikin(&mut points);
        }
        

        next_frame().await
    }
}
