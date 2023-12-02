#[aoc(day1, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    input.lines().fold(0, |sum, line| {
        let digits = line.chars().filter(|x| x.is_numeric()).collect::<Vec<_>>();
        sum + format!(
            "{}{}",
            digits.first().unwrap(),
            digits.last().unwrap_or(digits.first().unwrap())
        )
        .parse::<i32>()
        .unwrap()
    })
}

const CANDIDATES: [(&str, i32); 18] = [
    ("one", 1),
    ("1", 1),
    ("two", 2),
    ("2", 2),
    ("three", 3),
    ("3", 3),
    ("four", 4),
    ("4", 4),
    ("five", 5),
    ("5", 5),
    ("six", 6),
    ("6", 6),
    ("seven", 7),
    ("7", 7),
    ("eight", 8),
    ("8", 8),
    ("nine", 9),
    ("9", 9),
];

#[aoc(day1, part2, Chars)]
pub fn part2_chars(input: &str) -> i32 {
    input.lines().fold(0, |sum, mut line| {
        line = line.trim();
        let first_digit = CANDIDATES
            .iter()
            .filter_map(|(predicate, numeral)| line.find(predicate).map(|index| (index, numeral)))
            .min_by_key(|(index, _)| *index)
            .map(|(_, numeral)| numeral)
            .unwrap();
        let last_digit = CANDIDATES
            .iter()
            .filter_map(|(predicate, numeral)| line.rfind(predicate).map(|index| (index, numeral)))
            .max_by_key(|(index, _)| *index)
            .map(|(_, numeral)| numeral)
            .unwrap();
        let number = format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .unwrap();
        sum + number
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn sample1() {
        assert_eq!(
            crate::day1::part2_chars(
                r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#
            ),
            281
        )
    }
}
