use std::{io::{BufReader, BufRead}, ops::Range, fs::File, cmp::{min}, iter::Sum, collections::HashMap};


const FILE_PATH: &str = "/workspaces/rust/hello_cargo/dat/task4.txt";
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

fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        Box::new(reader.lines().map(|line| line.unwrap().to_string()))
    }
}

struct Rule {
    range:Range<i16>,
    value:i8
}

fn main(){
    let mut result = 0;
    let mut result2 = 0;
    let mut data: HashMap<String, Vec<Rule>> = HashMap::new();

    let mut current;
    for line in get_lines(false){
        if line.contains(":"){
            let tmp = Vec::new();
            current = data.insert(line, tmp).unwrap();
        }
        current.push(Rule { range: 1..3, value: 4 });
    }
    println!("{0}", result);
    println!("{0}",result2);

}