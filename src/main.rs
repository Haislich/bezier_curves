use raylib::prelude::*;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;
const FPS: u32 = 60;
#[derive(Debug)]
struct Point {
    pub x: i32,
    pub y: i32,
}
fn interpolate(p1: &Point, p2: &Point, t: f32) -> Point {
    let t_sin = (f32::sin(t) + 1.0) / 2.0;
    let bezier_p = Point {
        x: ((1.0 - t_sin) * p1.x as f32) as i32 + (t_sin * p2.x as f32) as i32,
        y: ((1.0 - t_sin) * p1.y as f32) as i32 + (t_sin * p2.y as f32) as i32,
    };
    bezier_p
}
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Bezier Curves")
        .build();
    rl.set_target_fps(FPS);
    let mut t = 0.0;
    let mut points: Vec<Point> = vec![];
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            points.push(Point {
                x: d.get_mouse_x(),
                y: d.get_mouse_y(),
            })
        }
        for (i, point) in points.iter().enumerate() {
            d.draw_circle(point.x, point.y, 10.0, Color::BLACK);
            if (i + 1) < points.len() {
                let bezier_p = interpolate(&points[i], &points[i + 1], t);
                d.draw_circle(bezier_p.x, bezier_p.y, 10.0, Color::BLUE);
            }
        }
        t += 0.1
    }
}
