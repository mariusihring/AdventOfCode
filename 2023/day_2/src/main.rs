struct Solution;

impl Solution {
    pub fn prepare() -> Vec<Game> {
        let input = include_str!("inputs/input").lines();
        input
            .map(|game| {
                let (id, info) = game.split_once(": ").unwrap();
                let id = id.split_once(" ").unwrap().1.parse::<u32>().unwrap();
                let cubes = info
                    .split("; ")
                    .map(|cube_info| {
                        let mut cube = Cubes::default();
                        for cube_str in cube_info.split(", ") {
                            let (value, color) = cube_str.split_once(" ").unwrap();
                            match color {
                                "red" => cube.red += value.parse::<u32>().unwrap(),
                                "green" => cube.green += value.parse::<u32>().unwrap(),
                                "blue" => cube.blue += value.parse::<u32>().unwrap(),
                                _ => panic!("Invalid color: {}", color),
                            }
                        }
                        cube
                    })
                    .collect();

                Game { id, cubes: cubes }
            })
            .collect()
    }

    pub fn part_1() {
        let games = Self::prepare();
        let result: u32 = games
            .into_iter()
            .filter(|game| {
                !game
                    .cubes
                    .iter()
                    .any(|cubes| cubes.red > 12 || cubes.green > 13 || cubes.blue > 14)
            })
            .map(|game| game.id)
            .sum();

        println!("part 1: {}", result)
    }

    pub fn part_2() {
        let games = Self::prepare();
        let result: u32 = games
            .into_iter()
            .map(|game| {
                let mut green = 0;
                let mut red = 0;
                let mut blue = 0;
                game.cubes.iter().for_each(|cube| {
                    if cube.red > red {
                        red = cube.red
                    }
                    if cube.blue > blue {
                        blue = cube.blue
                    }
                    if cube.green > green {
                        green = cube.green
                    }
                });
                green * red * blue
            })
            .sum();

        println!("part 2: {}", result)
    }
}

pub fn main() {
    Solution::part_1();
    Solution::part_2();
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<Cubes>,
}

#[derive(Default, Debug)]
struct Cubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
