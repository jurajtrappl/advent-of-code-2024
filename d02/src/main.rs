#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn are_levels_monotonic(report: &[i32]) -> bool {
    let is_all_increasing = report.windows(2).all(|w| w[0] < w[1]);
    let is_all_decreasing = report.windows(2).all(|w| w[0] > w[1]);
    is_all_increasing || is_all_decreasing
}

fn are_adjacent_levels_difference_ok(report: &[i32]) -> bool {
    report.windows(2).all(|window| {
        let diff = window[0].abs_diff(window[1]);
        diff >= 1 && diff <= 3
    })
}

fn is_report_safe_fst_part(report: &[i32]) -> bool {
    are_levels_monotonic(report) && are_adjacent_levels_difference_ok(report)
}

fn is_report_safe_snd_part(report: &[i32]) -> bool {
    if is_report_safe_fst_part(report) {
        return true;
    }

    for i in 0..report.len() {
        let report_without_ith = [&report[..i], &report[i + 1..]].concat();
        if are_levels_monotonic(&report_without_ith)
            && are_adjacent_levels_difference_ok(&report_without_ith)
        {
            return true;
        }
    }

    return false;
}

fn solve(input: &str, report_safe_predicate: fn(&[i32]) -> bool) -> usize {
    let reports: Vec<Vec<i32>> = aoe::parse_to_2dvec(input);
    reports
        .iter()
        .filter(|report| report_safe_predicate(&report))
        .count()
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    println!(
        "First part: {}. Second part: {}",
        solve(&input, is_report_safe_fst_part),
        solve(&input, is_report_safe_snd_part)
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_are_levels_monotonic() {
        let test_cases = vec![
            (vec![1, 4, 11], true),
            (vec![9, 7, 1], true),
            (vec![3, 6, 7, 8, 2, 1], false),
            (vec![8, 6, 4, 4, 1], false),
        ];

        for (input, expected) in test_cases.into_iter() {
            assert_eq!(expected, super::are_levels_monotonic(&input));
        }
    }

    #[test]
    fn test_are_adjacent_levels_difference_ok() {
        let test_cases = vec![
            (vec![1, 4, 11], false),
            (vec![9, 7, 1], false),
            (vec![3, 6, 7, 8, 5, 2], true),
        ];

        for (input, expected) in test_cases.into_iter() {
            assert_eq!(expected, super::are_adjacent_levels_difference_ok(&input));
        }
    }

    #[test]
    fn test_solve_example_fst_part() {
        let input = aoe::read_input_file("example_input").unwrap();
        assert_eq!(2, super::solve(&input, super::is_report_safe_fst_part));
    }

    #[test]
    fn test_solve_fst_part() {
        let input = aoe::read_input_file("input").unwrap();
        assert_eq!(332, super::solve(&input, super::is_report_safe_fst_part));
    }

    #[test]
    fn test_solve_example_snd_part() {
        let input = aoe::read_input_file("example_input").unwrap();
        assert_eq!(4, super::solve(&input, super::is_report_safe_snd_part));
    }

    #[test]
    fn test_solve_snd_part() {
        let input = aoe::read_input_file("input").unwrap();
        assert_eq!(398, super::solve(&input, super::is_report_safe_snd_part));
    }
}
