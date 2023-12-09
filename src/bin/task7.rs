use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::println;

const CARDS : &'static [char] = &['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARDS2 : &'static [char] = &['A', 'K', 'Q',  'T', '9', '8', '7', '6', '5', '4', '3', '2','J'];

struct Game {
    ranking: u64,
    score: u32,
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./dat/task7.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut games : Vec<Game> = Vec::new();
        for line in lines{
            if let Ok(line) = line {
                let mut lineiter = line.split(" ");
                let cards : Vec<_> = lineiter.next().expect("no cards")
                        .chars().collect();
                let mut array: [u8; 13] = [0; 13];
                let mut cardrank: u64 = 0;
                let mut j = 0;
                for (i,card) in cards.iter().rev().enumerate(){
                    // for part 1
                    //let c_num = CARDS.iter().rev().position(|&x| x == *card).unwrap() ;
                    //if c_num == 0 && false{

                    let c_num = CARDS2.iter().rev().position(|&x| x == *card).unwrap() ;
                    if c_num == 0 {
                        j += 1;
                    }else{
                    array[c_num] += 1;
                    cardrank += u64::pow(10,2*i as u32)*(c_num as u64);
                    }
                }
                
                let mut typ: [Vec::<u8>; 5] =  Default::default();
                for (i,num) in array.iter().enumerate(){
                    if *num>0{
                        typ[(*num-1) as usize].push(i as u8);
                    }
                }
                let num_typ : Vec<u8> = typ.iter().map(|x| x.len() as u8).collect::<Vec<u8>>();

                let mut typ_rank: u64 = 0;
                match j {
                    5 => typ_rank=6,
                    4 => typ_rank=6,
                    3 => {
                        if num_typ[0]==2 {typ_rank=5};
                        if num_typ[1]==1 {typ_rank=6};
                    },
                    2 => {
                        if num_typ[0]==3 {typ_rank=3};
                        if num_typ[1]==1 {typ_rank=5};
                        if num_typ[2]==1 {typ_rank=6};
                    },
                    1 => {
                        if num_typ[0]==4 {typ_rank=1};
                        if num_typ[1]==1 {typ_rank=3};
                        if num_typ[1]==2 {typ_rank=4};
                        if num_typ[2]==1 {typ_rank=5};
                        if num_typ[3]==1 {typ_rank=6};
                    },
                    0 => {
                        if num_typ[0]==5 {typ_rank=0};
                        if num_typ[1]==1 {typ_rank=1};
                        if num_typ[1]==2 {typ_rank=2};
                        if num_typ[2]==1 {typ_rank=3};
                        if num_typ[2]==1 && num_typ[1]==1 {typ_rank=4};
                        if num_typ[3]==1 {typ_rank=5};
                        if num_typ[4]==1 {typ_rank=6};
                    },
                    _ => println!("error")
                }

                cardrank += u64::pow(10,2*cards.len() as u32)*typ_rank;
                if typ_rank==0 &&   num_typ[0]!=5{              
                    println!("{0:?}",cards);
                    println!("{0}",typ_rank);
                    println!("{0}",cardrank);
                }
                games.push(Game{
                    ranking: cardrank,
                    score: lineiter.next().expect("no score").parse().expect("score not a number") 
                });
            }
        }
        games.sort_by(|a, b| a.ranking.cmp(&b.ranking));
        let mut result : u64 = 0;
        for (i,game) in games.iter().enumerate(){
            result += ((i as u64)+1)*(game.score as u64);
        }
        println!("score {0}",result);
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