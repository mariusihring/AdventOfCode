use indexmap::IndexMap;
use std::env::current_exe;

struct Solution;

impl Solution {
    pub fn prepare() -> Vec<Report> {
        let lines = include_str!("./inputs/input").lines();
        let reports: Vec<Report> = lines
            .enumerate()
            .map(|(i, line)| {
                let nums: Vec<isize> = line
                    .split_ascii_whitespace()
                    .map(|number| number.trim().parse::<isize>().unwrap())
                    .collect();
                Report {
                    line: nums,
                    differences: Vec::new(),
                }
            })
            .collect();
        reports
    }

    pub fn get_differences(nums: &Vec<isize>) -> Vec<isize> {
        let mut differences: Vec<isize> = Vec::new();
        for x in 1..nums.len() {
            let diff = nums[x] - nums[x - 1];
            differences.push(diff)
        }
        differences
    }

    pub fn solve_first() {
        let mut reports = Self::prepare();

        let sum: isize = reports.iter_mut().map(|mut report| {
            let first = Self::get_differences(&report.line);
            report.differences.push(first);
            'get_children: loop {
                let diffs = Self::get_differences(&report.differences.last().unwrap());
                report.differences.push(diffs.clone());
                if diffs.iter().all(|x| x == &0) {
                    break 'get_children;
                }
            }
            let len = report.differences.len() - 1;
            for index in (0..len).rev() {
                let last_value = report.differences[index].last().cloned().unwrap_or_default();
                let current_last = report.differences[index + 1].last().cloned().unwrap_or_default();
                report.differences[index].push(last_value + current_last);
            }
            report.line.last().unwrap() + report.differences[0].last().unwrap()
        }).sum();

        println!("part 1 : {:?}", sum)
    }
}

fn main() {
    Solution::solve_first();
}

#[derive(Debug)]
struct Report {
    line: Vec<isize>,
    differences: Vec<Vec<isize>>,
}
