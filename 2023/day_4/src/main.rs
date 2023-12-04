struct Solution;

impl Solution {
    pub fn prepare() -> Vec<Numbers> {
        let lines = include_str!("./inputs/input").lines();
        let numbers = lines.map(|line| {
            let card: Vec<&str> = line.split(":").collect();

            let mut my_str_numbers = card[1].split(" | ").collect::<Vec<&str>>()[1]
                .split(" ")
                .collect::<Vec<&str>>();
            my_str_numbers.retain(|s| !s.is_empty());
            let my_numbers: Vec<usize> = my_str_numbers
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();

            let mut win_str_numbers = card[1].split(" | ").collect::<Vec<&str>>()[0]
                .split(" ")
                .collect::<Vec<&str>>();
            win_str_numbers.retain(|s| !s.is_empty());
            let win_numbers: Vec<usize> = win_str_numbers
                .iter()
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            Numbers { win_numbers, my_numbers }
        }).collect();
        numbers
    }
    pub fn solve_first() {
        let numbers = Self::prepare();
        let sum: usize = numbers.iter().map(|card| {
            let mut points = 0;
            card.my_numbers.iter().for_each(|number| {
                if card.win_numbers.contains(number) {
                    if points == 0 {
                        points = 1
                    } else {
                        points = points * 2
                    }
                }
            });
            points
        }).sum();

        println!("part 1 : {}", sum)
    }
}

fn main() {
    Solution::solve_first();
}

#[derive(Debug)]
struct Numbers {
    my_numbers: Vec<usize>,
    win_numbers: Vec<usize>,
}