fn count_easy_numbers(output: &Vec<(u8, usize)>) -> usize {
    output
        .into_iter()
        .filter(|(_, len)| {
            match len {
                2 => true, // 1
                3 => true, // 7
                4 => true, // 4
                7 => true, // 8
                _ => false,
            }
        })
        .count()
}

fn part_1(positions: &Vec<(Vec<(u8, usize)>, Vec<(u8, usize)>)>) -> usize {
    positions
        .iter()
        .map(|(_, output)| count_easy_numbers(output))
        .sum()
}

/// This is so we can use bitwise operators in order to e.g. diff & intersect arrays
fn hash(display: &str) -> u8 {
    let mut res = 0;
    display.chars().for_each(|c| {
        res |= match c {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => panic!("invalid display"),
        }
    });
    res
}

fn calculate_output_number(
    displays: &Vec<(u8, usize)>,
    output_displays: &Vec<(u8, usize)>,
) -> usize {
    let (mut one, mut four) = (0, 0);
    for &(display, len) in displays {
        match len {
            2 => one = display,  // 1
            4 => four = display, // 4
            _ => {}
        }
    }

    output_displays
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, &(display, len))| {
            10_usize.pow(i as u32)
                * match len {
                    2 => 1,
                    4 => 4,
                    3 => 7,
                    7 => 8,
                    5 => {
                        if (display & one).count_ones() == 2 {
                            3
                        } else if (display & four).count_ones() == 3 {
                            5
                        } else {
                            2
                        }
                    }
                    6 => {
                        if (display & four).count_ones() == 4 {
                            9
                        } else if (display & one).count_ones() == 2 {
                            0
                        } else {
                            6
                        }
                    }
                    _ => panic!("invalid display"),
                }
        })
        .sum()
}

fn part_2(positions: &Vec<(Vec<(u8, usize)>, Vec<(u8, usize)>)>) -> usize {
    positions
        .into_iter()
        .map(|(pattern, output)| calculate_output_number(pattern, output))
        .sum()
}

fn main() {
    let input = include_str!("./input.txt").trim();
    let positions: Vec<(Vec<(u8, usize)>, Vec<(u8, usize)>)> = input
        .lines()
        .filter_map(|line| line.split_once(" | "))
        .map(|(patterns, outputs)| {
            (
                patterns
                    .split_whitespace()
                    .map(|s| (hash(s), s.len()))
                    .collect(),
                outputs
                    .split_whitespace()
                    .map(|s| (hash(s), s.len()))
                    .collect(),
            )
        })
        .collect();

    println!("part 1: {}", part_1(&positions));
    println!("part 2: {}", part_2(&positions));
}
