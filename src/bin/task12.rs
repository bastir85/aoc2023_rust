use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, num,
    collections::HashMap,
};

const TEXT: &str = "?????????? 1,2,1";

//from https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/12.rs
fn possible_ways(cache: &mut HashMap<(usize, usize, usize), usize>, s: &[char], within: Option<usize>, remaining: &[usize]) -> usize {
    if s.is_empty() {
      return match (within, remaining.len()) {
        (None, 0) => 1,
        (Some(x), 1) if x == remaining[0] => 1,
        _ => 0
      };
    }
    if within.is_some() && remaining.is_empty() {
      return 0;
    }
  
    let key = (s.len(), within.unwrap_or(0), remaining.len());
    if let Some(&x) = cache.get(&key) {
      return x;
    }
  
    let ways = match (s[0], within) {
      ('.', Some(x)) if x != remaining[0] => 0,
      ('.', Some(_)) => possible_ways(cache, &s[1..], None, &remaining[1..]),
      ('.', None)    => possible_ways(cache, &s[1..], None, remaining),
      ('#', Some(_)) => possible_ways(cache, &s[1..], within.map(|x| x+1), remaining),
      ('#', None)    => possible_ways(cache, &s[1..], Some(1), remaining),
      ('?', Some(x)) => {
        let mut ans = possible_ways(cache, &s[1..], within.map(|x| x+1), remaining);
        if x == remaining[0] {
          ans += possible_ways(cache, &s[1..], None, &remaining[1..])
        }
        ans
      }
      ('?', None) =>
        possible_ways(cache, &s[1..], Some(1), remaining) +
        possible_ways(cache, &s[1..], None, remaining),
      _ => unreachable!(),
    };
    cache.insert(key, ways);
    ways
  }

fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        Box::new(reader.lines().map(|line| line.unwrap().to_string()))
    }
}
const FILE_PATH: &str = "dat/task12.txt";

fn get_numbers(testpattern : &Vec<char>) -> Vec<u32>{
    let mut count = 0;
    let mut counts : Vec<u32> = Vec::new();
    for c in testpattern.iter(){
        if *c=='#'{
            count +=1;
        }else if count>0{
            counts.push(count);
            count = 0;
        }
    }
    if count>0{
        counts.push(count);
    }
    counts
}


fn replace_i(cache: &mut HashMap<(i8, usize), u64>, num : i8 ,start : usize, qs : &Vec<usize>,  testpattern : &mut Vec<char>,numbers :&Vec<u32>) -> u64{
    //println!("start {}  num {} P {:?}",start,num,&testpattern[0..start]);
    if let Some(&x) = cache.get(&(num,start)) {
        return x;
    }
    let mut ret : u64 = 0;
    for (ii,i) in qs[start..qs.len()-num as usize].iter().enumerate(){
        testpattern[*i] = '#';
        if num==0 {
            //println!("Test {:?}",testpattern);
            let counts = get_numbers(testpattern);
            //println!("{:?} {}",testpattern,&counts==numbers);
            if &counts==numbers{
                ret += 1;
            }
        }
        else{
            let next = if qs[start + ii + 1] == testpattern.len() {
                    testpattern.len()
                }else{
                    qs[start + ii + 1]
                };
            let tmp : Vec<char> = testpattern[0..next].to_vec();
            let counts = get_numbers(&tmp);
            let mut run = true;
            for i in 0..counts.len(){
                if i<numbers.len(){
                    if counts[i] > numbers[i]{
                        run = false;
                        break;
                    }else if counts[i] < numbers[i]  && i<counts.len()-1{
                        run = false;
                        break;                   
                    }
                }
            }
            //let run = true;
            //println!("Testi {:?} {:?} {:?} {} {} {}",tmp, counts, numbers, run, next,num);
            if run{
                ret += replace_i(cache, num-1,ii+1+start, qs, testpattern,numbers);
            }
        }
        testpattern[*i] = '.';
    }
    
    cache.insert((num,start), ret);
    
    ret
}


fn main() {
    let mut total = 0;
    let mut total2 = 0;

    let mut nn = 0;
    for line in get_lines(false){
        let (pattern,numbers )= line.split_once(' ').unwrap();
        let numbersA : Vec<_> = numbers.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
        let patternA: Vec<_> = pattern.chars().collect();
        let mut numbers: Vec<u32> = Vec::new();
        let mut pattern: Vec<char> = Vec::new();
        if true{
            for _i in 0u8..4{
                numbers.extend(numbersA.iter());
                pattern.extend(patternA.iter());
                pattern.push('?');
            } 
            numbers.extend(numbersA.iter());
            pattern.extend(patternA.iter());
        }else{
            numbers.extend(numbersA.iter());
            pattern.extend(patternA.iter());
        }
        let qs : Vec<_> = pattern.iter().enumerate().
            filter_map(|(i,c)| if *c=='?' {Some(i)} else{None}).collect();
        let temppattern : Vec<_> = pattern.clone().iter().map(|&x|  if x=='?' {'.'}else{x}).collect();
        let mut testpattern = temppattern.clone();
        //println!("{:?} {:?}",pattern,qs);
        let missing : u32= numbers.iter().sum::<u32>() - testpattern.iter().filter(|&&x| x=='#').count() as u32;
        if missing==0{
            total += 1;
            continue;
        }
        let mut cache = HashMap::new();
        let res = replace_i(&mut cache, (missing - 1) as i8,0 as usize, &qs, &mut testpattern,&numbers);
        
        let mut cache2 = HashMap::new();
        let tmp = numbers.iter().map(|&x| x as usize).collect::<Vec<usize>>();
        let res2 = possible_ways(&mut cache2, &pattern, None, &tmp);
        if res2  != res as usize{
            println!("m:{} R:{} l:{} r:{}",missing, res, nn, res2);
        }
        nn += 1;
        total += res as u64;
        total2 += res2 as u64;

    }
    println!("J{}",total);
    println!("I{}",total2);



}
