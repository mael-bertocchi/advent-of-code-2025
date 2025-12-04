const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn read_lines() -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }
    let file = std::fs::File::open(&args[1])?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}

fn count_neighbors(grid: &Vec<Vec<char>>, r: usize, c: usize) -> usize {
    let cols = grid[0].len() as isize;
    let rows = grid.len() as isize;
    let mut count = 0;

    for (dr, dc) in DIRECTIONS {
        let nrows = r as isize + dr;
        let ncols = c as isize + dc;

        if nrows >= 0 && nrows < rows && ncols >= 0 && ncols < cols {
            if grid[nrows as usize][ncols as usize] != '@' {
                continue;
            }
            count += 1;
        }
    }
    count
}

fn calculate_accessible_rolls_of_paper(grid: &Vec<Vec<char>>) -> u64 {
    let mut count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '@' {
                continue;
            }
            if count_neighbors(grid, row, col) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn calculate_removable_rolls_of_paper(mut grid: Vec<Vec<char>>) -> u64 {
    let mut total_removed = 0;

    loop {
        let mut to_remove = Vec::new();

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] != '@' {
                    continue;
                }
                if count_neighbors(&grid, row, col) < 4 {
                    to_remove.push((row, col));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        total_removed += to_remove.len() as u64;
        for (row, col) in to_remove {
            grid[row][col] = '.';
        }
    }

    total_removed
}

fn main() {
    let lines = match read_lines() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut array: Vec<Vec<char>> = Vec::new();
    for line in lines {
        if let Ok(content) = line {
            array.push(content.chars().collect());
        }
    }

    let accessible_rolls_of_paper = calculate_accessible_rolls_of_paper(&array);
    let removable_rolls_of_paper = calculate_removable_rolls_of_paper(array.clone());

    println!(
        "Step n°1: The number of accessible rolls of paper is {}",
        accessible_rolls_of_paper
    );
    println!(
        "Step n°2: The number of removable rolls of paper is {}",
        removable_rolls_of_paper
    );
}
