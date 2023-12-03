use core::num;
use regex::Regex;
use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
    ops::{Range, RangeBounds},
    vec,
};

const TEXT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

const FILE_PATH: &str = "/workspaces/rust/hello_cargo/dat/task3.txt";

fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        return Box::new(reader.lines().map(|line| line.unwrap().to_string()));
    }
}

struct Grid {
    data: Vec<Vec<char>>,
    mask: Vec<Vec<bool>>,
    size_x: usize,
    size_y: usize,
    numbers: Vec<Number>,
}

struct Number {
    value: i16,
    x_start: usize,
    x_stop: usize,
    y: usize,
}
impl Grid {
    pub fn new(lines: Box<dyn Iterator<Item = String>>) -> Grid {
        let mut data: Vec<Vec<char>> = Vec::new();
        let mut mask: Vec<Vec<bool>> = Vec::new();

        for line in lines {
            let tmp: Vec<_> = line.chars().collect();
            mask.push(vec![false; tmp.len()]);
            data.push(tmp);
        }
        let size_x = data[0].len();
        let size_y = data.len();
        let mut grid = Grid {
            data: data,
            size_x: size_x,
            size_y: size_y,
            mask: mask,
            numbers: Vec::new(),
        };
        grid.create_symbol_mask();
        grid.find_numbers();
        return grid;
    }

    pub fn create_symbol_mask(&mut self) {
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let chr = self.data[y][x];
                if chr != '.' && chr.is_ascii_punctuation() {
                    for y_offset in -1..2 {
                        for x_offset in -1..2 {
                            let nx = x as i16 + x_offset as i16;
                            let ny = y as i16 + y_offset as i16;
                            if nx < 0
                                || ny < 0
                                || nx > self.size_x as i16
                                || ny > self.size_y as i16
                            {
                                continue;
                            }
                            let nx = nx as usize;
                            let ny = ny as usize;
                            self.mask[ny][nx] = true;
                        }
                    }
                }
            }
        }
    }

    fn find_numbers(&mut self) {
        let re = Regex::new(r"\d+").unwrap();
        self.numbers.clear();
        for y in 0..self.size_y {
            let line: String = self.data[y].iter().collect();

            for c in re.captures_iter(line.as_str()) {
                if c.len() == 1 {
                    let c = c.get(0).unwrap();
                    let num: i16 = c.as_str().parse().unwrap();
                    self.numbers.push(Number {
                        value: num,
                        x_start: c.start(),
                        x_stop: c.end() - 1,
                        y: y,
                    });
                }
            }
        }
    }

    fn get_valid_numbers(&mut self) -> Vec<i16> {
        let mut valid_numbers = Vec::new();
        for number in &self.numbers {
            if self.mask[number.y][number.x_start..number.x_stop + 1]
                .iter()
                .any(|f| *f)
            {
                valid_numbers.push(number.value);
            } else {
                println!(
                    "{0},{1}-{2}-> {3}",
                    number.y, number.x_start, number.x_stop, number.value
                );
            }
        }

        return valid_numbers;
    }

    pub fn get_gears(&mut self) -> i64 {
        let mut result: i64 = 0;
        for y in 0..self.size_y {
            for x in 0..self.size_x {
                let chr = self.data[y][x];
                if chr == '*' {
                    println!("*{y},{x}");
                    let adjecent_numbers = self.find_adjacent_numbers(x, y);
                    if adjecent_numbers.len() == 2 {
                        println!("{0},{1}->{2:?}", y, x, adjecent_numbers);
                        result += adjecent_numbers[0] as i64 * adjecent_numbers[1] as i64;
                    }
                }
            }
        }
        return result;
    }

    fn find_adjacent_numbers(&mut self, x: usize, y: usize) -> Vec<i16> {
        let mut adjacent_numbers: Vec<i32> = Vec::new();
        return self
            .numbers
            .iter()
            .filter(|f| {
                let a =
                    f.y.abs_diff(y) <= 1 && min(f.x_start.abs_diff(x), f.x_stop.abs_diff(x)) <= 1;
                if a {
                    println!(
                        "{0},{1}:{2}",
                        f.y.abs_diff(y),
                        f.x_start.abs_diff(x),
                        f.x_stop.abs_diff(x)
                    );
                }
                a
            })
            .map(|v| v.value)
            .collect();
    }
}

fn main() {
    let mut grid = Grid::new(get_lines(false));

    let results = grid.get_valid_numbers();
    let result2 = grid.get_gears();
    let mut result: i32 = 0;
    for res in results {
        result += res as i32;
    }
    println!("Result {result} result2 {result2}");
}
