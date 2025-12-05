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
    let lines = match read_lines() {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    for line in lines {
        if let Ok(content) = line {
            if content.contains("-") {
                let parts: Vec<&str> = content.split('-').collect();
                if parts.len() == 2 {
                    if let (Ok(start), Ok(end)) = (
                        parts[0].trim().parse::<u64>(),
                        parts[1].trim().parse::<u64>(),
                    ) {
                        ranges.push((start, end));
                    }
                }
            } else {
                if let Ok(id) = content.trim().parse::<u64>() {
                    ids.push(id);
                }
            }
        }
    }

    let mut number_fresh_ids: u32 = 0;

    for id in &ids {
        for (start, end) in &ranges {
            if id >= start && id <= end {
                number_fresh_ids += 1;
                break;
            }
        }
    }

    let mut number_fresh_available_ids: u64 = 0;
    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    for range in ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    for (start, end) in &merged_ranges {
        number_fresh_available_ids += end - start + 1;
    }

    println!("Step n°1: Number of fresh IDs is {}", number_fresh_ids);
    println!(
        "Step n°2: Maximum number of fresh available IDs is {}",
        number_fresh_available_ids
    );
}
