use itertools::Itertools;

fn get_points(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("no point for char"),
    }
}

fn part_1(lines: &Vec<&str>) -> usize {
    lines
        .into_iter()
        .map(|&line| {
            let mut stack = Vec::new();
            let invalid_char = line.chars().find(|&c| match c {
                ')' => stack.pop() != Some('('),
                ']' => stack.pop() != Some('['),
                '}' => stack.pop() != Some('{'),
                '>' => stack.pop() != Some('<'),
                c => {
                    stack.push(c);
                    false
                }
            });

            match invalid_char {
                Some(c) => get_points(c),
                None => 0,
            }
        })
        .sum()
}

fn incomplete_stacks(lines: Vec<&'static str>) -> impl Iterator<Item = Vec<char>> {
    lines.into_iter().filter_map(|line| {
        let mut stack = Vec::new();
        let invalid_chars = line.chars().find(|&c| match c {
            ')' => stack.pop() != Some('('),
            ']' => stack.pop() != Some('['),
            '}' => stack.pop() != Some('{'),
            '>' => stack.pop() != Some('<'),
            c => {
                stack.push(c);
                false
            }
        });

        match invalid_chars {
            Some(_) => None,
            None => Some(stack),
        }
    })
}

fn part_2(lines: Vec<&'static str>) -> usize {
    let scores = incomplete_stacks(lines).map(|chars| {
        println!("{:?}", chars);
        chars.into_iter().rev().fold(0, |acc, c| {
            (acc * 5)
                + match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("invalid line"),
                }
        })
    }).sorted().collect_vec();
    scores[scores.len() / 2]
}


fn main() {
    let input = include_str!("./input.txt").trim();
    let lines = input.lines().collect_vec();

    println!("part 1: {}", part_1(&lines));
    println!("part 2: {}", part_2(lines));
}
