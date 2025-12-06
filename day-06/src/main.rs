fn read_lines() -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }
    let file = std::fs::File::open(&args[1])?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}

fn main() {
    let raw_lines: Vec<String> = match read_lines() {
        Ok(lines_iter) => lines_iter.filter_map(|l| l.ok()).collect(),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut numbers: Vec<Vec<u32>> = Vec::new();
    let mut signs: Vec<char> = Vec::new();

    for content in &raw_lines {
        let parts: Vec<&str> = content.split_whitespace().collect();
        let mut numbers_to_add: Vec<u32> = Vec::new();
        for part in parts {
            if let Ok(num) = part.parse::<u32>() {
                numbers_to_add.push(num);
            } else {
                for c in part.chars() {
                    signs.push(c);
                }
            }
        }
        if !numbers_to_add.is_empty() {
            numbers.push(numbers_to_add);
        }
    }

    let mut horizontal_result: i64 = 0;
    if !numbers.is_empty() {
        for i in 0..numbers[0].len() {
            let mut column_result: i64 = numbers[0][i] as i64;

            for j in 1..numbers.len() {
                match signs[i] {
                    '+' => column_result += numbers[j][i] as i64,
                    '*' => column_result *= numbers[j][i] as i64,
                    _ => (),
                }
            }
            horizontal_result += column_result;
        }
    }

    println!(
        "Step n°1: The result of the operations horizontally is {}",
        horizontal_result
    );

    let max_len = raw_lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in &raw_lines {
        let mut row: Vec<char> = line.chars().collect();
        while row.len() < max_len {
            row.push(' ');
        }
        grid.push(row);
    }

    let mut vertical_result: i64 = 0;
    let mut current_problem_numbers: Vec<i64> = Vec::new();

    let operator_row_index = grid.len() - 1;
    let rows_of_numbers = grid.len() - 1;

    for x in 0..max_len {
        let mut col_is_empty = true;
        let mut number_string = String::new();

        for y in 0..rows_of_numbers {
            let c = grid[y][x];
            if c.is_digit(10) {
                col_is_empty = false;
                number_string.push(c);
            }
        }

        if col_is_empty {
            if !current_problem_numbers.is_empty() {
                let mut op = '+';
                for search_x in (0..x).rev() {
                    let c = grid[operator_row_index][search_x];
                    if c == '+' || c == '*' {
                        op = c;
                        break;
                    }
                }

                let mut problem_sum = current_problem_numbers[0];
                for &num in &current_problem_numbers[1..] {
                    match op {
                        '+' => problem_sum += num,
                        '*' => problem_sum *= num,
                        _ => (),
                    }
                }
                vertical_result += problem_sum;
                current_problem_numbers.clear();
            }
        } else {
            if let Ok(num) = number_string.parse::<i64>() {
                current_problem_numbers.push(num);
            }
        }
    }

    if !current_problem_numbers.is_empty() {
        let mut op = '+';
        for search_x in (0..max_len).rev() {
            let c = grid[operator_row_index][search_x];
            if c == '+' || c == '*' {
                op = c;
                break;
            }
        }
        let mut problem_sum = current_problem_numbers[0];
        for &num in &current_problem_numbers[1..] {
            match op {
                '+' => problem_sum += num,
                '*' => problem_sum *= num,
                _ => (),
            }
        }
        vertical_result += problem_sum;
    }

    println!(
        "Step n°2: The result of the operations vertically is {}",
        vertical_result
    );
}
