use regex::Regex;

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
                        symbol: c,
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
            let range = (number.start as isize - 1)..=(number.end as isize + 1);
            if range.contains(&symbol.idx) {
                used = true
            }
        });

        used
    }

    pub fn check_for_gear(symbol: &Symbol, numbers: &Vec<Number>) -> usize {
        let symbol_range = (symbol.idx - 1)..=(symbol.idx + 1);
        let mut valid_numbers: Vec<usize> = Vec::new();
        let relevant_numbers: Vec<&Number> = numbers
            .iter()
            .filter(|number| {
                symbol.line as isize == (number.line - 1)
                    || symbol.line as isize == (number.line + 1)
                    || symbol.line as isize == number.line
            })
            .collect();

        let sum = relevant_numbers
            .iter()
            .map(|number| {
                let sum = if symbol_range.contains(&(number.start as isize))
                    || symbol_range.contains(&(number.end as isize))
                {
                    valid_numbers.push(number.number);
                    if valid_numbers.len() == 2 {
                        return valid_numbers[0] * valid_numbers[1];
                    }
                    return 0;
                } else {
                    0
                };
                sum
            })
            .sum::<usize>();

        sum
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

    fn part_2() {
        let engine = Self::prepare();
        let sum = engine
            .symbols
            .iter()
            .map(|symbol| {
                if symbol.symbol == '*' {
                    return Self::check_for_gear(symbol, &engine.numbers);
                }
                return 0;
            })
            .sum::<usize>();

        println!("part 2: {}", sum)
    }
}

fn main() {
    Solution::part_1();
    Solution::part_2();
}
#[derive(Debug)]
struct Engine {
    symbols: Vec<Symbol>,
    numbers: Vec<Number>,
}

#[derive(Debug)]
struct Symbol {
    symbol: char,
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
