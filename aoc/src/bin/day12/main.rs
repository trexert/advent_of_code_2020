use std::fmt;

trait ShipShape {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn perform(&mut self, action: &str);
}

#[derive(Debug)]
struct Ship {
    pub x: i32,
    pub y: i32,
    pub bearing: i32,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            bearing: 90,
        }
    }
}

impl ShipShape for Ship {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn perform(&mut self, action: &str) {
        let action_type = &action[0..1];
        let action_value = &action[1..].parse().unwrap();
        match action_type {
            "N" => self.y += action_value,
            "E" => self.x += action_value,
            "S" => self.y -= action_value,
            "W" => self.x -= action_value,
            "L" => self.bearing = (self.bearing - action_value).rem_euclid(360),
            "R" => self.bearing = (self.bearing + action_value).rem_euclid(360),
            "F" => match self.bearing {
                0 => self.y += action_value,
                90 => self.x += action_value,
                180 => self.y -= action_value,
                270 => self.x -= action_value,
                bearing => panic!("Unexpected bearing {}", bearing),
            },
            _ => panic!("Unexpeted action {}", action),
        }
    }
}

#[derive(Debug)]
struct Ship2 {
    x: i32,
    y: i32,
    way_x: i32,
    way_y: i32,
}

impl Ship2 {
    pub fn new() -> Ship2 {
        Ship2 {
            x: 0,
            y: 0,
            way_x: 10,
            way_y: 1,
        }
    }
}

impl ShipShape for Ship2 {
    fn x(&self) -> i32 {
        self.x
    }
    fn y(&self) -> i32 {
        self.y
    }
    fn perform(&mut self, action: &str) {
        let action_type = &action[0..1];
        let action_value = &action[1..].parse().unwrap();
        match action_type {
            "N" => self.way_y += action_value,
            "E" => self.way_x += action_value,
            "S" => self.way_y -= action_value,
            "W" => self.way_x -= action_value,
            "L" => {
                for _ in 0..(action_value / 90) {
                    let way_xy = (-self.way_y, self.way_x);
                    self.way_x = way_xy.0;
                    self.way_y = way_xy.1;
                }
            }
            "R" => {
                for _ in 0..(action_value / 90) {
                    let way_xy = (self.way_y, -self.way_x);
                    self.way_x = way_xy.0;
                    self.way_y = way_xy.1;
                }
            }
            "F" => {
                self.x += self.way_x * action_value;
                self.y += self.way_y * action_value;
            }
            _ => panic!("Unexpeted action {}", action),
        }
    }
}

fn main() {
    let actions: Vec<_> = include_str!("input.txt").lines().collect();

    println!("part1: {}", run_actions(&mut Ship::new(), &actions));
    println!("part2: {}", run_actions(&mut Ship2::new(), &actions));
}

fn run_actions(ship: &mut (impl ShipShape + fmt::Debug), actions: &Vec<&str>) -> i32 {
    actions.iter().for_each(|action| ship.perform(action));
    ship.x().abs() + ship.y().abs()
}
