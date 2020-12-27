use core::panic;

#[derive(Debug, Clone)]
struct Row {
    row: Vec<bool>,
}

impl Row {
    fn new(row: Vec<bool>) -> Row {
        Row { row }
    }

    fn get(&self, position: usize) -> bool {
        *(self.row.get(position % self.row.len()).unwrap())
    }
}

fn toboggan_trajectory(forest: &[Row], down: usize, right: usize) -> usize {
    forest
        .iter()
        .enumerate()
        .step_by(down)
        .filter(|(index, row)| row.get((index / down) * right))
        .count()
}

fn main() {
    let forest: Vec<Row> = include_str!("input.txt")
        .split('\n')
        .map(|tree_row| {
            Row::new(
                tree_row
                    .chars()
                    .map(|possibly_tree| match possibly_tree {
                        '.' => false, // Not a tree
                        '#' => true,  // A tree
                        _ => panic!("Impossible char !"),
                    })
                    .collect(),
            )
        })
        .collect();

    let part1 = toboggan_trajectory(&forest, 1, 3);
    assert_eq!(part1, 247);
    println!("part1: {}", part1);

    let part2 = toboggan_trajectory(&forest, 1, 1)
        * toboggan_trajectory(&forest, 1, 3)
        * toboggan_trajectory(&forest, 1, 5)
        * toboggan_trajectory(&forest, 1, 7)
        * toboggan_trajectory(&forest, 2, 1);
    assert_eq!(part2, 2983070376);
    println!("part2: {}", part2);
}
