use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn prepare() -> Vec<Numbers> {
        let lines = include_str!("./inputs/input").lines();
        let numbers = lines
            .map(|line| {
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
                Numbers {
                    win_numbers,
                    my_numbers,
                }
            })
            .collect();
        numbers
    }

    pub fn solve_first() {
        let numbers = Self::prepare();
        let sum: usize = numbers
            .iter()
            .map(|card| {
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
            })
            .sum();

        println!("part 1 : {}", sum)
    }

    pub fn solve_second() {
        let mut numbers = Self::prepare();
        let mut amounts = HashMap::new();
        numbers.iter().enumerate().for_each(|(index, card)| {
            let mut matches: usize = 0;

            card.my_numbers.iter().for_each(|number| {
                if card.win_numbers.contains(number) {
                    matches += 1
                }
            });

            let mut copies = (*amounts
                .entry(index)
                .or_insert(Matches { matches, copies: 1 }))
            .copies;
            for i in 1..=matches {
                let index = index + 1;
                copies = (*amounts
                    .entry(index)
                    .or_insert(Matches { matches, copies: 1 }))
                .copies;
                (*amounts
                    .entry(index + i)
                    .or_insert(Matches { matches, copies: 1 }))
                .copies += (1 * copies);
            }
        });
        let sum: usize = amounts.iter().map(|(key, matches)| matches.copies).sum();
        println!("part 2 : {:?}", sum);
    }
}

fn main() {
    Solution::solve_first();
    Solution::solve_second();
}

#[derive(Debug)]
struct Matches {
    copies: usize,
    matches: usize,
}

#[derive(Debug, Clone)]
struct Numbers {
    my_numbers: Vec<usize>,
    win_numbers: Vec<usize>,
}
