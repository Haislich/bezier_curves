use std::ops::Add;
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
