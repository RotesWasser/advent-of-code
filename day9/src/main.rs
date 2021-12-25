use std::{error::Error, collections::HashSet};

use clap::{App, Arg};

struct Heightmap {
    // indexed y, x
    data: Vec<Vec<Point3D>>,
    width: usize,
    height: usize
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point3D {
    x: usize,
    y: usize,
    z: u8
}

impl Point3D {
    fn new(x: usize, y: usize, z: u8) -> Self {
        return Self {
            x, y, z
        }
    }
}

impl Heightmap {
    fn get_neighbours_of_point(&self, point: &Point3D) -> Vec<&Point3D>{
        let mut neighbours = vec![];

        if point.x != 0 {
            neighbours.push(&self.data[point.y][point.x-1]);
        }

        if point.x != self.width - 1 {
            neighbours.push(&self.data[point.y][point.x + 1]);
        }

        if point.y != 0 {
            neighbours.push(&self.data[point.y - 1][point.x]);
        }

        if point.y != self.height - 1 {
            neighbours.push(&self.data[point.y + 1][point.x]);
        }

        return neighbours;
    }

    fn get_low_points(&self) -> Vec<&Point3D> {
        let mut low_points: Vec<&Point3D> = vec![];

        for grid_point in self.data.iter().flatten() {
            let is_lowpoint = self.get_neighbours_of_point(grid_point)
                .iter()
                .fold(true, |acc, neighbour| acc && neighbour.z > grid_point.z);

            if is_lowpoint {
                low_points.push(grid_point);
            }
        }
        low_points
    }

    fn calculate_basin_size_for_lowpoint(&self, starting_point: &Point3D) -> usize {
        let mut to_visit: HashSet<&Point3D> = HashSet::new();
        let mut basin_points: HashSet<&Point3D> = HashSet::new();

        to_visit.insert(starting_point);

        while to_visit.len() > 0 {
            let current_point = to_visit.iter().next().unwrap().clone();
            to_visit.remove(current_point);

            if current_point.z < 9 {
                basin_points.insert(current_point);

                let neighbours: HashSet<&Point3D> = HashSet::from_iter(self.get_neighbours_of_point(current_point).into_iter());
                let unvisited: Vec<&Point3D> = neighbours.difference(&basin_points).map(|x| *x).collect();

                to_visit.extend(unvisited.into_iter());
            }
        }
        
        return basin_points.len();
    }
}

impl TryFrom<&str> for Heightmap {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut data = vec![];

        let discovered_width: usize = match value.lines().next() {
            Some(first_line) => first_line.len(),
            None => return Err("Can't work on an empty string!".into()),
        };

        for (y, line) in value.lines().enumerate() {
            let mut line_heights = vec![];

            for (x, height_value) in line.chars().enumerate() {
                 if height_value.is_numeric() {
                    line_heights.push(
                        Point3D::new(
                            x, 
                            y, 
                            height_value.to_digit(10).unwrap() as u8
                    ));
                 }
            }

            if line_heights.len() != discovered_width {
                return Err(
                    format!("Length of line {} doesn't match length {} of the first line!", line_heights.len(), discovered_width).into()
                );
            }

            data.push(line_heights);
        }

        return Ok(
            Heightmap {
                height: data.len(),
                width: discovered_width,
                data,
            }
        );
    }
}


fn main() {
    let matches = App::new("Advent of Code Day 9")
                    .arg(Arg::with_name("INPUT")
                        .help("Input file to parse.")
                        .required(true)
                        .index(1))
                    .get_matches();

    let input = std::fs::read_to_string(&matches.value_of("INPUT").unwrap())
        .expect("Failed to open the input file");

    let map: Heightmap = input
        .trim_end()
        .try_into()
        .unwrap();

    let low_points = map.get_low_points();

    let low_point_risk_sum: usize = low_points.iter().fold(0, |acc, p| acc + p.z as usize + 1);

    let mut basin_sizes: Vec<usize> = low_points.iter().map(|lp| map.calculate_basin_size_for_lowpoint(*lp)).collect();
    basin_sizes.sort();
    basin_sizes.reverse();

    let product_of_three_largest_basins = basin_sizes.iter().take(3).fold(1, |acc, size| acc * size);

    println!("Sum of risk levels of all lowpoints: {}", low_point_risk_sum);
    println!("Product of sizes of three largest basins: {}", product_of_three_largest_basins);

}

#[cfg(test)]
const EXAMPLE_STRING: &str =
r"2199943210
3987894921
9856789892
8767896789
9899965678";

#[test]
fn test_map_parsing() {
    let map: Heightmap = EXAMPLE_STRING.try_into().unwrap();

    assert_eq!(map.width, 10);
    assert_eq!(map.height, 5);
    assert_eq!(map.data[0][0], Point3D::new(0, 0, 2));
    assert_eq!(map.data[4][9], Point3D::new(9, 4, 8));
}

#[test]
fn test_example_lowpoint_risk() {
    let map: Heightmap = EXAMPLE_STRING.try_into().unwrap();

    let low_points = map.get_low_points();
    let sum = low_points.iter().fold(0, |acc, p| acc + p.z + 1);

    assert_eq!(sum, 15);
}

#[test]
fn test_example_basin_size() {
    let map: Heightmap = EXAMPLE_STRING.try_into().unwrap();

    assert_eq!(map.calculate_basin_size_for_lowpoint(&Point3D::new(1, 0, 1)), 3);
    assert_eq!(map.calculate_basin_size_for_lowpoint(&Point3D::new(9, 0, 0)), 9);
    assert_eq!(map.calculate_basin_size_for_lowpoint(&Point3D::new(2, 2, 5)), 14);
    assert_eq!(map.calculate_basin_size_for_lowpoint(&Point3D::new(6, 4, 5)), 9);
}