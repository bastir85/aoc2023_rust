use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader},
};

const TEXT: &str = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        Box::new(reader.lines().map(|line| line.unwrap().to_string()))
    }
}
const FILE_PATH: &str = "dat/task11.txt";


fn main() {
    let mut grid: Vec<Vec<char>> = get_lines(false).map(|v| v.chars().collect())
        .filter(|v: &Vec<char>| !v.is_empty()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut y_inserts = Vec::new();
    let grid2 = grid.clone();
    for y in 0..height{
        if grid[y].iter().filter(|v| **v != '.').count() == 0 {
            println!("y0 {}", y);
            y_inserts.push(y);
        }
    }
    let mut x_inserts = Vec::new();
    for x in 0..width{
        if grid.iter().filter(|v| v[x] != '.').count() == 0 {
            println!("x0 {}", x);
            x_inserts.push(x);
        }
    }
    for (idx,x) in x_inserts.iter().enumerate(){
        let x = x+idx;
        grid.iter_mut().for_each(|v: &mut Vec<char>| v.insert(x, v[x].clone()));
    }
    for (idx,y) in y_inserts.iter().enumerate(){
        let y = y+idx;
        grid.insert(y, grid[y].clone());

    }
    let height = grid.len();
    let width = grid[0].len();

    let mut galaxies = Vec::new();
    for y in 0..height{
        for x in 0..width{
            if grid[y][x] == '#'{
                galaxies.push((y,x));
            }
        }
    }
    let mut total = 0;
    for (idxA, galaxieA) in galaxies.iter().enumerate(){
        for (idxB, galaxieB) in galaxies[idxA+1..].iter().enumerate(){
            let dist = galaxieA.0.abs_diff(galaxieB.0) + galaxieA.1.abs_diff(galaxieB.1);
            //println!("{}", dist)
            total += dist;

        }
    }
    println!("{:?}", total);

    println!("{} {}", grid.len(), grid[0].len());

}
