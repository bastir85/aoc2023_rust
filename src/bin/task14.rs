use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, num,
    collections::HashMap,
};

const TEXT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";



fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        Box::new(reader.lines().map(|line| line.unwrap().to_string()))
    }
}
const FILE_PATH: &str = "dat/task14.txt";


fn main() {
    let mut total = 0;
    let mut total2 = 0;

    let mut nn = 0;
    let mut map : Vec<Vec<char>> = Vec::new();
    for line in get_lines(false){
            map.push(line.chars().collect());
    }
    let size_x = map[0].len();
    let size_y = map.len();   
    println!("size {} {}",size_x,size_y);
    for x in 0..size_x{
        let mut load = size_y;
        for y in 0..size_y{
            match map[y][x] {
                '#' =>  load = size_y - y - 1 , 
                '.' => continue ,
                'O' => {
                    total += load; 
                    load -=1;}, 
                _ => unreachable!("s"),
            }
        }
    } 
    println!("res {}",total);



}
