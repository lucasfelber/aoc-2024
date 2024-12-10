#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();

    input.lines().for_each(|line| {
        let mut line_iter = line.split_whitespace();
        list1.push(line_iter.next().unwrap().parse().unwrap());
        list2.push(line_iter.next().unwrap().parse().unwrap());
    });

    (list1, list2)
}

#[aoc(day1, part1)]
pub fn part1(lists: &(Vec<i64>, Vec<i64>)) -> u64 {
    let mut lists = lists.clone();

    lists.0.sort();
    lists.1.sort();

    lists.0.iter().zip(lists.1).map(|numbers| {
        numbers.0.abs_diff(numbers.1)
    }).sum()
}

#[aoc(day1, part2)]
pub fn part2(lists: &(Vec<i64>, Vec<i64>)) -> u64 {
    lists.0.iter().map(|l1num| {
        *l1num as u64 * lists.1.iter().filter(|l2num| *l1num == **l2num).count() as u64
    }).sum()
}