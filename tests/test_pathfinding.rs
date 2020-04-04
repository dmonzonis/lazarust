use lazarust::{basemap::BaseMap, distance::DistanceType, pathfinding::a_star_search, point::Point};
use lazy_static::lazy_static;

#[derive(Clone)]
struct SimpleMap {
    width: i32,
    height: i32,
    costs: Vec<f32>,
}

impl SimpleMap {
    fn point_to_index(&self, point: Point) -> usize {
        (point.y * self.width + point.x) as usize
    }

    fn is_out_of_bounds(&self, point: Point) -> bool {
        point.x < 0 || point.x >= self.width || point.y < 0 || point.y >= self.height
    }

    fn is_wall(&self, point: Point) -> bool {
        self.costs[self.point_to_index(point)] < 0.
    }
}

// Generate a simple map from a matrix of 0s and 1s, where 0s are floors (cost 1) and 1s are
// walls (unpassable, i.e. infinite cost, but in this implementation we encode them as -1)
impl From<&Vec<Vec<i32>>> for SimpleMap {
    fn from(prefab: &Vec<Vec<i32>>) -> Self {
        let height = prefab.len() as i32;
        let width = prefab[0].len() as i32;
        let mut simpe_map = Self {
            height,
            width,
            costs: vec![-1.; (width * height) as usize],
        };
        for y in 0..simpe_map.height {
            for x in 0..simpe_map.width {
                if prefab[y as usize][x as usize] == 0 {
                    let idx = simpe_map.point_to_index(Point::new(x, y));
                    simpe_map.costs[idx] = 1.0;
                }
            }
        }
        simpe_map
    }
}

impl BaseMap for SimpleMap {
    // All directions including diagonals
    fn neighbours(&self, point: Point) -> Vec<(Point, f32)> {
        let mut result: Vec<(Point, f32)> = Vec::new();
        let directions = vec![
            Point::new(1, 0),
            Point::new(-1, 0),
            Point::new(0, 1),
            Point::new(0, -1),
            Point::new(1, 1),
            Point::new(-1, -1),
            Point::new(1, -1),
            Point::new(-1, 1),
        ];
        for dir in directions {
            let neighbour = point + dir;
            if !self.is_out_of_bounds(neighbour) && !self.is_wall(neighbour) {
                // Cost is simply the cost of the tile (diagonal movement costs the same)
                result.push((neighbour, self.costs[self.point_to_index(neighbour)]));
            }
        }
        result
    }

    fn is_transparent(&self, point: Point) -> bool {
        true // We are not testing FOV here
    }
}

lazy_static! {
    static ref SIMPLEMAP: SimpleMap = SimpleMap::from(&vec![
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 1],
        vec![1, 1, 1, 0, 1],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 1]
    ]);
}

#[test]
fn a_star_with_diagonals() {
    let path = a_star_search(
        Point::new(1, 3),
        Point::new(0, 0),
        &*SIMPLEMAP,
        DistanceType::Manhattan,
    )
    .unwrap(); // Path must exist
    assert_eq!(6, path.len());
    assert_eq!(Point::new(2, 4), path[0]);
    assert_eq!(Point::new(3, 3), path[1]);
    assert_eq!(Point::new(3, 2), path[2]);
    assert_eq!(Point::new(2, 1), path[3]);
    assert_eq!(Point::new(1, 0), path[4]);
    assert_eq!(Point::new(0, 0), path[5]);
}

#[test]
fn a_star_non_existant_path() {
    // Close off the path to (0, 0)
    let mut cloned_map = SIMPLEMAP.clone();
    let tile_idx = cloned_map.point_to_index(Point::new(1, 0));
    cloned_map.costs[tile_idx] = -1.;
    let path = a_star_search(
        Point::new(1, 3),
        Point::new(0, 0),
        &cloned_map,
        DistanceType::Manhattan,
    );
    assert!(
        path.is_none(),
        "A path should not exist but the following path was returned: {:?}",
        path.unwrap()
    );
}

#[test]
fn a_star_with_varying_costs() {
    // Set cost 10 on (3, 3), so the pathfinder will avoid it even if it requires more steps
    let mut cloned_map = SIMPLEMAP.clone();
    let tile_idx = cloned_map.point_to_index(Point::new(3, 3));
    cloned_map.costs[tile_idx] = 10.;
    let path = a_star_search(
        Point::new(3, 2),
        Point::new(2, 4),
        &cloned_map,
        DistanceType::Manhattan,
    )
    .unwrap();
    assert_eq!(3, path.len());
    assert_eq!(Point::new(4, 3), path[0]);
    assert_eq!(Point::new(3, 4), path[1]);
    assert_eq!(Point::new(2, 4), path[2]);
}

#[test]
fn a_star_to_same_point() {
    let path = a_star_search(
        Point::new(3, 2),
        Point::new(3, 2),
        &*SIMPLEMAP,
        DistanceType::Manhattan,
    )
    .unwrap(); // Path will exist but will be empty
    assert!(path.is_empty());
}
