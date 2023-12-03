use regex::Regex;
use std::ops::Index;

struct Solution;

impl Solution {
    pub fn prepare() -> Engine {
        let lines = include_str!("./inputs/input").lines();
        let num_reg = Regex::new(r"\d+").expect("failed to build Regex");
        let mut symbols: Vec<Symbol> = Vec::new();
        let mut numbers: Vec<Number> = Vec::new();
        let _ = lines.enumerate().for_each(|(i, line)| {
            let _ = line.chars().enumerate().for_each(|(x, c)| {
                if !c.is_ascii_alphanumeric() && !(c == '.') {
                    symbols.push(Symbol {
                        line: i,
                        idx: x as isize,
                    })
                }
            });
            for capture in num_reg.find_iter(line) {
                numbers.push(Number {
                    number: capture.as_str().parse::<usize>().unwrap(),
                    start: capture.start(),
                    end: capture.end() - 1,
                    line: i as isize,
                })
            }
        });
        Engine { symbols, numbers }
    }

    pub fn check_adjacent(symbols: &Vec<Symbol>, number: &Number) -> bool {
        let mut used = false;
        let relevant_symbols: Vec<&Symbol> = symbols
            .iter()
            .filter(|symbol| {
                symbol.line as isize == (number.line - 1)
                    || symbol.line as isize == (number.line + 1)
                    || symbol.line as isize == number.line
            })
            .collect();
        relevant_symbols.iter().for_each(|symbol| {
            if symbol.idx >= (number.start as isize - 1) && symbol.idx <= (number.end as isize + 1)
            {
                // println!("symbol: {}, number_start: {}, number_end: {} for number {}", symbol.idx, (number.start as isize), (number.end as isize) as isize, number.number);
                used = true
            }
        });

        used
    }

    fn part_1() {
        let engine = Self::prepare();
        let sum = engine
            .numbers
            .iter()
            .filter(|number| Self::check_adjacent(&engine.symbols, number.to_owned()))
            .map(|number| number.number)
            .sum::<usize>();

        println!("part 1: {}", sum)
    }
}

fn main() {
    Solution::part_1();
}
#[derive(Debug)]
struct Engine {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

#[derive(Debug)]
struct Symbol {
    line: usize,
    idx: isize,
}
#[derive(Debug)]
struct Number {
    number: usize,
    start: usize,
    end: usize,
    line: isize,
}
