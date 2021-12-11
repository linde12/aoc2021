use std::{collections::HashSet, str::FromStr};
use itertools::Itertools;

type Point = (usize, usize);

struct Map {
    width: usize,
    height: usize,
    map: Vec<Vec<usize>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = s
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).map(|n| n as usize).collect_vec())
            .collect_vec();

        let width = map[0].len();
        let height = map.len();

        Ok(Self { map, width, height })
    }
}

impl Map {
    fn adjacent_points(&self, x: usize, y: usize) -> Vec<Point> {
        let mut adjacent = vec![];
        if x > 0 { adjacent.push((x - 1, y)); }
        if y > 0 { adjacent.push((x, y - 1)); }
        if x < self.width - 1 { adjacent.push((x + 1, y)); }
        if y < self.height - 1 { adjacent.push((x, y + 1)); }
        adjacent
    }

    fn find_low_points(&self) -> Vec<Point> {
        let mut result = vec![];
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let adjacent = self.adjacent_points(x, y);
                let found = adjacent
                    .into_iter()
                    .filter_map(|(x, y)| self.at(x, y))
                    .all(|d| d > self.at(x, y).unwrap());

                if found {
                    result.push((x, y));
                }
            }
        }
        result
    }

    fn at(&self, x: usize, y: usize) -> Option<usize> {
        Some(*(self.map.get(y)?.get(x)?))
    }

    fn find_larger_neighbours(
        &self,
        points: &Vec<Point>,
        size: usize,
        visited: &mut HashSet<Point>,
    ) -> usize {
        let mut sum = 0;
        for &(x, y) in points {
            if visited.contains(&(x, y)) {
                continue;
            }

            let point_height = self.at(x, y).unwrap();
            if point_height > size && point_height != 9 {
                let points = self.adjacent_points(x, y);
                sum += 1 + self.find_larger_neighbours(&points, point_height, visited);
                visited.insert((x, y));
            }
        }
        sum
    }
}

fn part_1(map: &Map) -> usize {
    map.find_low_points()
        .into_iter()
        .map(|(x, y)| (map.at(x, y).unwrap() + 1))
        .sum()
}

// Start from low points from part 1 and find increasing adjacent points
// recursively until a 9 is it. Keep a map of already-visited points as
// not to count them twice.
fn part_2(map: &Map) -> usize {
    let mut visited = HashSet::new();
    let low_points = map.find_low_points();
    low_points
        .into_iter()
        .filter_map(|(x, y)| {
            let height = map.at(x, y)?;
            let points = map.adjacent_points(x, y);
            let basin = map.find_larger_neighbours(&points, height, &mut visited) + 1 as usize;
            Some(basin)
        })
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn main() {
    let input = include_str!("./input.txt").trim();
    let map = Map::from_str(input).expect("bad map");

    println!("part 1: {}", part_1(&map));
    println!("part 2: {}", part_2(&map));
}
