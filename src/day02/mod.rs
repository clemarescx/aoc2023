use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

pub fn main() {
    let sum = part_one(INPUT);
    println!("valid game id sum: {sum}");
    let sum = part_two(INPUT);
    println!("powers sum: {sum}");
}

fn part_two(input: &str) -> u32 {
    let games = parse_games(input);
    let cube_powers = find_minimum_powers(&games);

    cube_powers.iter().sum()
}

fn find_minimum_powers(games: &[Game]) -> Vec<u32> {
    let mut powers = Vec::new();
    for game in games {
        let mut min_counts: HashMap<&String, u32> = HashMap::new();
        for reveal in &game.cube_draws {
            for (color, count) in &reveal.colors {
                let entry = min_counts.entry(color).or_insert(0);
                *entry = (*entry).max(*count);
            }
        }
        let power = min_counts.values().product();
        powers.push(power);
    }
    powers
}

fn part_one(input: &str) -> u32 {
    let constraint: HashMap<String, u32> = [
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]
    .into();
    let games = parse_games(input);
    let valid_games = find_valid_games(&games, &constraint);
    valid_games.iter().map(|g| g.id).sum()
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(parse_line).collect()
}

fn find_valid_games<'a>(games: &'a [Game], constraint: &'a HashMap<String, u32>) -> Vec<&'a Game> {
    let mut valid = Vec::new();
    for game in games {
        if is_valid_game(game, constraint) {
            valid.push(game);
        }
    }
    valid
}

fn is_valid_game(game: &Game, constraints: &HashMap<String, u32>) -> bool {
    for trekk in &game.cube_draws {
        for (color, count) in &trekk.colors {
            let constraint = constraints.get(color);
            match constraint {
                Some(c) if c >= count => {}
                _ => return false,
            }
        }
    }
    true
}

fn parse_line(line: &str) -> Game {
    let (game_id, cube_draws) = line.split_once(':').unwrap();
    let (_, id_str) = game_id.split_once(' ').unwrap();
    let cube_draw_sets = cube_draws.split(':');
    let mut cube_draws = Vec::new();
    for reveal in cube_draw_sets {
        let sets = reveal.split(';');
        for set in sets {
            let cubes = set.split(',');
            let mut colors = HashMap::new();
            for cube in cubes {
                let (cube_count, cube_color) = cube.trim().split_once(' ').unwrap();
                colors.insert(cube_color.to_owned(), cube_count.parse().unwrap());
            }
            let reveal = CubeDraw { colors };
            cube_draws.push(reveal);
        }
    }

    Game {
        id: id_str.parse().unwrap(),
        cube_draws,
    }
}

struct Game {
    id: u32,
    cube_draws: Vec<CubeDraw>,
}

struct CubeDraw {
    colors: HashMap<String, u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let sum = part_one(input);
        assert_eq!(sum, 8)
    }

    #[test]
    fn part_two_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let sum = part_two(input);
        assert_eq!(sum, 2286)
    }
}
