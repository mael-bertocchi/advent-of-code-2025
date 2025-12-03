fn read_lines() -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }
    let file = std::fs::File::open(&args[1])?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}

fn get_largest_subsequence(digits: &[u32], k: usize) -> u64 {
    let n = digits.len();

    if n < k {
        return 0;
    }

    let mut stack: Vec<u32> = Vec::with_capacity(k);

    for (i, &digit) in digits.iter().enumerate() {
        while let Some(&top) = stack.last() {
            if digit > top && (stack.len() + n - i > k) {
                stack.pop();
            } else {
                break;
            }
        }
        if stack.len() < k {
            stack.push(digit);
        }
    }

    let mut result: u64 = 0;
    for d in stack {
        result = result * 10 + (d as u64);
    }
    result
}

fn main() {
    let lines = match read_lines() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut twelve_digit_sum: u64 = 0;
    let mut two_digit_sum: u64 = 0;

    for line in lines {
        if let Ok(content) = line {
            let digits: Vec<u32> = content.chars().filter_map(|c| c.to_digit(10)).collect();

            let largest_2 = get_largest_subsequence(&digits, 2);
            two_digit_sum += largest_2;

            let largest_12 = get_largest_subsequence(&digits, 12);
            twelve_digit_sum += largest_12;
        }
    }

    println!(
        "Sum of largest 2-digit numbers is {} and sum of largest 12-digit numbers is {}",
        two_digit_sum, twelve_digit_sum
    );
}
