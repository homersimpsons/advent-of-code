use std::collections::HashMap;

fn is_valid_passport_part1(passport: &HashMap<&str, &str>) -> bool {
    passport.contains_key(&"byr")
        && passport.contains_key(&"iyr")
        && passport.contains_key(&"eyr")
        && passport.contains_key(&"hgt")
        && passport.contains_key(&"hcl")
        && passport.contains_key(&"ecl")
        && passport.contains_key(&"pid")
}

fn is_valid_passport_part2(passport: &HashMap<&str, &str>) -> bool {
    is_valid_passport_part1(passport)
        && passport.iter().all(|(&k, v)| match k {
            "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
            "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
            "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
            "hgt" => {
                let height = v[0..(v.len() - 2)].parse().unwrap_or(0);
                match &v[(v.len() - 2)..] {
                    "cm" => (150..=193).contains(&height),
                    "in" => (59..=76).contains(&height),
                    _ => false,
                }
            }
            "hcl" => {
                v.starts_with('#')
                    && v.len() == 7
                    && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
            }
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
            "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
            "cid" => true,
            _ => unreachable!(),
        })
}

fn main() {
    let passports = include_str!("input.txt")
        .split("\n\n")
        .map(|passport| {
            passport
                .split(&[' ', '\n'][..])
                .map(|field| {
                    let mut res = field.split(':');
                    (res.next().unwrap(), res.next().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    let part1 = passports
        .iter()
        .filter(|passport| is_valid_passport_part1(passport))
        .count();
    assert_eq!(part1, 182);
    println!("part1: {}", part1);

    let part2 = passports
        .iter()
        .filter(|passport| is_valid_passport_part2(passport))
        .count();
    assert_eq!(part2, 109);
    println!("part2: {}", part2);
}
