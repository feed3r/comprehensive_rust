// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    
    pub fn new(x: i32, y: i32) -> Self {
        Point{ x, y }
    }
    
    pub fn magnitude(&self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }
    
    pub fn dist(self, other: Point) -> f64 {
        // the following required &self as first parameter
        // f64::sqrt(f64::from(i32::pow(other.x - self.x, 2) + i32::pow(other.y - self.y, 2)))
        (self - other).magnitude()
    }
}

//Implement add and sub for Point class
impl std::ops::Add for Point {
    type Output = Self;
    
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


pub struct Polygon {
    points: Vec<Point>
}

impl Polygon {
    fn new() -> Self {
        Polygon{points: Vec::new()}
    } 
    
    fn add_point(&mut self, p: Point) {
        self.points.push(p)
    }
    
    pub fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).copied()
    }

    pub fn length(&self) -> f64 {
        let mut length = 0.0;
        if self.points.is_empty() {
            return length;
        }

        let mut last_point = self.points[0];

        for point in &self.points[1..] {
            length += last_point.dist(*point);
            last_point = *point;
        }
        length += last_point.dist(self.points[0]);
        return length
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }
}

pub struct Circle {
    center: Point,
    r: i32
}

impl Circle {
    // constructor
    pub fn new(center: Point, r: i32) -> Self {
        Circle { center: center, r: r }
    }

    pub fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * f64::from(self.r)
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl From<Polygon> for Shape {
    fn from(polygon : Polygon) -> Self {
        Shape::Polygon(polygon)
    }
}


impl From<Circle> for Shape {
    fn from(circle : Circle) -> Self {
        Shape::Circle(circle)
    }
}

impl Shape {
    pub fn circumference(&self) -> f64 {
        match self {
            Shape::Polygon(poly) => poly.length(),
            Shape::Circle(circle) => circle.circumference()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }
    
    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }
    
    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }
    
    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }
    
    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);
        
        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }
    
    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);
        
        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        
        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }
    
    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
        Shape::from(poly),
        Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
        .iter()
        .map(Shape::circumference)
        .map(round_two_digits)
        .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}

#[allow(dead_code)]
fn main() {}