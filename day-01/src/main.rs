fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }

    let contents = std::fs::read_to_string(&args[1]).expect("Could not read file");

    let mut stopped_on_zeros: i32 = 0;
    let mut passed_zeros: i32 = 0;
    let mut current_pos: i32 = 50;

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        let direction: char = line.chars().next().unwrap();
        let mut amount: i32 = line[1..].parse().expect("Invalid number in input");

        let step: i32 = match direction {
            'R' => Ok(1),
            'L' => Ok(-1),
            other => Err(format!("Unknown direction: {}", other)),
        }
        .expect("Invalid input in file");

        while amount > 0 {
            current_pos += step;
            if current_pos.rem_euclid(100) == 0 {
                passed_zeros += 1;
            }
            amount -= 1;
        }
        if current_pos.rem_euclid(100) == 0 {
            stopped_on_zeros += 1;
        }
    }

    println!(
        "We stopped on zero {} times and clicked on zero {} times.",
        stopped_on_zeros, passed_zeros
    );
}
