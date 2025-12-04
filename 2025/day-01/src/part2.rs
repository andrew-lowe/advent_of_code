use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("part_2_test_input.txt")?;
    let reader = BufReader::new(file);

    let mut dial: i32 = 50;
    let mut ticks: i32 = 0;

    for line in reader.lines() {
        let line = line?;

        let (new_ticks, new_dial) = process_move(line, dial);
        dial = new_dial;
        ticks += new_ticks;
    }

    println!("{}", ticks);
    Ok(())


}

fn process_move(line: String, dial: i32) -> (i32, i32) {
    let mut new_ticks = 0;
    // calculate dist_to_zero
        // l: 100 - dial
        // r: (dial - 100).abs()
    // calculate rotation
    
    // calculate new_ticks
        // if left
            // if rotation > dist_to_zero:
                // new_ticks += 1
            // new_ticks += (rotation - dist_to_zero) % 100

        // if right
            // if rotation > dist_to_zero:
                // new_ticks += 1
            // new_tickks += 

    let raw_rotation = line[1..].parse::<i32>().unwrap();
    let dist_to_zero = match line.chars().next() {
        Some('L') => if dial == 0 { 100 } else { dial },
        Some('R') => if dial == 0 { 100 } else { 100 - dial },
        _ => panic!("Invalid direction"),
    };
    if raw_rotation >= dist_to_zero {
        new_ticks += 1;
    }
    new_ticks += (raw_rotation - dist_to_zero) / 100;

    let rotation: i32 = if line.starts_with("L") {
        -get_rotation(line).unwrap()
    } else if line.starts_with("R") {
        get_rotation(line).unwrap()
    } else {
        panic!("AHH!");
    };
    println!("dial: {}, raw_rotation: {}, dist_to_zero: {}, new_ticks: {}", dial, raw_rotation, dist_to_zero, new_ticks);

    let new_dial = get_new_position(dial, rotation);

    return (new_ticks, new_dial);

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