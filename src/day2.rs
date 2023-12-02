use std::collections::HashMap;

#[aoc(day2, part1, Chars)]
pub fn part1_chars(input: &str) -> i32 {
    let max_cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    input.lines().fold(0, |sum, line| {
        let mut parts = line.split(":");
        let id = parts
            .next()
            .unwrap()
            .split(" ")
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        sum + if parts.next().unwrap().split(";").any(|picked| {
            picked.split(",").any(|mut color| {
                color = color.trim();
                let mut parts = color.split(" ");
                let count = parts.next().unwrap().parse::<i32>().unwrap();
                let color = parts.next().unwrap();
                if let Some(v) = max_cubes.get(color) {
                    count > *v
                } else {
                    println!("WARNING: key not found: {}", color);
                    false
                }
            })
        }) {
            0
        } else {
            id
        }
    })
}

#[aoc(day2, part2, Chars)]
fn part2_chars(input: &str) -> i32 {
    input.lines().fold(0, |sum, line| {
        let picks = line
            .split(":")
            .last()
            .unwrap()
            .split(";")
            .map(|e| {
                e.split(",")
                    .map(|mut f| {
                        f = f.trim();
                        let mut split = f.split(" ");
                        (split.next().unwrap(), split.next().unwrap())
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let max_green = picks
            .iter()
            .filter_map(|e| {
                e.iter().find_map(|f| {
                    if f.1 == "green" {
                        Some(f.0.parse::<i32>().unwrap())
                    } else {
                        None
                    }
                })
            })
            .max()
            .unwrap_or(1);
        let max_red = picks
            .iter()
            .filter_map(|e| {
                e.iter().find_map(|f| {
                    if f.1 == "red" {
                        Some(f.0.parse::<i32>().unwrap())
                    } else {
                        None
                    }
                })
            })
            .max()
            .unwrap_or(1);
        let max_blue = picks
            .iter()
            .filter_map(|e| {
                e.iter().find_map(|f| {
                    if f.1 == "blue" {
                        let parsed = f.0.parse::<i32>().unwrap();
                        Some(parsed)
                    } else {
                        None
                    }
                })
            })
            .max()
            .unwrap_or(1);

        sum + (max_green * max_red * max_blue)
    })
}

#[cfg(test)]
mod tests {}
