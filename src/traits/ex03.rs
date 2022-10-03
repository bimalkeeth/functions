use std::ops::Add;

#[derive(Debug)]
pub struct Point{
    x:f64,
    y:f64
}

impl Add for Point{
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point{
         x:self.x+other.x,
         y:self.y+other.y
        }
    }
}