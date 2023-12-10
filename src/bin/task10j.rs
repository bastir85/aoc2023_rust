use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::ops::RangeBounds;
use std::path::Path;
use std::{println, vec};
use std::collections::HashMap;
use std::cmp::max;

#[derive(Copy, Clone)]
struct Pos {
    a : (i8,i8), //tuples x,y
    b : (i8,i8),
}

fn find_starts(x:u8,y:u8,map: & Vec<Vec<char>>,navi : &HashMap<char,Pos>, size_x : u8, size_y : u8) ->  ((u8,u8,u8,u8),(u8,u8,u8,u8)) {
    let mut starts : Vec<(u8,u8,u8,u8)> = Vec::new();
    for (yd,xd) in iter::zip([-1i8,1,0,0],[0i8,0,-1,1]){
        //for xd in -1i8..1{
            let xnew : u8 = (x as i16 + xd as i16) as u8;
            let ynew : u8 = (y as i16 + yd as i16) as u8;
            if xnew < 0 || ynew < 0 || xnew > size_x-1 || ynew > size_y-1 { continue;}
            let c : char = map[(ynew) as usize][(xnew) as usize];
            if c=='.' {continue;}
            let el  = navi[&c];
            if el.a.0 == -xd && el.a.1 == -yd{
                starts.push( (xnew,ynew,x,y) );
            }else if el.b.0 == -xd && el.b.1 == -yd{
                starts.push( (xnew,ynew,x,y) );
            }
        //}   
    } 
    if starts.len()!=2{
        panic!("2");
    }
    (starts[0],starts[1])
}

fn go_step(p:(u8,u8,u8,u8) ,map: & Vec<Vec<char>>,navi : &HashMap<char,Pos>) ->  (u8,u8,u8,u8) {
    let (x,y,xold,yold) = p;
    let c : char = map[(y) as usize][(x) as usize];
    let el  = navi[&c];
    let ( mut xnew, mut ynew) =  ((x as i16 + el.a.0 as i16) as u8, (y as i16 + el.a.1 as i16 ) as u8);
    if xnew == xold && ynew == yold {
        (xnew, ynew) =  ((x as i16 + el.b.0 as i16) as u8, (y as i16 + el.b.1 as i16 ) as u8);
    }

    (xnew,ynew,x,y)
}

fn mark_x(p:(u8,u8,u8,u8),c :char,direction : i8,mut map2: & mut Vec<Vec<char>>, size_x : u8, size_y : u8) -> bool{
    let mut n : i16 = 0;
    loop{
        n += direction as i16;
        let next = p.0 as i16 + n;
        if next < 0 || next as u8 > size_x-1 {
            return false
        }
        if map2[p.1 as usize][next as usize] == 'S'{
            break;
        }
        map2[p.1 as usize][next as usize]  = c;
    }
    true
}
fn mark_y(p:(u8,u8,u8,u8),c :char,direction : i8,mut map2: & mut Vec<Vec<char>>, size_x : u8, size_y : u8) -> bool{
    let mut n : i16 = 0;
    loop{
        n += direction as i16;
        let next = p.1 as i16 + n;
        if next < 0 || next as u8 > size_y-1 {
            return false
        }
        if map2[next as usize][(p.0  ) as usize] == 'S'{
            break;
        }
        map2[next as usize][(p.0) as usize]  = c;
    }
    true
}


fn main() {
    // File hosts.txt must exist in the current path
    let mut navi : HashMap<char,Pos> = HashMap::new();
    navi.insert('|', Pos{a : (0,1),b : (0,-1)});
    navi.insert('-', Pos{a : (1,0),b : (-1,0)});
    navi.insert('L', Pos{a : (0,-1),b : (1,0)});
    navi.insert('J', Pos{a : (0,-1),b : (-1,0)});
    navi.insert('7', Pos{a : (0,1),b : (-1,0)});
    navi.insert('F', Pos{a : (0,1),b : (1,0)});

    let mut map : Vec<Vec<char>> = Vec::new();  
    if let Ok(lines) = read_lines("./dat/task10.txt") {        
        let mut start : (u8,u8) = (0,0);
        for (y,line) in lines.enumerate(){
            if let Ok(line) = line {
                let mut tmp : Vec<char> = Vec::new();
                for (x,c) in line.chars().enumerate(){
                    tmp.push(c);
                    if c=='S'{
                        start = (x as u8,y as u8);
                    }
                }
                map.push( tmp  );
                
            }
        }
        let size_x = map[0].len() as u8;
        let size_y = map.len() as u8;
        for i in map.iter(){println!("{:?}",i);}
        let mut map2 = map.clone();
        let mut res : i64 = 0;
        let  ( mut startA, mut startB) = find_starts(start.0,start.1,&map,&navi,size_x,size_y);
        println!("res {:?}",startA);
        map2[startA.1 as usize][startA.0 as usize] = 'S';
        map2[startB.1 as usize][startB.0 as usize] = 'S';
        loop{
            res += 1;
            startA = go_step(startA,&map,&navi);
            map2[startA.1 as usize][startA.0 as usize] = 'S';
            if startA.0 == startB.0 && startA.1 == startB.1 {
                break;
            }
            
            startB = go_step(startB,&map,&navi);
            map2[startB.1 as usize][startB.0 as usize] = 'S';
            if startA.0 == startB.0 && startA.1 == startB.1 {
                break;
            }
        }
        println!("res {:?}",res+1);
        let  ( mut startA, mut startB) = find_starts(start.0,start.1,&map,&navi,size_x,size_y);
        let mut left = true;
        let mut right = true;
        loop{
            if map[startA.1 as usize][startA.0 as usize] == '|'{
                if startA.1 as i16 - startA.3 as i16 > 0 { //going to higer y
                    left &= mark_x(startA,'L',1,& mut map2,size_x,size_y);
                    right&= mark_x(startA,'R',-1,& mut map2,size_x,size_y);
                }else{
                    left &= mark_x(startA,'L',-1,& mut map2,size_x,size_y);
                    right&= mark_x(startA,'R',1,& mut map2,size_x,size_y);
                }
            }else if map[startA.1 as usize][startA.0 as usize] == '-'{
                if startA.0 as i16 - startA.2 as i16 > 0 { //going to higer x
                    right&= mark_y(startA,'R',1,& mut map2,size_x,size_y);
                    left &= mark_y(startA,'L',-1,& mut map2,size_x,size_y);
                }else{
                    right&= mark_y(startA,'R',-1,& mut map2,size_x,size_y);
                    left &= mark_y(startA,'L',1,& mut map2,size_x,size_y);
                }
            }else if map[startA.1 as usize][startA.0 as usize] == 'F'{
                if startA.1 as i16 != startA.3 as i16 { 
                    left &= mark_x(startA,'L',-1,& mut map2,size_x,size_y);
                    left &= mark_y(startA,'L',-1,& mut map2,size_x,size_y);
                }else{
                    right &= mark_x(startA,'R',-1,& mut map2,size_x,size_y);
                    right &= mark_y(startA,'R',-1,& mut map2,size_x,size_y);
                }
            }else if map[startA.1 as usize][startA.0 as usize] == '7'{
                if startA.0 as i16 != startA.2 as i16  { 
                    left &= mark_x(startA,'L',1,& mut map2,size_x,size_y);
                    left &= mark_y(startA,'L',-1,& mut map2,size_x,size_y);
                }else{
                    right &= mark_x(startA,'R',1,& mut map2,size_x,size_y);
                    right &= mark_y(startA,'R',-1,& mut map2,size_x,size_y);
                }
            }else if map[startA.1 as usize][startA.0 as usize] == 'J'{
                if startA.1 as i16 != startA.3 as i16  { 
                    left &= mark_x(startA,'L',1,& mut map2,size_x,size_y);
                    left &= mark_y(startA,'L',1,& mut map2,size_x,size_y);
                }else{
                    right &= mark_x(startA,'R',1,& mut map2,size_x,size_y);
                    right &= mark_y(startA,'R',1,& mut map2,size_x,size_y);
                }
            }else if map[startA.1 as usize][startA.0 as usize] == 'L'{
                if startA.0 as i16 != startA.2 as i16  { 
                    left &= mark_x(startA,'L',-1,& mut map2,size_x,size_y);
                    left &= mark_y(startA,'L',1,& mut map2,size_x,size_y);
                }else{
                    right &= mark_x(startA,'R',-1,& mut map2,size_x,size_y);
                    right &= mark_y(startA,'R',1,& mut map2,size_x,size_y);
                }
            }
            startA = go_step(startA,&map,&navi);
            if map[startA.1 as usize][startA.0 as usize] == 'S'{break;}
        }
        
        println!("right {} left {}",right,left);
        let pattter = if right {'R'} else {'L'};
        let pattter2 = if !right {'R'} else {'L'};
        res = 0;
        for (y,tmp) in map2.iter().enumerate(){
            for (x,c) in tmp.iter().enumerate(){
                if *c == pattter {
                    res+=1;
                    map[y][x] = 'I';
                }
                if *c == pattter2 {
                    map[y][x] = '.';
                }
            }
        }
        for i in map.iter(){println!("{:?}",i.into_iter().collect::<String>());}
        println!("res {} ",res);
    }else{
        println!("file not found");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
