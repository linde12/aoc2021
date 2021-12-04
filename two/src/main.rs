fn main() {
    let input = include_str!("./input.txt").trim();
    let instructions: Vec<Vec<&str>> = input
        .split('\n')
        .into_iter()
        .map(|line| line.split(' ').collect())
        .collect();

    println!("{}", part_1(&instructions));
    println!("{}", part_2(&instructions));
}

fn part_1(instructions: &Vec<Vec<&str>>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    for instruction in instructions {
        match instruction[0] {
            "forward" => horizontal += instruction[1].parse::<i32>().unwrap(),
            "down" => depth += instruction[1].parse::<i32>().unwrap(),
            "up" => depth -= instruction[1].parse::<i32>().unwrap(),
            _ => {}
        }
    }
    depth * horizontal
}

fn part_2(instructions: &Vec<Vec<&str>>) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instruction in instructions {
        match instruction[0] {
            "down" => {
                aim += instruction[1].parse::<i32>().unwrap();
            }
            "up" => {
                aim -= instruction[1].parse::<i32>().unwrap();
            }
            "forward" => {
                let operand = instruction[1].parse::<i32>().unwrap();
                horizontal += operand;
                depth += operand * aim;
            }
            _ => {}
        }
    }
    depth * horizontal
}
