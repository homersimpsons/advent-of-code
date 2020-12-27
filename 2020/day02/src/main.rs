#[derive(Clone)]
struct PasswordAndRequirements {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl PasswordAndRequirements {
    fn new(min: usize, max: usize, character: char, password: String) -> PasswordAndRequirements {
        PasswordAndRequirements {
            min,
            max,
            character,
            password,
        }
    }
}

fn password_philosophy_part1(passwords_and_requirements: &[PasswordAndRequirements]) -> usize {
    passwords_and_requirements
        .iter()
        .filter(|password_and_requirements| {
            let nb_matches = password_and_requirements
                .password
                .matches(password_and_requirements.character)
                .count();
            nb_matches >= password_and_requirements.min
                && nb_matches <= password_and_requirements.max
        })
        .count()
}

fn password_philosophy_part2(passwords_and_requirements: &[PasswordAndRequirements]) -> usize {
    passwords_and_requirements
        .iter()
        .filter(|password_and_requirements| {
            let opt_char0 = password_and_requirements
                .password
                .chars()
                .nth(password_and_requirements.min - 1);
            let opt_char1 = password_and_requirements
                .password
                .chars()
                .nth(password_and_requirements.max - 1);
            (opt_char0.is_some() && opt_char0.unwrap() == password_and_requirements.character)
                ^ (opt_char1.is_some() && opt_char1.unwrap() == password_and_requirements.character)
        })
        .count()
}

fn main() {
    let passwords_and_requirements: Vec<PasswordAndRequirements> = include_str!("input.txt")
        .split('\n')
        .map(|raw_input| {
            // Parses "{min}-{max} {char}: {password}" to a PasswordAndRequirements
            let mut parsed = raw_input
                .split([':', '-', ' '].as_ref())
                .filter(|s| !s.is_empty());
            PasswordAndRequirements::new(
                parsed.next().unwrap().parse::<usize>().unwrap(),
                parsed.next().unwrap().parse::<usize>().unwrap(),
                parsed.next().unwrap().chars().next().unwrap(),
                parsed.next().unwrap().to_string(),
            )
        })
        .collect();

    let part1 = password_philosophy_part1(&passwords_and_requirements);
    assert_eq!(part1, 622);
    println!("part1: {}", part1);

    let part2 = password_philosophy_part2(&passwords_and_requirements);
    assert_eq!(part2, 263);
    println!("part2: {}", part2); // 263
}
