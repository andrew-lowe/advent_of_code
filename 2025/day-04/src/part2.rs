use std::collections::VecDeque;



fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let grid: Vec<Vec<u8>> = input.lines()
        .map(|line| {
            line.chars()
                .map(|ch| (ch == '@') as u8)
                .collect()
        })
        .collect();
    return grid;
}

fn build_neighbor_count_grid(grid: Vec<Vec<u8>>) -> (Vec<Vec<u8>>, VecDeque<(usize, usize)>) {
    let mut neighbor_count_grid: Vec<Vec<u8>> = vec![];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    for (i, row) in grid.iter().enumerate() {
        let mut neighbor_count_row: Vec<u8> = vec![];

        for (j, roll) in row.iter().enumerate() {
            let mut neighbor_count = 0;
            if *roll == 0 {
                neighbor_count_row.push(0);
                continue;
            }
            // above
            if i > 0 {
                neighbor_count += grid[i-1][j];
            }
            // under
            if i < grid.len() - 1 {
                neighbor_count += grid[i+1][j];
            }
            // left
            if j > 0 {
                neighbor_count += grid[i][j-1];
            }
            // right
            if j < row.len() - 1 {
                neighbor_count += grid[i][j+1];
            }
            // above left
            if i > 0 && j > 0 {
                neighbor_count += grid[i-1][j-1];
            }
            // above right
            if i > 0 && j < row.len() - 1 {
                neighbor_count += grid[i-1][j+1];
            }
            // under left
            if i < grid.len() - 1 && j > 0 {
                neighbor_count += grid[i+1][j-1];
            }
            // under right
            if i < grid.len() - 1 && j < row.len() - 1 {
                neighbor_count += grid[i+1][j+1];
            }
            neighbor_count_row.push(neighbor_count);
            if neighbor_count < 4 {
                queue.push_back((i, j));
            }
        }
        neighbor_count_grid.push(neighbor_count_row);
    }
    return (neighbor_count_grid, queue);
}


#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    // queue = [] - of count < 4 rolls
    // scan entire input (steal code from part 1) building a map of 
        // build 2d array [[]]
    // go through queue removing. 
    // as remoove, update array.
    // if count < 4, add to queue.
    let mut sum: u32 = 0;

    let initial_grid = parse_input(_input);

    let (mut neighbor_count_grid, mut queue) = build_neighbor_count_grid(initial_grid);

    while let Some((i, j)) = queue.pop_front() {
        // decrement neighbors
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 { continue; }
                let Some(ni) = i.checked_add_signed(di) else { continue };
                let Some(nj) = j.checked_add_signed(dj) else { continue };

                if let Some(val) = neighbor_count_grid.get_mut(ni).and_then(|row| row.get_mut(nj)) {
                    if *val == 0 { continue; };
                    *val -= 1;
                    if *val < 4 && !queue.contains(&(ni, nj)) {
                        queue.push_back((ni, nj));
                    }
                }
            }
        }
        neighbor_count_grid[i][j] = 0;
        sum += 1;
    }
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
        assert_eq!("43", process(input)?);
        Ok(())
    }
}
