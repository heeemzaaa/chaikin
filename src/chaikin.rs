use macroquad::shapes::draw_line;
use macroquad::prelude::*;

pub fn chaikin(points: &mut Vec<(f32,f32)>, iterations: i32) -> Vec<(f32,f32)> {
    let mut temp_points: Vec<(f32,f32)> = points.clone();

    for _ in 0..iterations {
        let mut new_points = Vec::new();
        new_points.push(temp_points[0]);

        for i in 0..temp_points.len() - 1 {
            let p0 = temp_points[i];
            let p1 = temp_points[i + 1];

            let q = (0.75 * p0.0 + 0.25 * p1.0, 0.75 * p0.1 + 0.25 * p1.1);
            let r = (0.25 * p0.0 + 0.75 * p1.0, 0.25 * p0.1 + 0.75 * p1.1);

            new_points.push(q);
            new_points.push(r);
        }

        new_points.push(temp_points[temp_points.len() - 1]);
        temp_points = new_points;
    }
    temp_points
}

