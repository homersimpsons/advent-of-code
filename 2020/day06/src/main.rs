use std::collections::HashSet;

fn count_answers_part1(group_answers: &str) -> usize {
    group_answers
        .split('\n')
        .flat_map(|answers| answers.chars())
        .collect::<HashSet<_>>()
        .len()
}

fn count_answers_part2(group_answers: &str) -> usize {
    group_answers
        .split('\n')
        .map(|answers| answers.bytes())
        // We do represent the answers as a binary set
        .fold(0b00000011111111111111111111111111u32, |acc, answers| {
            acc & answers.fold(0, |acc, answer| acc | (1 << (answer - b'a')))
        })
        .count_ones() as usize
}

fn main() {
    let answers: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let part1: usize = answers.iter().map(|a| count_answers_part1(a)).sum();
    assert_eq!(part1, 7283);
    println!("part1: {}", part1);

    let part2: usize = answers.iter().map(|a| count_answers_part2(a)).sum();
    assert_eq!(part2, 3520);
    println!("part2: {}", part2);
}
