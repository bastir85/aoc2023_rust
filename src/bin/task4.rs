use std::{io::{BufReader, BufRead}, fs::File, cmp::{min}, iter::Sum};

const FILE_PATH: &str = "/workspaces/rust/hello_cargo/dat/task4.txt";
const TEXT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        Box::new(reader.lines().map(|line| line.unwrap().to_string()))
    }
}

fn extract_numbers(data: &str) -> Vec<i16>{
    let mut numbers: Vec<i16> = Vec::new();
    for d in data.split_whitespace(){
        numbers.push(d.parse().unwrap());
    }
    numbers
}
fn main(){
    let mut result = 0;
    let mut points = Vec::new();
    for line in get_lines(false){
        let (name, data) = line.split_once(':').unwrap();
        let (winning, game) = data.split_once('|').unwrap();
        let winning = extract_numbers(winning);
        let game = extract_numbers(game);

        let a= game.iter().filter(|v| winning.contains(*v)).count();
        points.push(a);
        if a > 0 {
            let b = 2_i32.pow((TryInto::<i32>::try_into(a).unwrap() - 1) as u32);
            result +=b;
            println!("{0}->{1}**2={2}",name, a, b);
        }
    }

    let mut num_cards = vec![1;points.len()];

    for idx in 0..points.len(){
        let point = points[idx];


        let end = min(idx+point+1, points.len());

        let copies = num_cards[idx];
        num_cards[idx+1..end].iter_mut().for_each(|v| *v += copies);
        println!("{0},{1}", idx+1,end);

    }

    println!("{0}", result);
    let result2:i32 = num_cards.iter().sum();
    println!("{0}",result2);

}