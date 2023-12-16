use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, num,
    collections::HashMap,
};

const TEXT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

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
const FILE_PATH: &str = "dat/task13.txt";


fn main() {
    let mut total = 0;
    let mut total2 = 0;

    let mut nn = 0;
    let mut map : Vec<Vec<char>> = Vec::new();
    for line in get_lines(false){
        if ! line.is_empty(){
            map.push(line.chars().collect());
        }else{
            println!("next");
            let size_x = map[0].len();
            let size_y = map.len();    
            for i in 0..size_y-1{
                let mut axis = true;
                let mut fix = 0;
                let mut d = 0;
                while d<=i && d<=size_y-2-i {
                    let m: usize = (0..size_x).filter(|&x| map[i-d][x] == map[i+d+1][x]).count();
                    if m==size_x - 1 && fix<2{
                        fix +=1;
                        d +=1;
                    }else if  m != size_x {
                        axis = false;
                        break;
                    }else{
                        d +=1;
                    }
                }
                if axis {println!("f {} x {}",fix,i); if fix==0 {total += 100*(i+1);}else{total2+=100*(i+1)}}
            }
            for i in 0..size_x-1{
                let mut axis = true;
                let mut d = 0;
                let mut fix = 0;

                while d<=i && d<=size_x-2-i {
                    let m: usize = (0..size_y).filter(|&x| map[x][i-d]==map[x][i+d+1] ).count();
                    if m==size_y - 1 && fix<2{
                        fix +=1;
                        d +=1;
                    }else if  m != size_y {
                        axis = false;
                        break;
                    }else{
                        d +=1;
                    }
                }
                if axis {println!("f {} x {}",fix,i); if fix==0 {total += i+1;}else{total2+=i+1;}}
            }

            map.iter_mut().for_each(|x| x.clear());
            map.clear();
        }
        
    }
    println!("res {} {}",total,total2);



}
