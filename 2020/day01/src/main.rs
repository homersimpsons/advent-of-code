#![feature(is_sorted)]

fn report_pair<'a>(
    expenses: &'a [usize],
    sum_to_get: usize,
    number_of_entries: &'a mut [usize],
) -> bool {
    if number_of_entries.len() == 1 {
        assert!(expenses.is_sorted());
        if expenses.binary_search(&sum_to_get).is_ok() {
            number_of_entries[0] = sum_to_get;
            true
        } else {
            false
        }
    } else {
        expenses.iter().enumerate().any(|(index, &expense)| {
            if let Some(target_sum) = sum_to_get.checked_sub(expense) {
                if report_pair(
                    &expenses[(index + 1)..],
                    target_sum,
                    &mut number_of_entries[1..],
                ) {
                    number_of_entries[0] = expense;
                    true
                } else {
                    false
                }
            } else {
                false
            }
        })
    }
}

fn main() {
    dbg!();
    let mut raw_expenses = include_str!("input.txt")
        .split('\n')
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    raw_expenses.sort_unstable();
    let expenses = raw_expenses.as_slice();

    let mut part1 = [0; 2];
    assert!(report_pair(&expenses, 2020, &mut part1));
    assert_eq!(part1[0] * part1[1], 719796);
    println!(
        "{} + {} = {} and {} * {} = {}",
        part1[0],
        part1[1],
        part1[0] + part1[1],
        part1[0],
        part1[1],
        part1[0] * part1[1]
    );

    let mut part2 = [0; 3];
    assert!(report_pair(&expenses, 2020, &mut part2));
    assert_eq!(part2[0] * part2[1] * part2[2], 144554112);
    println!(
        "{} + {} + {} = {} and {} * {} * {} = {}",
        part2[0],
        part2[1],
        part2[2],
        part2[0] + part2[1] + part2[2],
        part2[0],
        part2[1],
        part2[2],
        part2[0] * part2[1] * part2[2]
    );
}
