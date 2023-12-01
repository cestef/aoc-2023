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

#[cfg(test)]
mod tests {}
