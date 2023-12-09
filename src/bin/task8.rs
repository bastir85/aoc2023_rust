use std::fs::File;
use std::io::{self, BufRead};
use std::net::TcpListener;
use std::ops::RangeBounds;
use std::path::Path;
use std::println;
use std::collections::HashMap;
use std::cmp::max;


struct Lr{
    left : String,
    right : String,
}



fn part2(navi : HashMap<String,Lr>, cmds : Vec<char>, start : Vec<String> ){
    let mut start = start.clone();
    let mut run = start.len() ;
    let mut res : u32  = 0;
    let mut res2 : i32  = 0;
    let mut start_loop  : Vec<Vec<String>> = Vec::new();
    let mut loops = vec![0; start.len()];
    while run!=0{
        start_loop.push(start.clone()); 
        for c in cmds.iter(){
            run = start.len() ;
            for s in start.iter_mut(){
                let ends = match navi.get(s){
                    Some(x) => x,
                    None => panic!("node not found"),
                };
                *s = match c{
                    'L' => ends.left.clone(),
                    'R' => ends.right.clone(),
                    _ =>  panic!("cmd not L or R"),
                };
                
                if s.chars().rev().next() == Some('Z') {
                    run -= 1;
                    //println!("{} {}",s,(res+1) as f64 / cmds.len() as f64);
                }
            }
            res += 1;      
        }
        
        for (i,sl) in start_loop.iter().enumerate(){
            for j in 0..start.len(){
                if start[j]==sl[j]{
                    //println!("loop detected for {} {}",i,j);
                    if loops[j]==0{
                        loops[j] = res2 - i as i32 + 1;
                    }
                }
            }
        }
        res2 += 1;
        if loops.iter().all(|&x| x > 0){
            //println!("{:?}",loops);
            break;
        }
        //if res == 53*47*(cmds.len() as u32){break;};
    }
    for j in 0..start.len(){
        println!("{:?} {:?}",start[j], start_loop[0*(res2 - loops[j]) as usize][j]);
    }
    println!("{:?}",loops);
    println!("{}",(cmds.len() as u64)*loops.iter().fold(1 as u64,|acc,y| acc*(*y as u64) )); //luckily they are all prime numbers ....
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./dat/task8.txt") {
        // Consumes the iterator, returns an (Optional) String
        
        let mut cmds : Vec<char> = Vec::new();
        let mut navi : HashMap<String,Lr> = HashMap::new();
        let mut start  : Vec<String> = Vec::new();
        for (i,line) in lines.enumerate(){
            if let Ok(line) = line {
                if i==0{
                cmds.extend(line.chars());
                }else if line.len()>0{
                let mut lsplit = line.split('=');
                let node : &str =  lsplit.next().unwrap().trim();
                if node.chars().rev().next() == Some('A'){
                    start.push(node.to_string());
                }
                let mut lr =  lsplit.next().unwrap().trim().split(',');
                navi.insert(node.to_string(),
                            Lr{left : lr.next().unwrap().trim()[1..4].to_string(),
                            right : lr.next().unwrap().trim()[0..3].to_string()});
                }
            }
        }
        let mut res = 0;
        let mut run = true;
        let mut starti = "AAA".to_string();
        while run{
            for c in cmds.iter(){
                let ends = match navi.get(&starti){
                    Some(x) => x,
                    None => panic!("node not found"),
                };
                starti = match c{
                    'L' => ends.left.clone(),
                    'R' => ends.right.clone(),
                    _ =>  panic!("cmd not L or R"),
                };
                res += 1; 
                if starti == "ZZZ" {
                    run = false;
                    break;
                }
                     
            }
        }
        println!("res {res}");
        part2(navi,cmds,start);
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