use macroquad::{miniquad::window::request_quit, prelude::*};
mod chaikin;

use chaikin::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut temp_points: Vec<(f32, f32)> = Vec::new();
    let mut click_enter = false;
    let mut error_points = false;    
    let mut index: i32 = 0;
    let mut timer: i32 = 0;

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
            if points.len() < 3 {
               error_points = true;
            } else {
                click_enter = true;
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            request_quit();
        }

        if error_points == true {
            draw_text("Please insert more than two points to see the results !", 20.,20., 20., RED);
        }

        if click_enter == true && points.len() > 2 {
            error_points = false;
            if index > 6 {
                index = 0;
                continue;
            }

            if timer >= 100 {
                temp_points = chaikin(&mut points, index);
                index += 1;
                timer = 0;
            }
        }

        for (i, (x, y)) in temp_points.iter().enumerate() {
            if temp_points.len() - 1 == i {
                break;
            }

            draw_line(
                *x,
                *y,
                temp_points[i + 1].0,
                temp_points[i + 1].1,
                2.,
                WHITE,
            );
        }
        timer += 1;

        next_frame().await
    }
}
