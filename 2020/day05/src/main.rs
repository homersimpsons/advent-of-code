fn find_empty_in_list(list: &[u16]) -> u16 {
    list.windows(2)
        .find_map(|window| {
            if window[0] + 1 < window[1] {
                Some(window[0] + 1)
            } else {
                None
            }
        })
        .unwrap()
}

fn main() {
    let mut seats = include_str!("input.txt")
        .split('\n')
        .map(|seat| {
            seat.chars()
                .rev()
                .enumerate()
                .fold(0, |acc, (index, value)| match value {
                    'R' | 'B' => acc | (1 << index),
                    'L' | 'F' => acc & !(1 << index),
                    _ => unreachable!(),
                })
        })
        .collect::<Vec<u16>>();

    let part1 = *seats.iter().max().unwrap();
    assert_eq!(part1, 906);
    println!("part1: {}", part1);

    seats.sort();
    let part2 = find_empty_in_list(&seats);
    assert_eq!(part2, 519);
    println!("part2: {}", part2);
}
