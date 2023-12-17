use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, convert,
};

use rand::{thread_rng, Rng};

const TEXT: &str = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;

fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        Box::new(reader.lines().map(|line| line.unwrap().to_string()))
    }
}
const FILE_PATH: &str = "dat/task17.txt";  // 6,7

fn run(grid: &Vec<Vec<u32>>, max_heat: u32) -> u32{
    let mut rng = thread_rng();

    let height = grid.len() -1;
    let width = grid[0].len() -1;

    let mut y: i32 = 0;
    let mut x: i32 = 0;

    let mut dy: i8 = 0;
    let mut dx: i8 = 1;

    let mut dir: u8 = 0; //dir is either left 0 right 1 or straight 2

    let mut steps = 0;
    let mut heat = 0;
    loop {
        let n_dir: u8 = rng.gen_range(0..3);

        {
        let n_dy: i8;
        let n_dx: i8;

        match n_dir {
            0 => {n_dx = -dy; n_dy = -dx}, // -1,0 --> 1,0, 1,0->0,-1, 0,1//-1,0
            1 => {n_dx = dy; n_dy= dx}, // -1,0 -> 0,-1
            _ => {n_dx = dx; n_dy= dy},
        }
        let nx = x + n_dx as i32;
        let ny = y + n_dy as i32;
        if nx < 0 || ny < 0 || nx > width as i32 || ny > height as i32 {
            continue;
        }

        if dir == n_dir {
            steps += 1;
        } else {
            steps = 0;
        }
        if steps > 3 {
            continue;
        }

        x = nx;
        y = ny;
        dir = n_dir;
        }

        heat += grid[y as usize][x as usize];
        //println!("{} {} = {}   {}",y,x,grid[y as usize][x as usize], heat);

        if y == height as i32 && x == width as i32{
            break;
        }
        if heat >= max_heat{
            break;
        }
    }
    return heat;
}


fn main() {
    let mut grid: Vec<Vec<u32>> = get_lines(true).map(|v| v.chars().map(|v| v.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .filter(|v: &Vec<u32>| !v.is_empty()).collect();
    let height = grid.len();
    let width = grid[0].len();
    println!("{:?}", grid);

    let mut min_heat = <u32>::max_value();

    for idx in 1..1000000 {
        min_heat = min(min_heat,run(&grid, min_heat));
    }

    println!("total {}", min_heat);

}
