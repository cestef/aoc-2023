use anyhow::Result;

#[aoc(day4, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    input.lines().fold(0, |sum, mut line| {
        line = line.split(":").last().unwrap().trim();
        let mut parts = line.split("|");
        let winning = parts
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|e| !e.is_empty())
            .collect::<Vec<_>>();
        let scratched = parts
            .next()
            .unwrap()
            .trim()
            .split(" ")
            .filter(|e| !e.is_empty())
            .collect::<Vec<_>>();
        println!("{:?} {:?}", winning, scratched);
        let matching = scratched
            .iter()
            .filter(|e| winning.contains(e))
            .collect::<Vec<_>>()
            .len();

        println!("{}", matching);
        if matching == 0 {
            sum
        } else {
            sum + i32::pow(2, (matching - 1) as u32)
        }
    })
}

#[cfg(test)]
mod tests {}
