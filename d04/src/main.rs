#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[derive(Debug)]
enum Diagonal {
    Strings(String),
    StringsWithPositions((String, (usize, usize))),
}

fn diagonals(grid: &[&str], length: usize, with_positions: Option<bool>) -> Vec<Diagonal> {
    let with_positions = with_positions.unwrap_or(false);
    let grid: Vec<Vec<char>> = grid.iter().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut diags: Vec<Diagonal> = Vec::new();

    // from top left towards bottom right
    for row in 0..rows - length + 1 {
        for col in 0..cols - length + 1 {
            let mut positions = vec![];
            for i in 0..length {
                positions.push((row + i, col + i));
            }

            let mut diag = String::new();
            for &(r, c) in positions.iter() {
                diag.push(grid[r][c]);
            }

            if with_positions {
                diags.push(Diagonal::StringsWithPositions((
                    diag,
                    positions[length / 2],
                )));
            } else {
                diags.push(Diagonal::Strings(diag));
            }
        }
    }

    // from bottom left to top right
    for row in (length - 1..rows).rev() {
        for col in (0..cols - length + 1).rev() {
            let mut positions = vec![];
            for i in 0..length {
                positions.push((row - i, col + i));
            }

            let mut diag = String::new();
            for &(r, c) in positions.iter() {
                diag.push(grid[r][c]);
            }

            if with_positions {
                diags.push(Diagonal::StringsWithPositions((
                    diag,
                    positions[length / 2],
                )));
            } else {
                diags.push(Diagonal::Strings(diag));
            }
        }
    }

    diags
}

fn solve_fst_part(word_map: &[&str]) -> usize {
    let mut counter = 0;
    for line in word_map.iter() {
        let horizontal: Vec<_> = line.match_indices("XMAS").collect();
        let backwards_horizontal: Vec<_> = line.match_indices("SAMX").collect();

        counter += horizontal.len() + backwards_horizontal.len()
    }

    let word_map_chars: Vec<Vec<char>> = word_map.iter().map(|l| l.chars().collect()).collect();
    let word_map_transposed: Vec<String> = aoe::transpose(&word_map_chars)
        .iter()
        .map(|l| String::from_iter(l))
        .collect();

    for line in word_map_transposed.iter() {
        let vertical: Vec<_> = line.match_indices("XMAS").collect();
        let backwards_vertical: Vec<_> = line.match_indices("SAMX").collect();

        counter += vertical.len() + backwards_vertical.len();
    }

    let diags = diagonals(word_map, 4, None);
    counter + diags
        .iter()
        .filter(|w| match w {
            Diagonal::Strings(word) => *word == "XMAS" || *word == "SAMX",
            _ => false,
        })
        .count()
}

fn solve_snd_part(word_map: &[&str]) -> usize {
    let diagonals_pos = diagonals(word_map, 3, Some(true));
    let mut counter_snd = 0;
    let mut mas_sam_diags = Vec::new();
    for d in diagonals_pos {
        if let Diagonal::StringsWithPositions((value, pos)) = d {
            if value == "MAS" || value == "SAM" {
                mas_sam_diags.push((value, pos));
            }
        }
    }

    for (i, &(_, (x1, y1))) in mas_sam_diags.iter().enumerate() {
        for &(_, (x2, y2)) in mas_sam_diags[i + 1..].iter() {
            if x1 == x2 && y1 == y2 {
                counter_snd += 1;
            }
        }
    }

    counter_snd
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let word_map: Vec<_> = input.split("\n").collect();

    let counter_fst = solve_fst_part(&word_map);
    let counter_snd = solve_snd_part(&word_map);

    println!("First part: {}, Second part: {}", counter_fst, counter_snd);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_fst_part() {
        let input = aoe::read_input_file("input").unwrap();
        let example_input = aoe::read_input_file("example_input").unwrap();
        let input_word_map: Vec<_> = input.split("\n").collect();
        let example_input_word_map: Vec<_> = example_input.split("\n").collect();

        assert_eq!(18, super::solve_fst_part(&example_input_word_map));
        assert_eq!(2633, super::solve_fst_part(&input_word_map));
    }

    #[test]
    fn test_solve_snd_part() {
        let input = aoe::read_input_file("input").unwrap();
        let example_input = aoe::read_input_file("example_input").unwrap();
        let input_word_map: Vec<_> = input.split("\n").collect();
        let example_input_word_map: Vec<_> = example_input.split("\n").collect();

        assert_eq!(9, super::solve_snd_part(&example_input_word_map));
        assert_eq!(1936, super::solve_snd_part(&input_word_map));
    }
}