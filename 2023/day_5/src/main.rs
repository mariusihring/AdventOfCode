use regex::Regex;

struct Solution;

impl Solution {
    pub fn prepare() -> Vec<Map> {
        let map_regex =
            Regex::new(r#"(\w+ map:)\s*((\d+\s+){2}\d+\s*)+"#).expect("Failed to build regex");
        let content: Vec<&str> = include_str!("./inputs/test_input")
            .split("\r\n\r\n")
            .collect();

        content
            .iter()
            .map(|map| {
                let mut name = String::new();
                let mut ranges: Vec<Range> = Vec::new();
                map.lines().enumerate().for_each(|(index, line)| {
                    if index == 0 {
                        name = line.replace(":", "")
                    } else {
                        let nums: Vec<&str> = line.split(" ").collect();
                        ranges.push(Range {
                            dest_start: nums[0].parse::<usize>().unwrap(),
                            source_start: nums[1].parse::<usize>().unwrap(),
                            range: nums[2].parse::<usize>().unwrap(),
                        })
                    }
                });
                Map {
                    name: name.to_owned(),
                    ranges,
                }
            })
            .collect::<Vec<Map>>()
    }

    pub fn solve_first() {
        let maps = Self::prepare();

        let seeds: Vec<usize> = maps[0].name.split(" ").map()
    }
}

fn main() {
    Solution::prepare();
}

#[derive(Debug)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}
#[derive(Debug)]
struct Range {
    dest_start: usize,
    source_start: usize,
    range: usize,
}
