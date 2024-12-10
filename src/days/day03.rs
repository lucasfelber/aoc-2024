use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let regex = Regex::new(r"mul\((?<num1>\d+),(?<num2>\d+)\)").unwrap();
    regex.captures_iter(input).map(|caps| {
        caps["num1"].parse::<usize>().unwrap() * caps["num2"].parse::<usize>().unwrap()
    }).sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let regex = Regex::new(r"mul\(\d+,\d+\)|do(n't)?\(\)").unwrap();
    let mut ok = true;
    regex.find_iter(input).fold(0, |acc, op| {
        match op.as_str() {
            "do()" => {
                ok = true;
                acc
            }
            "don't()" => {
                ok = false;
                acc
            }
            mul => {
                let (a, b) = mul[4..mul.len()-1].split_once(",").unwrap();
                if ok {
                    acc + a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
                } else {
                    acc
                }
            }
        }
    })
}