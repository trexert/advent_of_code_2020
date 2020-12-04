#[derive(Debug)]
struct Passport {
    pub byr: Option<String>,
    pub iyr: Option<String>,
    pub eyr: Option<String>,
    pub hgt: Option<String>,
    pub hcl: Option<String>,
    pub ecl: Option<String>,
    pub pid: Option<String>,
    pub cid: Option<String>,
}

impl Passport {
    fn empty() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn new(string_rep: &str) -> Passport {
        let mut passport = Passport::empty();
        for (key, value) in string_rep.trim().split_whitespace().map(|s| {
            let mut iter = s.split(":");
            (iter.next().unwrap(), iter.next().unwrap())
        }) {
            match key {
                "byr" => passport.byr = Some(value.to_string()),
                "iyr" => passport.iyr = Some(value.to_string()),
                "eyr" => passport.eyr = Some(value.to_string()),
                "hgt" => passport.hgt = Some(value.to_string()),
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                "cid" => passport.cid = Some(value.to_string()),
                _ => panic!("Unexpected key"),
            }
        }
        passport
    }

    pub fn is_valid(self: &Passport) -> bool {
        self.validate_byr()
            && self.validate_iyr()
            && self.validate_eyr()
            && self.validate_hgt()
            && self.validate_hcl()
            && self.validate_ecl()
            && self.validate_pid()
    }

    fn validate_byr(self: &Passport) -> bool {
        let mut result = false;
        if let Some(x) = &self.byr {
            if let Ok(y) = x.parse::<i32>() {
                result = y >= 1920 && y <= 2002
            }
        }
        result
    }
    fn validate_iyr(self: &Passport) -> bool {
        let mut result = false;
        if let Some(x) = &self.iyr {
            if let Ok(y) = x.parse::<i32>() {
                result = y >= 2010 && y <= 2020
            }
        }
        result
    }
    fn validate_eyr(self: &Passport) -> bool {
        let mut result = false;
        if let Some(x) = &self.eyr {
            if let Ok(y) = x.parse::<i32>() {
                result = y >= 2020 && y <= 2030
            }
        }
        result
    }
    fn validate_hgt(self: &Passport) -> bool {
        let mut result = false;
        if let Some(x) = &self.hgt {
            let num = (&x[..x.len() - 2]).parse::<i32>();
            let unit = &x[x.len() - 2..];
            result = match (num, unit) {
                (Ok(value), "cm") => value >= 150 && value <= 193,
                (Ok(value), "in") => value >= 59 && value <= 76,
                (_, _) => false,
            }
        }
        result
    }
    fn validate_hcl(self: &Passport) -> bool {
        if let Some(x) = &self.hcl {
            x.len() == 7 && x.chars().next().unwrap() == '#'
        } else {
            false
        }
    }
    fn validate_ecl(self: &Passport) -> bool {
        if let Some(x) = &self.ecl {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&x[..])
        } else {
            false
        }
    }
    fn validate_pid(self: &Passport) -> bool {
        if let Some(x) = &self.pid {
            x.len() == 9 && x.parse::<i32>().is_ok()
        } else {
            false
        }
    }
}

fn main() {
    let passports: Vec<Passport> = include_str!("input.txt")
        .split("\n\n")
        .map(|s| Passport::new(s))
        .collect();

    let part1 = passports.iter().fold(0, |acc, passport| {
        acc + if let Passport {
            byr: Some(_),
            iyr: Some(_),
            eyr: Some(_),
            hgt: Some(_),
            hcl: Some(_),
            ecl: Some(_),
            pid: Some(_),
            cid: _,
        } = passport
        {
            1
        } else {
            0
        }
    });
    println!("part1: {}", part1);
    println!(
        "part2: {}",
        passports.iter().filter(|p| p.is_valid()).count()
    );
}
