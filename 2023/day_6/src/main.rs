struct Solution;

impl Solution {
    pub fn prepare() -> Vec<Race> {
        let mut lines = include_str!("./inputs/input").lines();
        let times: Vec<usize> = lines
            .next()
            .unwrap()
            .strip_prefix("Time: ")
            .unwrap()
            .trim_start()
            .split_ascii_whitespace()
            .map(|time| time.trim_start().trim_end().parse::<usize>().unwrap())
            .collect();
        let distances: Vec<usize> = lines
            .next()
            .unwrap()
            .strip_prefix("Distance: ")
            .unwrap()
            .trim_start()
            .split_ascii_whitespace()
            .map(|time| time.trim_start().trim_end().parse::<usize>().unwrap())
            .collect();
        let mut races: Vec<Race> = Vec::new();
        for i in 0..times.len() {
            races.push(Race {
                time: times[i],
                distance: distances[i],
            })
        }

        races
    }
    pub fn prepare_second() -> Race {
        let mut lines = include_str!("./inputs/input").lines();
        let time: String = lines
            .next()
            .unwrap()
            .strip_prefix("Time: ")
            .unwrap()
            .trim_start()
            .split_ascii_whitespace()
            .map(|time| time.trim_start().trim_end())
            .collect();
        let distance: String = lines
            .next()
            .unwrap()
            .strip_prefix("Distance: ")
            .unwrap()
            .trim_start()
            .split_ascii_whitespace()
            .map(|time| time.trim_start().trim_end())
            .collect();

        Race {time: time.parse::<usize>().unwrap(), distance: distance.parse::<usize>().unwrap()}
    }
    pub fn solve_first() {
        let races = Self::prepare();

        let mut sum = 0;
        races.iter().for_each(|race| {
            let mut solutions = 0;
            for x in 1..race.time {
                let my_distance = (race.time - x) * x;
                if my_distance > race.distance {
                    solutions += 1;
                }
            }
            if sum == 0 {
                sum = solutions
            } else {
                sum *= solutions
            }
        });

        println!("part 1 : {}", sum)
    }

    pub fn solve_second() {
        let race = Self::prepare_second();
        let mut start = 0;
        let mut stop = 0;
        'distance_loop: for x in 1..race.time {
            let my_distance = (race.time - x) * x;

            if my_distance >= race.distance {
               stop = race.time - x;
                break 'distance_loop;
            }
        }
        'reverse_distance: for x in (1..race.time).rev() {
            let my_distance = (race.time - x) * x;
            if my_distance >= race.distance {
                start = race.time - x;
                break 'reverse_distance;
            }
        }

        let ways = (stop - start) + 1;
        println!("part 2 : {}", ways)

    }
}

fn main() {
    Solution::solve_first();
    Solution::solve_second()
}

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}
