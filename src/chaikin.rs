use macroquad::shapes::draw_line;
use macroquad::prelude::WHITE;

pub fn chaikin(points: &mut Vec<(f32,f32)>) {
        lines_drawing(points);

        for i in 0..7 {
            
        }
}

pub fn lines_drawing(points:&mut Vec<(f32,f32)>) {
    for( i, (x,y)) in points.iter().enumerate() {
        if i == points.len() - 1 {
            break;
        }
        draw_line(*x, *y, points[i+1].0, points[i+1].1, 1., WHITE);
    }
}

