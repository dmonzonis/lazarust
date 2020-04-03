use crate::point::Point;

/// Trait to make maps work with pathfinding and FOV algorithms
pub trait BaseMap {
    /// Given a point in the map, return a list of pairs where each pair is a walkable neighbour
    /// point and the cost to go from the current point to that neighbour point.
    /// This must be implemented
    fn neighbours(&self, point: Point) -> Vec<(Point, f32)>;

    /// Return whether the given point is transparent, that is, if light can pass through it.
    /// Non-transparent (opaque) points can be lit, but they will block light rays from going further.
    fn is_transparent(&self, point: Point) -> bool;
}
