use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut dial: i32 = 50;
    let mut count: i32 = 0;

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);

        dial = process_move(line, dial);
        if dial == 0 {
            count += 1;
        }
        println!("dial: {}; count {}", dial, count);

    }

    println!("{}", count);
    Ok(())


}

fn process_move(line: String, dial: i32) -> i32 {
    let rotation: i32 = if line.starts_with("L") {
        -get_rotation(line).unwrap()
    } else if line.starts_with("R") {
        get_rotation(line).unwrap()
    } else {
        panic!("AHH!");
    };

    return get_new_position(dial, rotation);
}

fn get_new_position(dial: i32, rotation: i32) -> i32 {
    let mut raw_position = dial + rotation;
    raw_position = raw_position.rem_euclid(100);

    if raw_position >= 0 {
        return raw_position % 100;
    } else {
        return 100 - raw_position.abs();
    }
}

fn get_rotation(mut line: String) -> Result<i32, std::num::ParseIntError> {
    line.remove(0);
    return line.parse();
}