use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, num,
    collections::HashMap, usize,
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

fn move_sn( map :& mut Vec<Vec<char>>, size_x : usize, size_y : usize, n : bool ) {
    for x in 0..size_x{ 
        let mut skip : i32=0;
        for yy in 0..size_y{
            let y =  if n {yy} else {size_y - 1 - yy};
            match map[y][x] {
                '#' =>  skip = 0 , 
                '.' => skip += if n {1} else {-1} ,
                'O' => {
                    if skip!=0{
                    map[(y as i32 - skip) as usize][x] = 'O'; 
                    map[y][x] = '.';
                    }
                    }, 
                _ => unreachable!("s"),
            }        
        }
    }
}
fn move_ew( map :& mut Vec<Vec<char>>, size_x : usize, size_y : usize, w : bool ) {
    for x in 0..size_y{ 
        let mut skip : i32=0;
        for yy in 0..size_x{
            let y =  if w {yy} else {size_y - 1 - yy};
            match map[x][y] {
                '#' =>  skip = 0 , 
                '.' => skip += if w {1} else {-1} ,
                'O' => {
                    if skip!=0{
                    map[x][(y as i32 - skip) as usize] = 'O'; 
                    map[x][y] = '.';
                    }
                    }, 
                _ => unreachable!("s"),
            }        
        }
    }
}


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
    //map.iter().for_each(|f| println!("{}", f.iter().collect::<String>()));
    println!("  s");
    let mut hist : Vec<Vec<Vec<char>>> = Vec::new(); 
    let mut run = true;
    let mut nloop = 0;
    let mut ninit = 0;
    for i in 0..999999{
        hist.push( map.clone() );
        move_sn(&mut map,size_y,size_x,true);
        move_ew(&mut map,size_y,size_x,true);
        move_sn(&mut map,size_y,size_x,false);
        move_ew(&mut map,size_y,size_x,false);
        for (j,map2) in hist.iter().enumerate(){
            if *map2==map {
                println!("i {} j {}",i,j); 
                nloop = i - j + 1;
                ninit = j;
                run=false; 
                break;}
        }
        if !run{
            break;
        }
    }
    let remaining = (1000000000 - ninit) % nloop; 
    println!("ninit {} nloop {}  r {}",ninit,nloop,remaining); 
    for _i in 0..remaining{
        move_sn(&mut map,size_y,size_x,true);
        move_ew(&mut map,size_y,size_x,true);
        move_sn(&mut map,size_y,size_x,false);
        move_ew(&mut map,size_y,size_x,false);
    }
    total = 0;
    for x in 0..size_x{
        for y in 0..size_y{
            total += if map[y][x]=='O' {size_y-y} else {0};
        }
    }
    println!("total {}",total); 
    //map.iter().for_each(|f| println!("{}", f.iter().collect::<String>()));

}
