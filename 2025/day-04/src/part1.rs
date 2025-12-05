use std::collections::HashMap;

fn process_prev_line(line_map: &HashMap<usize, u32>) -> u32 {
    let mut sum = 0;
    for v in line_map.values() {
        if *v < 4u32 {
            sum += 1;
        }
    }

    sum
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut prev_line_map: HashMap<usize, u32> = HashMap::new();
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut cur_line_map: HashMap<usize, u32> = HashMap::new();

        for (j, ch) in line.chars().enumerate() {

            if ch == '@' {
                cur_line_map.entry(j).or_insert(0);

                if j > 0 {
                    if prev_line_map.contains_key(&(j-1)) {
                        *cur_line_map.entry(j).or_insert(0) += 1;
                        *prev_line_map.entry(j-1).or_insert(0) += 1;
                    }
                    if cur_line_map.contains_key(&(j-1)) {
                        *cur_line_map.entry(j).or_insert(0) += 1;
                        *cur_line_map.entry(j-1).or_insert(0) += 1;
                    }
                }
                if prev_line_map.contains_key(&j) {
                    *cur_line_map.entry(j).or_insert(0) += 1;
                    *prev_line_map.entry(j).or_insert(0) += 1;
                }
                if prev_line_map.contains_key(&(j+1)) {
                    *cur_line_map.entry(j).or_insert(0) += 1;
                    *prev_line_map.entry(j+1).or_insert(0) += 1;
                }

            }
        }
        sum += process_prev_line(&prev_line_map);
        prev_line_map = cur_line_map;
    }
    sum += process_prev_line(&prev_line_map);
    Ok(sum.to_string())
}  

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!("13", process(input)?);
        Ok(())
    }
}
