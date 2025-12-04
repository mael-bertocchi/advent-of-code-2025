fn read_file_from_args() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input-file>", args[0]);
        std::process::exit(1);
    }
    std::fs::read_to_string(&args[1]).expect("Could not read file")
}

fn is_repeated_twice(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();

    if len % 2 != 0 {
        return false;
    }

    let mid = len / 2;
    &bytes[..mid] == &bytes[mid..]
}

fn is_repeated_multiple_times(s: &str) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();

    for size in 1..=len / 2 {
        if len % size != 0 {
            continue;
        }

        let pattern = &bytes[..size];

        if bytes.chunks(size).all(|chunk| chunk == pattern) {
            return true;
        }
    }
    false
}

fn main() {
    let content = read_file_from_args();

    let mut sequence_repeated_multiple_times: u64 = 0;
    let mut sequence_repeated_twice: u64 = 0;

    for item in content.trim().split(',') {
        if let Some((start_str, end_str)) = item.split_once('-') {
            let start: u64 = start_str.parse().expect("Invalid start");
            let end: u64 = end_str.parse().expect("Invalid end");

            for num in start..=end {
                let s: String = num.to_string();

                if is_repeated_twice(&s) {
                    sequence_repeated_twice += num;
                }

                if is_repeated_multiple_times(&s) {
                    sequence_repeated_multiple_times += num;
                }
            }
        }
    }

    println!(
        "Step n°1: Sum of sequences repeated twice: {}",
        sequence_repeated_twice
    );
    println!(
        "Step n°2: Sum of sequences repeated multiple times: {}",
        sequence_repeated_multiple_times
    );
}
