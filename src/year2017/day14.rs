use super::day10::knot_hash;

static KEY: &str = "xlqgujun";

pub fn solve() -> (u32, u32) {
    (part1(), part2())
}

fn part1() -> u32 {
    (0..128)
        .map(|row| {
            knot_hash(&format!("{}-{}", KEY, row))
                .iter()
                .map(|x| x.count_ones())
                .sum::<u32>()
        })
        .sum()
}

fn part2() -> u32 {
    let mut grid: Vec<Vec<_>> = (0..128)
        .map(|row| {
            knot_hash(&format!("{}-{}", KEY, row))
                .iter()
                .flat_map(|x| {
                    format!("{:08b}", x)
                        .bytes()
                        .map(|b| b == b'1')
                        .collect::<Vec<_>>()
                })
                .collect()
        })
        .collect();

    let mut regions = 0;
    let mut stack = vec![];
    for i in 0..128 {
        for j in 0..128 {
            if grid[i][j] {
                regions += 1;
                stack.extend(neighbors(i, j));
                while let Some((r, c)) = stack.pop() {
                    if grid[r][c] {
                        grid[r][c] = false;
                        stack.extend(neighbors(r, c));
                    }
                }
            }
        }
    }

    regions
}

fn neighbors(r: usize, c: usize) -> Vec<(usize, usize)> {
    let mut n = vec![];
    if r != 0 {
        n.push((r - 1, c));
    }
    if r < 127 {
        n.push((r + 1, c));
    }
    if c != 0 {
        n.push((r, c - 1));
    }
    if c < 127 {
        n.push((r, c + 1));
    }
    n
}

#[cfg(test)]
#[test]
fn ans() {
    assert_eq!(solve(), (8204, 1089));
}
