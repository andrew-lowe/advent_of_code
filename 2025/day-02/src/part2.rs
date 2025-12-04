use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashSet;

fn main() -> io::Result<()>{
    let file = File::open("part1_test.txt")?;
    let reader = BufReader::new(file);

    let mut invalid_id_total: i64 = 0;

    for line in reader.lines() {
        let line = line?;
        
        let ranges: Vec<&str> = line.split(",").collect();

        for range in ranges {
            let parts: Vec<&str> = range.split("-").collect();

            let mut lower_int: i64 = parts[0].parse().unwrap();
            let upper_int: i64 = parts[1].parse().unwrap();

            while lower_int <= upper_int {
                let lower_str = lower_int.to_string();
                let is_invalid_id: bool = check_id_validity(lower_str);
                if is_invalid_id {
                    invalid_id_total += lower_int;
                }
                lower_int += 1;
            }
        }
    }
    println!("total: {}", invalid_id_total);

    Ok(())
}

fn check_id_validity(num_str: String) -> bool {
    // loop n to num.len() - 1
        // check if divisible by n
        // if true, check if duplicated all parts
            // if true, return true
    let mut chunks_size = 1;
    let num_len = num_str.len();

    while chunks_size <= (num_len / 2) {
        if num_len % chunks_size == 0 {
            let chunks: Vec<&str> = num_str.as_bytes()
                                           .chunks(chunks_size)
                                           .map(|chunk| std::str::from_utf8(chunk)
                                           .unwrap()).collect();
            let chunks_set: HashSet<&str> = chunks.into_iter().collect();

            if chunks_set.len() == 1 {
                println!("invalid id {}", num_str);
                return true
            }


            
        }

        chunks_size += 1
    }
    return false
}

fn count_digits(mut n: i64) -> i64 {
    if n == 0 {
        return 1
    }

    let mut count = n.abs();

    while n > 0 {
        n /= 10;
        count += 1
    }

    return count;

}