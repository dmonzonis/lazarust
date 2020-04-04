use crate::{distance::DistanceType, basemap::BaseMap, point::Point};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone)]
struct Node {
    point: Point,
    f: f32,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.f == other.f
    }
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Flip ordering on f-scores to use a min-heap
        other.f.partial_cmp(&self.f).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.f.partial_cmp(&self.f)
    }
}

fn construct_path(origin: Point, goal: Point, previous: HashMap<Point, Point>) -> Vec<Point> {
    let mut path: Vec<Point> = Vec::new();
    let mut current = goal;
    while current != origin {
        path.push(current);
        current = *previous.get(&current).unwrap();
    }
    path.reverse();
    path
}

/// Perform an A* search on the given map from origin to goal, using the given heuristic
/// to estimate distances.
/// Return the optimal path ordered from start to goal if it was found, or None if no path exists
/// between the two points.
pub fn a_star_search<T: BaseMap>(
    origin: Point,
    goal: Point,
    map: &T,
    heuristic: DistanceType,
) -> Option<Vec<Point>> {
    let mut cost_to_node: HashMap<Point, f32> = HashMap::new();
    cost_to_node.insert(origin, 0.0);
    let mut previous: HashMap<Point, Point> = HashMap::new();
    let mut open_list = BinaryHeap::new();
    open_list.push(Node {
        point: origin,
        f: 0.0,
    });

    while let Some(Node { point, f: _ }) = open_list.pop() {
        // If we popped the goal from the open_list, we can finish
        if point == goal {
            return Some(construct_path(origin, goal, previous));
        }

        // Expand neighbours
        for (neighbour, neighbour_cost) in map.neighbours(point) {
            let cost = cost_to_node.get(&point).unwrap() + neighbour_cost;
            // Consider unvisited nodes and nodes that would have a smaller total cost from this new path
            if !previous.contains_key(&neighbour) || cost < *cost_to_node.get(&point).unwrap() {
                cost_to_node.insert(neighbour, cost);
                previous.insert(neighbour, point);
                let f = cost + heuristic.distance(point, neighbour);
                open_list.push(Node {
                    point: neighbour,
                    f: f,
                });
            }
        }
    }

    None
}
