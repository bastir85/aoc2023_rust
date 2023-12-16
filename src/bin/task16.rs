use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, convert,
};



const TEXT: &str = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        Box::new(reader.lines().map(|line| line.unwrap().to_string()))
    }
}
const FILE_PATH: &str = "dat/task16.txt";  // 6,7

fn beam(start_y: i32, start_x: i32,
        init_dy: i8, init_dx :i8,
        grid: &Vec<Vec<char>>, covert: &mut Vec<Vec<bool>> ){
    let height = grid.len() ;
    let width = grid[0].len();

    let mut y: i32 = start_y as i32;
    let mut x: i32 = start_x as i32;

    let mut dx = init_dx;
    let mut dy = init_dy;

    if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
        //println!("out");
        return;
    }


    loop {
        //println!("{0},{1}",y,x);

        match grid[y as usize][x as usize]{
            '\\' => (dy, dx) = (dx, dy), // 0,1 -> 1,0; 1,0 -> 0,1; -1,0 -> 0, -1; -1,0 ->-1,0
            '/' => (dy, dx) = (-dx, -dy), // 0,1 -> -1,0; -1,0 -> 0,1;
            '|' => if dy==0 {
                // we have been here alreday?
                if covert[y as usize][x as usize] {
                    //println!("covered -> break!");
                    return;
                }
                covert[y as usize][x as usize] = true;
                //println!("split beam | down {}, {}",y,x);
                beam(y +1, x , 1, 0, grid, covert);
                //println!("split beam | up {}, {}",y,x);
                beam(y -1, x, -1, 0, grid, covert);
                return;
            },
            '-' => if dx==0 {
                // we have been here alreday?
                if covert[y as usize][x as usize] {
                    //println!("covered -> break!");
                    return;
                }
                covert[y as usize][x as usize] = true;
                //println!("split beam - right {}, {}",y,x);
                beam(y, x + 1, 0, 1, grid, covert);
                //println!("split beam - left {}, {}",y,x);
                beam(y, x - 1, 0, -1, grid, covert);
                return;
            },
            _ => ()
        }
        covert[y as usize][x as usize] = true;

        x = x + dx as i32;
        y = y + dy as i32;

        // not efficent loop detection
        if  y  == start_y && x ==start_x &&  dx == init_dx && dy == init_dy{
            //println!("Circle");

            return;
        }
        if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
            //println!("out");

            return;
        }
    }

}


fn main() {
    let mut grid: Vec<Vec<char>> = get_lines(false).map(|v| v.chars().collect())
        .filter(|v: &Vec<char>| !v.is_empty()).collect();
    let height = grid.len();
    let width = grid[0].len();
    println!("{:?}", grid);

    let mut total = 0;
    for y in 0..height{
        let mut covered:Vec<Vec<bool>> = grid.iter().map(|v| vec![false; v.len()]).collect();
        beam(y as i32,0, 0, 1, &grid,  &mut covered); // start to the right
        let total2: usize = covered.iter().map(|v| v.iter().filter(|v| **v).count()).sum();
        total = max(total, total2);
    }
    for y in 0..height{
        let mut covered:Vec<Vec<bool>> = grid.iter().map(|v| vec![false; v.len()]).collect();
        beam(y as i32,width as i32 -1, 0, -1, &grid,  &mut covered); // start to the right
        let total2: usize = covered.iter().map(|v| v.iter().filter(|v| **v).count()).sum();
        total = max(total, total2);
    }
    for x in 0..width{
        let mut covered:Vec<Vec<bool>> = grid.iter().map(|v| vec![false; v.len()]).collect();
        beam(0,x as i32, 1, 0, &grid,  &mut covered); // start to the right
        let total2: usize = covered.iter().map(|v| v.iter().filter(|v| **v).count()).sum();
        total = max(total, total2);
    }
    for x in 0..width{
        let mut covered:Vec<Vec<bool>> = grid.iter().map(|v| vec![false; v.len()]).collect();
        beam(width as i32 - 1,x as i32, -1, 0, &grid,  &mut covered); // start to the right
        let total2: usize = covered.iter().map(|v| v.iter().filter(|v| **v).count()).sum();
        total = max(total, total2);
    }

    println!("total {}", total);

}
