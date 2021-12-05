use std::collections::HashMap;

type Point = (i32, i32);

#[derive(Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

struct Board {
    map: HashMap<(i32, i32), u32>,
}

impl From<&Vec<Line>> for Board {
    fn from(lines: &Vec<Line>) -> Self {
        let mut map = HashMap::new();
        lines.iter().for_each(|line| {
            let (mut lx, mut ly, rx, ry) = (line.start.0, line.start.1, line.end.0, line.end.1);
            let (dx, dy) = (
                (rx - lx).signum(), // x direction towards rx
                (ry - ly).signum(), // y direction towards ry
            );

            while (lx, ly) != (rx, ry) {
                map.entry((lx, ly)).and_modify(|v| *v += 1).or_insert(1);
                lx += dx;
                ly += dy;
            }
            map.entry((lx, ly)).and_modify(|v| *v += 1).or_insert(1);
        });

        Self { map }
    }
}

impl Board {
    fn count_overlaps(&self) -> usize {
        self.map.iter().filter(|&(_, count)| *count >= 2).count()
    }
}

fn parse(s: &str) -> Vec<Line> {
    s.lines()
        .filter_map(|line| {
            let (lhs, rhs) = line.split_once(" -> ")?;
            let (lx, ly) = lhs.split_once(",")?;
            let (rx, ry) = rhs.split_once(",")?;

            Some(Line {
                start: (lx.parse().ok()?, ly.parse().ok()?),
                end: (rx.parse().ok()?, ry.parse().ok()?),
            })
        })
        .collect()
}

fn part_1(input: &str) {
    let lines: Vec<Line> = parse(input)
        .into_iter()
        .filter(|line| line.start.0 == line.end.0 || line.start.1 == line.end.1)
        .collect();
    let board = Board::from(&lines);
    println!("{}", board.count_overlaps());
}

fn part_2(input: &str) {
    let lines = parse(input);
    let board = Board::from(&lines);
    println!("{}", board.count_overlaps());
}

fn main() {
    let input = include_str!("./input.txt").trim();
    part_1(input);
    part_2(input);
}
