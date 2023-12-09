
use regex::Regex;
use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    vec,
    collections::HashMap,
    iter,
    fs
};
use itertools::izip;
const TEXT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

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
    //let lines : Vec<_> = TEXT.lines().collect();
    let tmps  = fs::read_to_string(FILE_PATH).expect("Unable to read file");
    let lines : Vec<&str> = tmps.split('\n').collect();
    let mut pos : usize = 0;
    let re_seeds = Regex::new(r"seeds: (?<numbers>[0-9 ]*)").unwrap();
    let seeds = lines[pos];
    pos += 1;
    let Some(caps) = re_seeds.captures(seeds) else {
    println!("no match!");
    return;
    };
    let seeds : Vec<u64> = caps["numbers"].split(' ').map(|x| x.parse::<u64>().unwrap()).collect();
    
    let (seed_to_soil_numbers,pos) = get_list(&lines,pos,"seed-to-soil map:");
    let (soil_to_fertilizer,pos) = get_list(&lines,pos,"soil-to-fertilizer map:");
    let (fertilizer_to_water,pos) = get_list(&lines,pos,"fertilizer-to-water map:");
    let (water_to_light,pos) = get_list(&lines,pos,"water-to-light map:");
    let (light_to_temperature,pos) = get_list(&lines,pos,"light-to-temperature map:");
    let (temperature_to_humidity,pos) = get_list(&lines,pos,"temperature-to-humidity map:");
    let (humidity_to_location,pos) = get_list(&lines,pos,"humidity-to-location map:");
    let mut locs : Vec<u64> = Vec::new();
    for ele in seeds.iter(){
        let soil = map(*ele,&seed_to_soil_numbers);
        let fertilizer = map(soil,&soil_to_fertilizer);
        let water = map(fertilizer,&fertilizer_to_water);
        let light = map(water,&water_to_light);
        let temperature = map(light,&light_to_temperature);
        let humidity = map(temperature,&temperature_to_humidity);
        let location = map(humidity,&humidity_to_location);
        //println!("seed {0} needs {1}, {2}, {3} ,{4} ,{5} ,{6}  location {7}",ele,soil,fertilizer,water,light,temperature,humidity,location);
        locs.push(location)
    }
    locs.sort();
    //for ele in locs.iter(){println!("loc {0}",ele);}
    println!("loc {0}",locs[0]);
    locs.clear();
    for i in 0..seeds.len()/2{
        println!("working on {0}",2*i);
        for j in 0..seeds[2*i+1]{
            let soil = map(seeds[2*i] +j,&seed_to_soil_numbers);
            let fertilizer = map(soil,&soil_to_fertilizer);
            let water = map(fertilizer,&fertilizer_to_water);
            let light = map(water,&water_to_light);
            let temperature = map(light,&light_to_temperature);
            let humidity = map(temperature,&temperature_to_humidity);
            let location = map(humidity,&humidity_to_location);
            //println!("seed {0} needs {1}, {2}, {3} ,{4} ,{5} ,{6}  location {7}",ele,soil,fertilizer,water,light,temperature,humidity,location);
            locs.push(location)
        }
    }
    locs.sort();
    //for ele in locs.iter(){println!("loc {0}",ele);}
    println!("loc {0}",locs[0]);
        
}
