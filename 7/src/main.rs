fn part_1(positions: &Vec<i32>) -> u64 {
    let median = if positions.len() % 2 == 0 {
        let a = positions[(positions.len() - 1) / 2];
        let b = positions[positions.len() / 2];
        (a + b) / 2
    } else {
        positions[positions.len() / 2]
    };
    // the number in the middle is closest to all other numbers, so we know
    // that this is the target. calculate the ditstance from every crab to this position
    positions.iter().fold(0, |acc, pos| acc + (pos - median).abs()) as u64
}

fn fuel_spent(positions: &Vec<i32>, target: i32) -> u64 {
    positions.iter().fold(0, |acc, &pos| {
        let distance = (pos-target).abs();
        // our fuel consumption is an arithmetic progression with the length
        // of `distance`, e.g. 1, 2, 3, 4, 5 = length 5
        // so we can calculate the arithmetic sum by taking the mean of
        // the first and last value of the progression and multiplying it
        // by the length of the progression
        // e.g. for the progression 1,2,3,4,5 the mean would be:
        // (1 + 5) / 2 = 3
        // and the sum would be 5 * 3 = 15
        let fuel_requirements = distance * (1 + distance) / 2;
        acc + fuel_requirements
    }) as u64
}

fn part_2(positions: &Vec<i32>) -> u64 {
    let mean_floor = (positions.iter().sum::<i32>() as f32 / positions.len() as f32).floor() as i32;
    let mean_ceil = (positions.iter().sum::<i32>() as f32 / positions.len() as f32).ceil() as i32;
    fuel_spent(positions, mean_floor).min(fuel_spent(positions, mean_ceil))
}

fn main() {
    let input = include_str!("./input.txt").trim();
    let mut positions: Vec<i32> = input.split(',').filter_map(|n| n.parse().ok()).collect();
    positions.sort_unstable();

    println!("part 1: {}", part_1(&positions));
    println!("part 2: {}", part_2(&positions));
}
