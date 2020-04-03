use std::ops;

/// Defines a point in a grid-like 2D plane.
/// It is essentially a vector in Z^2, and supports common operations
/// like element-wise addition and subtraction, product by a scalar
/// and dot product.
/// Division by a scalar is not supported, as the resulting values
/// may need to be truncated to integers, possibly providing unwanted behaviour.
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// Addition of points
impl ops::Add<Point> for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Subtraction of points
impl ops::Sub<Point> for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Multiplication of a point by a scalar (integer)
impl ops::Mul<i32> for Point {
    type Output = Self;
    fn mul(self, scalar: i32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Dot product
impl ops::Mul<Point> for Point {
    type Output = i32;
    fn mul(self, other: Self) -> i32 {
        self.x * other.x + self.y * other.y
    }
}

// Division of a point by a scalar (integer)
impl ops::Div<i32> for Point {
    type Output = Self;
    fn div(self, scalar: i32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_addition() {
        let pos1 = Point::new(2, 3);
        let pos2 = Point::new(0, 5);
        let pos3 = Point::new(-3, 1);
        assert_eq!(Point::new(2, 8), pos1 + pos2);
        assert_eq!(Point::new(-1, 4), pos1 + pos3);
    }

    #[test]
    fn point_subtraction() {
        let pos1 = Point::new(2, 3);
        let pos2 = Point::new(0, 5);
        let pos3 = Point::new(-3, 1);
        assert_eq!(Point::new(2, -2), pos1 - pos2);
        assert_eq!(Point::new(5, 2), pos1 - pos3);
    }

    #[test]
    fn point_mult_by_scalar() {
        let pos = Point::new(2, 3);
        assert_eq!(Point::new(4, 6), pos * 2);
        assert_eq!(Point::new(-2, -3), pos * -1);
        assert_eq!(Point::new(0, 0), pos * 0);
    }
    #[test]
    fn dot_product() {
        let pos1 = Point::new(2, 3);
        let pos2 = Point::new(0, 5);
        let pos3 = Point::new(-3, 1);
        assert_eq!(15, pos1 * pos2);
        assert_eq!(-3, pos1 * pos3);
        assert_eq!(5, pos2 * pos3);
    }

    #[test]
    fn point_div_by_scalar() {
        let pos = Point::new(2, 3);
        assert_eq!(Point::new(1, 1), pos / 2);  //
        assert_eq!(Point::new(-2, -3), pos / -1);
    }

    #[test]
    #[should_panic]
    fn point_div_by_zero() {
        // Put inside print macro so the compiler doesn't complain about unused expressions
        print!("{:?}", Point::new(2, 3) / 0);
    }
}
