#![feature(in_band_lifetimes)]

use std::collections::{HashMap, HashSet};

const ITERATIONS: usize = 100;

fn main() {
    let instructions: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| parse_instruction(line))
        .collect();

    let mut lobby = Lobby::from_instructions(&instructions);

    println!("part1: {}", lobby.black_tiles());
    println!("part2: {}", part2(&mut lobby));
}

fn part2(lobby: &mut Lobby) -> usize {
    for _ in 0..ITERATIONS {
        lobby.step();
    }

    lobby.black_tiles()
}

struct Lobby {
    tiles: HashMap<Vec<i32>, bool>,
}

impl Lobby {
    pub fn from_instructions(instructions: &Vec<Vec<i32>>) -> Lobby {
        let mut tiles = HashMap::with_capacity(instructions.len());
        instructions
            .iter()
            .for_each(|instruction| *tiles.entry(instruction.clone()).or_default() ^= true);
        Lobby { tiles }
    }

    pub fn step(&mut self) {
        let tile_changes: HashMap<_, _> = self
            .boundary_iter()
            .filter_map(|pos| {
                match (self.count_neighbors(&pos), self.tiles.get(&pos)) {
                    (0, Some(true)) => Some((pos, false)), // Black tiles with 0 neighbors become white
                    (x, Some(true)) if x > 2 => Some((pos, false)), // Black tiles with more than 2 neighbors become white
                    (_, Some(true)) => None, // Black tiles with any other number of neighbors stay black
                    (2, _) => Some((pos, true)), // White tiles with 2 black neighbors become black
                    (_, _) => None, // White tiles with any other number of neighbors stay white
                }
            })
            .collect();
        self.tiles.extend(tile_changes);
    }

    pub fn black_tiles(&self) -> usize {
        self.tiles.values().filter(|&&black| black).count()
    }

    fn neighbors(&self, pos: &Vec<i32>) -> HashSet<Vec<i32>> {
        let neighbor_positions = vec![
            vec![2, 0],
            vec![1, 1],
            vec![-1, 1],
            vec![-2, 0],
            vec![-1, -1],
            vec![1, -1],
        ];
        neighbor_positions
            .iter()
            .map(|position| {
                position
                    .iter()
                    .zip(pos)
                    .map(|(increment, loc)| increment + loc)
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn count_neighbors(&self, pos: &Vec<i32>) -> usize {
        self.neighbors(pos)
            .iter()
            .filter_map(|neighbor| {
                if let Some(true) = self.tiles.get(neighbor) {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    }

    fn boundary_iter(&'a self) -> impl Iterator<Item = Vec<i32>> + 'a {
        self.tiles
            .iter()
            .filter(|(_pos, &black)| black)
            .flat_map(move |(pos, _black)| {
                let mut neighbors = self.neighbors(pos);
                neighbors.insert(pos.clone());
                neighbors
            })
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

fn parse_instruction(instruction: &str) -> Vec<i32> {
    let mut vertical_is_north: Option<bool> = None;
    let mut position = vec![0, 0];
    for letter in instruction.chars() {
        match letter {
            'n' => vertical_is_north = Some(true),
            'e' => match vertical_is_north {
                Some(true) => {
                    vertical_is_north = None;
                    position[0] += 1;
                    position[1] += 1;
                }
                Some(false) => {
                    vertical_is_north = None;
                    position[0] += 1;
                    position[1] -= 1;
                }
                None => position[0] += 2,
            },
            's' => vertical_is_north = Some(false),
            'w' => match vertical_is_north {
                Some(true) => {
                    vertical_is_north = None;
                    position[0] -= 1;
                    position[1] += 1;
                }
                Some(false) => {
                    vertical_is_north = None;
                    position[0] -= 1;
                    position[1] -= 1;
                }
                None => position[0] -= 2,
            },
            _ => panic!("Unexpected character"),
        }
    }
    position
}

#[cfg(test)]
mod tests {
    use crate::{parse_instruction, Lobby};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn correct_neighbors() {
        let lobby: Lobby = Lobby::from_instructions(
            &include_str!("test_input_2.txt")
                .lines()
                .map(|line| parse_instruction(line))
                .collect(),
        );
        assert_eq!(
            lobby.tiles,
            vec![(vec![0, 0], true), (vec![2, 0], true)]
                .into_iter()
                .collect::<HashMap<_, _>>()
        );
        assert_eq!(
            lobby.neighbors(&vec![0, 0]),
            vec![
                vec![2, 0],
                vec![1, 1],
                vec![-1, 1],
                vec![-2, 0],
                vec![-1, -1],
                vec![1, -1],
            ]
            .into_iter()
            .collect::<HashSet<Vec<i32>>>()
        );
        assert_eq!(lobby.count_neighbors(&vec![0, 0]), 1);
    }
}
