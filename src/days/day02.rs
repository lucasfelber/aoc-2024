use itertools::Itertools;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(|line| {
        line.split_whitespace().map(|number| {
            number.parse().unwrap()
        }).collect()
    }).collect()
}

pub fn is_safe(report: Vec<i64>) -> bool {
    let mut safe = false;
    safe |= report.iter().tuple_windows().all(|(a, b)| a < b);
    safe |= report.iter().tuple_windows().all(|(a, b)| a > b);
    safe && report.iter().tuple_windows().all(|(a, b)| (1..=3).contains(&(a - b).abs()))
}

pub fn is_safe_pd(report: Vec<i64>) -> bool {
    report.iter().enumerate().any(|(index, _)| {
        let mut temp_report = report.to_vec();
        temp_report.remove(index);
        is_safe(temp_report)
    })
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Vec<i64>>) -> u32 {
    input.iter().fold(0, |acc, report| {
        if is_safe(report.to_vec()) { acc + 1 }
        else { acc }
    })
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Vec<i64>>) -> u32 {
    input.iter().fold(0, |acc, report| {
        if is_safe_pd(report.to_vec()) {acc + 1 }
        else { acc }
    })
}