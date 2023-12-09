
use regex::Regex;
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    vec,
    iter,
    fs
};
const TEXT: &str = "Time:      7  15   30
Distance:  9  40  200";

const TEXT2: &str = "Time:        56     71     79     99
Distance:   334   1135   1350   2430";

const TEXT3: &str = "Time:        56717999
Distance:   334113513502430";

const FILE_PATH: &str = "/var/home/julian/rust/aoc2023_rust/dat/task5.txt";

struct Range {
    start_a: u64,
    end_a: u64,
    start_b: u64,
}


fn get_list(lines : &Vec<&str>, mut pos:usize, typ : &str) -> (Vec<Range>,usize){
    loop {
        let line = lines[pos];
        pos += 1;
        if line.eq(typ) {
            break;
        }
    }
    let mut map_vector = Vec::<Range>::new();
    loop {
        let s_line = lines[pos].split(' ');
        pos += 1;
        if pos==lines.len() {break;}
        if s_line.clone().count()!=3 {break;}
        let numbers : Vec<_> = s_line.map(|x| x.parse::<u64>().unwrap()).collect();
        map_vector.push(Range{start_a : numbers[1], end_a : numbers[1]+numbers[2]-1, start_b : numbers[0]})
    }
    map_vector.sort_by(|a, b| a.start_a.cmp(&b.start_a));
    (map_vector, pos)
}

fn map(input:u64,map_vec : &Vec<Range>) -> u64{
    if input<map_vec[0].start_a {return input;}
    else if input>map_vec[map_vec.len() - 1].end_a {return input;}
    else {
        for ri in map_vec{
            if input>=ri.start_a && input<=ri.end_a {
                let tmp = input - ri.start_a ;
                return ri.start_b + tmp;  
            } 
        }
    }
    return input;
}

fn main() {
    let lines : Vec<_> = TEXT3.lines().collect();
    //let tmps  = fs::read_to_string(FILE_PATH).expect("Unable to read file");
    //let lines : Vec<&str> = tmps.split('\n').collect();
    let re_seeds = Regex::new(r"Time: (?<numbers>[0-9 ]*)").unwrap();
    let Some(caps) = re_seeds.captures(lines[0]) else {
    println!("no match!");
    return;
    };
    let time : Vec<u64> = caps["numbers"].split(' ')
        .filter_map(|x| match x.is_empty() {
            true => None,
            false =>  Some(x.parse::<u64>().unwrap()),
            })
        .collect();
    let re_dist = Regex::new(r"Distance: (?<numbers>[0-9 ]*)").unwrap();
    let Some(caps) = re_dist.captures(lines[1]) else {
    println!("no match!");
    return;
    };
    let dist : Vec<u64> = caps["numbers"].split(' ')
        .filter_map(|x| match x.is_empty() {
            true => None,
            false =>  Some(x.parse::<u64>().unwrap()),
            })
        .collect();
    
    let mut res : u64 = 1;
    for (timei,disti) in iter::zip(time.iter(),dist.iter()){
        let p = 0.5*(*timei as f64);
        let timeL =  p - ( p*p - (*disti as f64)).sqrt();
        let timeU =  p + ( p*p - (*disti as f64)).sqrt();
        let lower = match timeL.rem_euclid(1.0) {
            0.0 => (timeL + 1.0) as u64,
            _ => (timeL.ceil()) as u64
            };
        let upper = match timeU.rem_euclid(1.0) {
                0.0 => (timeU - 1.0) as u64,
                _ => (timeU.floor()) as u64
                };
        println!("time {0}",upper-lower + 1);
        res *= upper-lower + 1;
    }
    println!("time {0}",res);
 
 
        
}
