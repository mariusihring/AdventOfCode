const NUMS: &[&[u8]] = &[
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];
struct Solution;

impl Solution {
    fn find_digits(line: &[u8], index: usize) -> Option<usize> {
        line[index]
            .is_ascii_digit()
            .then_some((line[index] - b'0') as usize)
    }
    fn find_digits_and_strings(line: &[u8], index: usize) -> Option<usize> {
        line[index]
            .is_ascii_digit()
            .then_some((line[index] - b'0') as usize)
        .or(NUMS
            .iter()
            .enumerate()
            .find(|(_, name)| line[index..].starts_with(name))
            .map(|(num, _)| num + 1))
    }
    pub fn solve_first() {
        let result = include_bytes!("input/1/input")
            .split(|b| b == &b'\n')
            .map(|line| {
                (0..line.len()).find_map(|i| Self::find_digits(line, i)).unwrap() * 10
                    + (0..line.len())
                    .rev()
                    .find_map(|i| Self::find_digits(line, i))
                    .unwrap()
            })
            .sum::<usize>();
        println!("part 1: {}", result)
    }

    pub fn solve_second() {
        let result = include_bytes!("input/1/input")
            .split(|b| b == &b'\n')
            .map(|line| {
                (0..line.len()).find_map(|i| Self::find_digits_and_strings(line, i)).unwrap() * 10
                    + (0..line.len())
                        .rev()
                        .find_map(|i| Self::find_digits_and_strings(line, i))
                        .unwrap()
            })
            .sum::<usize>();

        println!("part 2: {}", result)
    }
}

fn main() {
    Solution::solve_first();
    Solution::solve_second();
}
