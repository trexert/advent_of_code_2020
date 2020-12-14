use std::cmp::min;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
enum Chair {
    FLOOR,
    OPEN,
    OCCUPIED,
}

impl fmt::Debug for Chair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Chair::FLOOR => ".",
            Chair::OPEN => "L",
            Chair::OCCUPIED => "#",
        })
    }
}

#[derive(Clone)]
struct Area {
    pub seats: Vec<Vec<Chair>>,
    pub height: usize,
    pub width: usize,
}

impl fmt::Debug for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .seats
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|seat| format!("{:?}", seat))
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

impl Area {
    pub fn from_seats(seats: Vec<Vec<Chair>>) -> Area {
        Area {
            seats: seats.clone(),
            height: seats.len(),
            width: seats[0].len(),
        }
    }

    pub fn step(&mut self, is_part2: bool) -> usize {
        let mut changes = 0;
        let mut new_seats = self.seats.clone();
        let limit = if is_part2 { 5 } else { 4 };
        for y in 0..self.height {
            for x in 0..self.width {
                if self.seats[y][x] != Chair::FLOOR {
                    new_seats[y][x] = match if is_part2 {
                        self.count_neighbors2(x, y)
                    } else {
                        self.count_neighbors(x, y)
                    } {
                        0 => {
                            changes += (self.seats[y][x] == Chair::OPEN) as usize;
                            Chair::OCCUPIED
                        }
                        count if count < limit => self.seats[y][x],
                        _ => {
                            changes += (self.seats[y][x] == Chair::OCCUPIED) as usize;
                            Chair::OPEN
                        }
                    }
                }
            }
        }
        self.seats = new_seats;
        changes
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut neighbors = 0;
        let min_y = if y == 0 { 0 } else { y - 1 };
        let min_x = if x == 0 { 0 } else { x - 1 };
        let max_y = min(self.height, y + 2);
        let max_x = min(self.width, x + 2);
        for j in min_y..max_y {
            for i in min_x..max_x {
                if i != x || j != y {
                    neighbors += (self.seats[j][i] == Chair::OCCUPIED) as usize;
                }
            }
        }
        neighbors
    }

    fn count_neighbors2(&self, x: usize, y: usize) -> usize {
        let mut neighbors = 0;
        for j in -1..2 {
            for i in -1..2 {
                if i != 0 || j != 0 {
                    neighbors += (self.find_neighbor(x, y, i, j) == Chair::OCCUPIED) as usize;
                }
            }
        }
        neighbors
    }

    fn find_neighbor(&self, x: usize, y: usize, x_dir: i32, y_dir: i32) -> Chair {
        let mut neighbor = Chair::FLOOR;
        let mut new_x = x as i32 + x_dir;
        let mut new_y = y as i32 + y_dir;
        while new_x >= 0 && new_x < self.width as i32 && new_y >= 0 && new_y < self.height as i32 {
            if self.seats[new_y as usize][new_x as usize] != Chair::FLOOR {
                neighbor = self.seats[new_y as usize][new_x as usize];
                break;
            }
            new_x += x_dir;
            new_y += y_dir;
        }
        neighbor
    }

    pub fn count_occupied(&self) -> usize {
        self.seats.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |inner_acc, seat| {
                inner_acc + (*seat == Chair::OCCUPIED) as usize
            })
        })
    }
}

fn main() {
    let seats = Area::from_seats(
        include_str!("input.txt")
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '.' => Chair::FLOOR,
                        'L' => Chair::OPEN,
                        '#' => Chair::OCCUPIED,
                        _ => panic!("Unexpected character"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    );

    println!("part1: {}", part1(&mut seats.clone()));
    println!("part2: {}", part2(&mut seats.clone()));
}

fn part1(seats: &mut Area) -> usize {
    while seats.step(false) != 0 {}
    seats.count_occupied()
}

fn part2(seats: &mut Area) -> usize {
    while seats.step(true) != 0 {}
    seats.count_occupied()
}
