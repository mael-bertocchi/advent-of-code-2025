fn analyse_line(line: &str) -> i32 {
    let direction: char = line.chars().next().unwrap();
    let amount: i32 = line[1..].parse().unwrap();

    match direction {
        'L' => -amount,
        'R' => amount,
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let contents = std::fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading '{}': {}", filename, err);
        std::process::exit(1);
    });

    let mut current_pos: i32 = 50;
    let mut total_zeros: i32 = 0;

    for line in contents.lines() {
        let movement: i32 = analyse_line(line);

        current_pos = (((current_pos + movement) % 100) + 100) % 100;
        if current_pos == 0 {
            total_zeros += 1;
        }
    }
    println!("Total number of zero: {}", total_zeros);
}
