#[derive(Clone, Copy, Debug)]
enum Visibility {
    Visible,
    Invisible,
}

type VisGraph = Vec<Vec<Visibility>>;

fn map_visibility_values(vis: &Visibility) -> usize {
    use crate::day8::Visibility::*;
    match vis {
        Visible => 1,
        Invisible => 0,
    }
}

fn make_graph(input: &str) -> VisGraph {
    use crate::day8::Visibility::*;

    let intermediate: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|c| !c.is_empty())
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    intermediate
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            let row_len = row.len();
            if idx == 0 {
                vec![Visible; row_len]
            } else if idx == (intermediate.len() - 1) {
                vec![Visible; row_len]
            } else {
                row.iter()
                    .enumerate()
                    .map(|(col, _num)| {
                        if col == 0 || col == (row_len - 1) {
                            Visible
                        } else {
                            // look as far left, right, up, down
                            // ensure this height > all of those
                            let curr_value = intermediate[idx][col];
                            let left = (0..col)
                                .map(|x| intermediate[idx][x])
                                .all(|left| left < curr_value);
                            let right = (col + 1..row_len)
                                .map(|x| intermediate[idx][x])
                                .all(|right| right < curr_value);

                            let above = (0..idx)
                                .map(|y| intermediate[y][col])
                                .all(|above| above < curr_value);

                            let below = (idx + 1..intermediate.len())
                                .map(|y| intermediate[y][col])
                                .all(|below| below < curr_value);

                            if above || below || left || right {
                                Visible
                            } else {
                                Invisible
                            }
                        }
                    })
                    .collect::<Vec<Visibility>>()
            }
        })
        .collect()
}

fn sum_graph_visibility(graph: VisGraph) -> usize {
    graph
        .iter()
        .map(|row| {
            row.iter()
                .map(map_visibility_values)
                .collect::<Vec<usize>>()
        })
        .flatten()
        .sum::<usize>()
}

fn part1(input: &str) -> usize {
    let graph = make_graph(input);
    sum_graph_visibility(graph)
}

pub fn run() -> anyhow::Result<()> {
    let input = include_str!("../data/day8.txt");
    let pt1 = part1(input);
    println!("day 8 part 1 {pt1}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day8::*;
    #[test]
    fn pt1_works_on_test() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(21, part1(input));
    }
}
