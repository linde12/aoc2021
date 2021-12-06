fn solve(mut n_fish_by_days_remaining: [u64; 9], days: usize) -> u64 {
    (0..days).for_each(|_| {
        n_fish_by_days_remaining.rotate_left(1);
        n_fish_by_days_remaining[6] += n_fish_by_days_remaining[8];
    });
    n_fish_by_days_remaining.iter().sum()
}

fn main() {
    let input = include_str!("./input.txt").trim();
    let n_fish_by_days_remaining: [u64; 9] = input
        .split(',')
        .fold([0; 9], |mut map, remaining| {
            map[remaining.parse::<usize>().unwrap()] += 1;
            map
        });

    println!("part 1: {}", solve(n_fish_by_days_remaining.clone(), 80));
    println!("part 2: {}", solve(n_fish_by_days_remaining, 256));
}
