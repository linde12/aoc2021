fn main() {
    let input = include_str!("./input.txt").trim();
    let lines: Vec<&str> = input
        .split('\n')
        .collect();

    let n_bits = lines[0].len();
    let numbers = lines.into_iter().map(|line| { i32::from_str_radix(line, 2).unwrap() }).collect();

    let [gamma, epsilon] = part_1(&numbers, n_bits);
    println!("{}", gamma * epsilon);
    let [oxygen, co2] = part_2(&numbers, n_bits);
    println!("{}", oxygen * co2);
}

/// Checks whether bit at position `pos` is 1 (set)
fn is_set(n: i32, pos: usize) -> bool {
    (n >> pos) & 0b1 == 1
}

/// Calculates the sum of bits in `numbers` at position `pos`
fn bitsum(numbers: &Vec<i32>, pos: usize) -> i32 {
    numbers.into_iter().map(|&n| {
        if is_set(n, pos) { 1 } else { -1 }
    }).sum::<i32>()
}

fn part_1(numbers: &Vec<i32>, bitlength: usize) -> [i32; 2] {
    let mut gamma: i32 = 0;

    for i in 0..bitlength {
        let sum = bitsum(&numbers, i);
        let flag = if sum > 0 { 1 } else { 0 };
        gamma |= flag << i;
    }

    let mut bitmask = 0;
    for i in 0..bitlength {
        bitmask |= 1 << i;
    }

    let epsilon = !gamma & bitmask;
    [gamma, epsilon]
}

enum Type {
    Oxygen,
    CO2
}

fn filter_diagnostics(mut numbers: Vec<i32>, bitlength: usize, diag_type: Type) -> i32 {
    for i in (0..bitlength).rev() {
        let sum = bitsum(&numbers, i);
        let flag = if sum >= 0 { 1 } else { 0 };

        numbers = numbers.into_iter().filter(|n| {
            let bit_value = (n >> i) & 0b1;
            match diag_type {
                Type::Oxygen => bit_value == flag,
                Type::CO2 => bit_value != flag,
            }
        }).collect();

        if numbers.len() == 1 {
            break;
        }
    }
    numbers[0]
}

fn part_2(numbers: &Vec<i32>, bitlength: usize) -> [i32; 2] {
    let oxygen = filter_diagnostics(numbers.clone(), bitlength, Type::Oxygen);
    let co2 = filter_diagnostics(numbers.clone(), bitlength, Type::CO2);
    [oxygen, co2]
}
