use crate::point::Point;
use std::cmp;

pub enum DistanceAlg {
    Euclidean,
    Manhattan,
    Chebyshev,
    Octile,
}

impl DistanceAlg {
    pub fn distance(self, a: &Point, b: &Point) -> f32 {
        match self {
            DistanceAlg::Euclidean => euclidean_distance(a, b),
            DistanceAlg::Manhattan => manhattan_distance(a, b),
            DistanceAlg::Chebyshev => chebyshev_distance(a, b),
            DistanceAlg::Octile => octile_distance(a, b),
        }
    }
}

fn euclidean_distance(a: &Point, b: &Point) -> f32 {
    let dx = (a.x - b.x) as f32;
    let dy = (a.y - b.y) as f32;
    f32::sqrt((dx * dx) + (dy * dy))
}

fn manhattan_distance(a: &Point, b: &Point) -> f32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs()) as f32
}

fn chebyshev_distance(a: &Point, b: &Point) -> f32 {
    cmp::max((a.x - b.x).abs(), (a.y - b.y).abs()) as f32
}

fn octile_distance(a: &Point, b: &Point) -> f32 {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    let min_diff = cmp::min(dx, dy) as f32;
    let abs_diff = (dx - dy).abs() as f32;
    f32::sqrt(2.) * min_diff + abs_diff
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_euclidean_distance() {
        assert_approx_eq!(
            f32::sqrt(5.),
            euclidean_distance(&Point::new(0, 0), &Point::new(2, 1))
        );
        assert_approx_eq!(
            f32::sqrt(2.),
            euclidean_distance(&Point::new(0, 0), &Point::new(1, 1))
        );
        assert_approx_eq!(
            0.0,
            euclidean_distance(&Point::new(-1, 2), &Point::new(-1, 2))
        );
        assert_approx_eq!(
            7. * f32::sqrt(2.),
            euclidean_distance(&Point::new(2, -5), &Point::new(-5, 2))
        );
        assert_approx_eq!(
            2.0,
            euclidean_distance(&Point::new(-1, -1), &Point::new(1, -1))
        );
        assert_approx_eq!(
            2.0,
            euclidean_distance(&Point::new(-1, -1), &Point::new(-1, 1))
        );
    }

    #[test]
    fn test_manhattan_distance() {
        assert_approx_eq!(
            3.0,
            manhattan_distance(&Point::new(0, 0), &Point::new(2, 1))
        );
        assert_approx_eq!(
            2.0,
            manhattan_distance(&Point::new(0, 0), &Point::new(1, 1))
        );
        assert_approx_eq!(
            0.0,
            manhattan_distance(&Point::new(-1, 2), &Point::new(-1, 2))
        );
        assert_approx_eq!(
            14.0,
            manhattan_distance(&Point::new(2, -5), &Point::new(-5, 2))
        );
        assert_approx_eq!(
            2.0,
            manhattan_distance(&Point::new(-1, -1), &Point::new(1, -1))
        );
        assert_approx_eq!(
            2.0,
            manhattan_distance(&Point::new(-1, -1), &Point::new(-1, 1))
        );
    }

    #[test]
    fn test_chebyshev_distance() {
        assert_approx_eq!(
            2.0,
            chebyshev_distance(&Point::new(0, 0), &Point::new(2, 1))
        );
        assert_approx_eq!(
            1.0,
            chebyshev_distance(&Point::new(0, 0), &Point::new(1, 1))
        );
        assert_approx_eq!(
            0.0,
            chebyshev_distance(&Point::new(-1, 2), &Point::new(-1, 2))
        );
        assert_approx_eq!(
            7.0,
            chebyshev_distance(&Point::new(2, -5), &Point::new(-5, 2))
        );
        assert_approx_eq!(
            2.0,
            chebyshev_distance(&Point::new(-1, -1), &Point::new(1, -1))
        );
        assert_approx_eq!(
            2.0,
            chebyshev_distance(&Point::new(-1, -1), &Point::new(-1, 1))
        );
    }

    #[test]
    fn test_octile_distance() {
        assert_approx_eq!(
            1. + f32::sqrt(2.),
            octile_distance(&Point::new(0, 0), &Point::new(2, 1))
        );
        assert_approx_eq!(
            f32::sqrt(2.),
            octile_distance(&Point::new(0, 0), &Point::new(1, 1))
        );
        assert_approx_eq!(
            0.0,
            octile_distance(&Point::new(-1, 2), &Point::new(-1, 2))
        );
        assert_approx_eq!(
            7. * f32::sqrt(2.),
            octile_distance(&Point::new(2, -5), &Point::new(-5, 2))
        );
        assert_approx_eq!(
            2.0,
            octile_distance(&Point::new(-1, -1), &Point::new(1, -1))
        );
        assert_approx_eq!(
            2.0,
            octile_distance(&Point::new(-1, -1), &Point::new(-1, 1))
        );
    }
}
