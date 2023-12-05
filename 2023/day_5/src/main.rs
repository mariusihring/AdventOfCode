use std::ops::Range;

struct Solution;

impl Solution {
    pub fn prepare() -> PrepareResponse {
        let input = include_str!("./inputs/input");
        let mut split = input.split("\n\n");

        let mut seeds: Vec<usize> = split
            .next()
            .unwrap()
            .strip_prefix("seeds: ")
            .unwrap()
            .split_whitespace()
            .map(|seed| seed.parse().unwrap())
            .collect();

        let maps: Vec<Vec<(Range<usize>, Range<usize>)>> = split
            .map(|map| {
                map.lines()
                    .skip(1)
                    .map(|range| {
                        let nums = range
                            .split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect::<Vec<usize>>();
                        (nums[1]..(nums[1] + nums[2]), nums[0]..(nums[0] + nums[2]))
                    })
                    .collect::<Vec<(Range<usize>, Range<usize>)>>()
            })
            .collect();

        PrepareResponse {seeds, maps}
    }

    pub fn solve_first() {
        let mut prepare = Self::prepare();
        for map in prepare.maps {
            for seed in prepare.seeds.iter_mut() {
                *seed = map.iter().find_map(|m| {
                    if m.0.start <= *seed && m.0.end > *seed {
                        Some(m.1.start + *seed - m.0.start)
                    } else {
                        None
                    }
                }).unwrap_or(*seed)
            }

        }
        let smallest = prepare.seeds.iter().min().copied().unwrap();

        println!("part 1 : {:?}", smallest)
    }
}

fn main() {
   Solution::solve_first()
}

#[derive(Debug)]
struct PrepareResponse {
    seeds: Vec<usize>,
    maps: Vec<Vec<(Range<usize>, Range<usize>)>>
}
