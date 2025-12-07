fn within_range(start: i64, end: i64, num: i64) -> bool {
    num >= start && num <= end
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let mut sum = 0;
    let mut fresh_ranges = Vec::<(i64, i64)>::new();
    let mut check_ingredients: bool = false;

    for line in input.lines() {
        if line.is_empty() {
            check_ingredients = true;
            continue;
        }

        if !check_ingredients {
            let parts = line.split("-").collect::<Vec<&str>>();
            let nstart: i64 = parts[0].parse().unwrap();
            let nend: i64 = parts[1].parse().unwrap();
            let mut inserted = false;
            let mut idx = 0;
            while idx < fresh_ranges.len() && !inserted{
                let (start, end) = fresh_ranges[idx];
                if within_range(nstart, nend, start) && within_range(nstart, nend, end) {
                    println!("1: nstart: {}, nend: {}, start: {}, end: {}", nstart, nend, start, end);
                    fresh_ranges[idx] = (nstart, nend);
                    inserted = true;
                    break;
                } else if within_range(nstart, nend, start) && nend >= end {
                    println!("2: nstart: {}, nend: {}, start: {}, end: {}", nstart, nend, start, end);
                    fresh_ranges[idx] = (start, nend);
                    inserted = true;
                    break;
                } else if nstart <= start && within_range(nstart, nend, end) {
                    println!("3: nstart: {}, nend: {}, start: {}, end: {}", nstart, nend, start, end);
                    fresh_ranges[idx] = (nstart, end);
                    inserted = true;
                    break;
                } 
                idx += 1;
            }
            if !inserted {
                println!("4: nstart: {}, nend: {}", nstart, nend);
                fresh_ranges.push((nstart, nend));
            }
        } else {
            let num: i64 = line.parse().unwrap();
            for &(start, end) in &fresh_ranges {
                println!("start: {}, end: {}, num: {}", start, end, num);
                if within_range(start, end, num) {
                    sum += 1;
                    break;
                }
            }
        }
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!("3", process(input)?);
        Ok(())
    }
}
