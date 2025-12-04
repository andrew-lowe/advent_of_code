use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("part1_test.txt")?;
    let reader = BufReader::new(file);

    let mut invalid_id_total: i64 = 0;

    for line in reader.lines() {
        let line = line?;
        
        let ranges: Vec<&str> = line.split(",").collect();

        for range in ranges {
            let parts: Vec<&str> = range.split("-").collect();
            println!("{:?}", parts);

            let mut lower_int: i64 = parts[0].parse().unwrap();
            let upper_int: i64 = parts[1].parse().unwrap();

            while lower_int <= upper_int {
                let is_invalid_id: bool = check_id_validity(lower_int);
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

fn check_id_validity(num: i64) -> bool {
    let num_str = num.to_string();
    let (first, second) = num_str.split_at(num_str.len() / 2);
    return first == second
}