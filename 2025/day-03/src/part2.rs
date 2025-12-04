use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("input_part2.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line: String = line?;

        sum += find_largest_batteries(line);

    }
    println!("{}", sum);
    Ok(())
}

fn find_largest_batteries(line: String) -> u64 {
    // 9 3 8 2 8 2 5 9 1 2 9 3 8 2 0 1
    // ^     ^
    // i
    // l = []
    // while i <= line_len:
    //      char = line[i]
    //      last_eligible_idx = line_len - 1 - (12 - l.len()) << check this
    // .    for inner_idx, v in enumerate(line):
    // .        if v > next_val:
    //              next_val = v
    //              i = inner_idx
    //      l.append(v)
    //      i = inner_idx + 1
    // 

    let mut l: Vec<u32> = Vec::new();
    let mut i: usize = 0;
    let line_len: usize = line.len(); // 16 - 12 = 4
    let line_bytes = line.as_bytes();

    while l.len() < 12 {
        let last_eligible_idx = line_len - (12 - l.len());
        let mut next_val: u32 = 0;
        let mut best_idx = i;
        for inner_idx in i..=last_eligible_idx {
            let v = (line_bytes[inner_idx] - b'0') as u32;
            if v > next_val {
                next_val = v;
                best_idx = inner_idx;
            }
        }
        l.push(next_val);
        i = best_idx + 1;
    }
    let mut sum: u64 = 0;
    for (i, v) in l.iter().enumerate() {
        let power = 11 - i as u32;  // 11, 10, 9, ... 0
        sum += *v as u64 * 10u64.pow(power);
    }
    return sum;
}