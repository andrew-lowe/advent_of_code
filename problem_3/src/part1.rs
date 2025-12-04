use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("input_part1.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line: String = line?;
        println!("{}", line);

        sum += find_largest_batteries(line);

    }

    println!("total {}", sum);
    
    Ok(())
}

fn find_largest_batteries(line: String) -> u32 {

    let mut first_val = 0;
    let mut second_val = 0;
    for (i,n) in line.chars().enumerate() {
        let n_int = n.to_digit(10).unwrap();
        if i != line.len() - 1 && n_int > first_val {
            first_val = n_int;
            second_val = 0;
        } else if n_int > second_val {
            second_val = n_int;
        }
    }

    return (first_val * 10) + second_val
}