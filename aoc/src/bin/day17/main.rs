use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Iterator;

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd)]
struct Point {
    pos: Vec<i32>,
}

impl Point {
    pub fn neighbors(&self) -> Vec<Point> {
        let mut neighbors = vec![self.clone()];
        for dimension in 0..self.pos.len() {
            let new_neighbors: Vec<_> = neighbors
                .iter()
                .flat_map(|neighbor| {
                    [-1, 1].iter().map(move |variation| {
                        let mut new_neighbor = neighbor.clone();
                        new_neighbor.pos[dimension] += variation;
                        new_neighbor
                    })
                })
                .collect();
            neighbors.extend(new_neighbors);
        }

        neighbors
    }
}

#[derive(Clone)]
struct Pocket {
    cubes: HashMap<Point, bool>,
    dimensions: usize,
}

impl Pocket {
    pub fn from_initial(initial_cubes: &str, dimensions: usize) -> Pocket {
        assert!(dimensions >= 2);
        let new_cubes = initial_cubes
            .lines()
            .enumerate()
            .flat_map(|(j, line)| {
                line.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let alive = match c {
                            '#' => true,
                            '.' => false,
                            _ => panic!("Unexpected character in initial state"),
                        };
                        let mut pos = vec![0; dimensions];
                        pos[0] = i as i32;
                        pos[1] = j as i32;
                        (Point { pos }, alive)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Pocket {
            cubes: new_cubes,
            dimensions: dimensions,
        }
    }

    pub fn step(&mut self) {
        let changes: HashMap<Point, bool> = self
            .boundary_iter()
            .filter_map(|(point, active)| {
                let new_active = match self.count_neighbors(&point) {
                    2 => active,
                    3 => true,
                    _ => false,
                };
                if new_active != active {
                    Some((point, new_active))
                } else {
                    None
                }
            })
            .collect();

        self.cubes.extend(changes);
    }

    pub fn count_active(&self) -> usize {
        self.iter().count()
    }

    fn get(&self, point: &Point) -> bool {
        if let Some(active) = self.cubes.get(point) {
            *active
        } else {
            false
        }
    }

    fn count_neighbors(&self, point: &Point) -> usize {
        point
            .neighbors()
            .iter()
            .filter(|neighbor| self.get(neighbor) && *neighbor != point)
            .count()
    }

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = Point> + 'a> {
        Box::new(
            self.cubes
                .iter()
                .filter_map(|(point, active)| if *active { Some(point.clone()) } else { None }),
        )
    }

    fn boundary_iter<'a>(&'a self) -> Box<dyn Iterator<Item = (Point, bool)> + 'a> {
        let boundary_set: Box<HashSet<Point>> =
            Box::new(self.iter().flat_map(|point| point.neighbors()).collect());
        Box::new(boundary_set.into_iter().map(move |point| {
            let active = self.get(&point);
            (point, active)
        }))
    }
}

fn main() {
    let initial_string = include_str!("input.txt");

    println!(
        "part1: {}",
        boot(&mut Pocket::from_initial(initial_string, 3))
    );
    println!(
        "part2: {}",
        boot(&mut Pocket::from_initial(initial_string, 4))
    );
}

fn boot(pocket: &mut Pocket) -> usize {
    for _ in 0..6 {
        pocket.step();
    }
    pocket.count_active()
}
