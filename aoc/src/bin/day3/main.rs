#[derive(PartialEq)]
enum Tile {
    TREE,
    SPACE,
}

struct Forest {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
}

impl Forest {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Forest {
        Forest {
            width: tiles[0].len(),
            height: tiles.len(),
            tiles: tiles,
        }
    }
}

fn main() {
    let forest: Forest = Forest::new(
        include_str!("input.txt")
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| if c == '#' { Tile::TREE } else { Tile::SPACE })
                    .collect()
            })
            .collect(),
    );

    println!("part1: {}", part1(&forest));
    println!("part2: {}", part2(&forest));
}

fn part1(forest: &Forest) -> i32 {
    count_trees(forest, 3, 1)
}

fn part2(forest: &Forest) -> i64 {
    let mut total: i64 = 1;
    for (x_speed, y_speed) in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        total *= count_trees(&forest, x_speed, y_speed) as i64;
    }
    total
}

fn count_trees(forest: &Forest, x_speed: usize, y_speed: usize) -> i32 {
    let mut count = 0;
    for (y, row) in forest
        .tiles
        .iter()
        .enumerate()
        .filter(|(num, _)| num % y_speed == 0)
    {
        let x = ((y / y_speed) * x_speed) % forest.width;
        if row[x] == Tile::TREE {
            count += 1
        }
    }
    count
}
