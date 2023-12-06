const INPUT: &str = include_str!("input.txt");
pub fn main() {
    let sum = part_one(INPUT);
    println!("part one: {sum}")
}

fn part_one(input: &str) -> u32 {
    let mut schematic = parse_schematic(input);
    let adjacent = get_adjacent_parts(&mut schematic);
    adjacent.iter().sum()
}

fn get_adjacent_parts(schematic: &mut Schematic) -> Vec<u32> {
    // let mut used_part_coords = HashSet::new();
    // for (row_idx, row) in schematic.grid.iter().enumerate() {
    //     for (col_idx, cell) in row.iter().enumerate() {
    //         if let GridCell::Symbol(_) = cell {
    //             let row_mask_range = {
    //                 let min = (row_idx - 1).max(0);
    //                 let max = (row_idx + 1).min(schematic.grid.len());
    //                 min..max
    //             };
    //             let col_mask_range = {
    //                 let min = (col_idx - 1).max(0);
    //                 let max = (col_idx + 1).min(row.len());
    //                 min..max
    //             };

    //             for r_idx in row_mask_range {
    //                 for c_idx in col_mask_range.clone() {
    //                     if r_idx == row_idx && c_idx == col_idx {
    //                         // current cell coords, don't add it
    //                         continue;
    //                     }
    //                     used_part_coords.insert((r_idx, c_idx));
    //                 }
    //             }
    //         }
    //     }
    // }
    let mut numbers = Vec::new();

    for (row_idx, row) in schematic.grid.iter().enumerate() {
        let mut num: u32 = 0;
        let mut first_idx = None;
        for (col_idx, cell) in row.iter().enumerate() {
            if let GridCell::Digit(d) = cell {
                num *= 10;
                num += *d as u32;
                if first_idx.is_none() {
                    first_idx = Some(col_idx);
                }
            } else if let Some(first) = first_idx {
                let min_row_idx = if row_idx <= 0 { 0 } else { row_idx - 1 };
                let max_row_idx = (row_idx + 1).min(schematic.grid.len() - 1);
                let min_col_idx = if first <= 0 { 0 } else { first - 1 };
                let max_col_idx = (col_idx + 1).min(row.len() - 1);
                'adjacent_check: for r_idx in min_row_idx..=max_row_idx {
                    for c_idx in min_col_idx..=max_col_idx {
                        let c = &schematic.grid[r_idx][c_idx];
                        if let GridCell::Symbol(_) = c {
                            numbers.push(num);
                            break 'adjacent_check;
                        }
                    }
                }

                num = 0;
                first_idx = None;
            }
        }
    }
    numbers
}

fn parse_schematic(input: &str) -> Schematic {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| {
                if let Some(d) = c.to_digit(10) {
                    GridCell::Digit(d as u8)
                } else if c == '.' {
                    GridCell::Space
                } else {
                    GridCell::Symbol(c)
                }
            })
            .collect();
        grid.push(row);
    }
    Schematic { grid }
}

struct Schematic {
    grid: Vec<Vec<GridCell>>,
}

enum GridCell {
    Digit(u8),
    Space,
    Symbol(char),
}

#[cfg(test)]
mod tests {
    use crate::day03::part_one;

    use super::INPUT;

    const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn puzzle_one_example() {
        let sum = part_one(EXAMPLE_INPUT);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn total_sum_test() {
        let mut total = 0;
        for l in INPUT.lines() {
            let line_sum: u32 = l
                .split(|c| !char::is_numeric(c))
                .filter_map(|s| s.parse::<u32>().ok())
                .sum();
            total += line_sum;
        }

        assert_ne!(total, 515956);
    }
}
