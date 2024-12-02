#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn are_levels_consistent(report: &[i32]) -> bool {
    if report.len() == 1 {
        return true;
    }

    let is_increasing = report[0] < report[1];
    report.windows(2).all(|window| {
        if is_increasing {
            window[0] < window[1]
        } else {
            window[0] > window[1]
        }
    })
}

fn is_adjacent_levels_difference_ok(report: &[i32]) -> bool {
    report.windows(2).all(|window| {
        let diff = window[0].abs_diff(window[1]);
        diff >= 1 && diff <= 3
    })
}

fn is_report_safe(report: &[i32]) -> bool {
    are_levels_consistent(report) && is_adjacent_levels_difference_ok(report)
}

fn solve(input: &str) -> usize {
    let reports: Vec<Vec<i32>> = aoe::parse_to_2dvec(input);
    reports.iter().filter(|r| is_report_safe(r)).count()
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    println!("{}", solve(&input))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_are_levels_consistent() {
        let test_cases = vec![
            (vec![1, 4, 11], true),
            (vec![9, 7, 1], true),
            (vec![3, 6, 7, 8, 2, 1], false),
        ];

        for (input, expected) in test_cases.into_iter() {
            assert_eq!(expected, super::are_levels_consistent(&input));
        }
    }

    #[test]
    fn test_is_adjacent_levels_difference_ok() {
        let test_cases = vec![
            (vec![1, 4, 11], false),
            (vec![9, 7, 1], false),
            (vec![3, 6, 7, 8, 5, 2], true),
        ];

        for (input, expected) in test_cases.into_iter() {
            assert_eq!(expected, super::is_adjacent_levels_difference_ok(&input));
        }
    }

    #[test]
    fn test_solve_example() {
        let input = aoe::read_input_file("example_input").unwrap();
        assert_eq!(2, super::solve(&input));
    }

    #[test]
    fn test_solve_fst_part() {
        let input = aoe::read_input_file("input").unwrap();
        assert_eq!(332, super::solve(&input));
    }
}
