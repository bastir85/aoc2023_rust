use std::{io::{BufReader, BufRead, Error}, fs::File};

const TEXT: &str="1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

const TEXT2: &str ="two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

const LOOKUP: [(&str, char); 9] = [("one", '1'),("two", '2'),("three", '3'),("four", '4'),("five", '5'),
                                   ("six", '6'), ("seven", '7'),("eight", '8'),("nine", '9')];


fn findNumber(line:String)-> Option<char>{
    let mut substr = String::new();
    for chr in line.chars(){
        if chr.is_numeric(){
            return Some(chr);
        } else {
            substr.push(chr);
            for (a, b) in LOOKUP{
                if substr.find(a).is_some(){
                    return Some(b);
                }
            }
        }
    }
    return None
}

fn findNumberR(line:String)-> Option<char>{
    let mut substr = String::new();
    for chr in line.chars(){
        if chr.is_numeric(){
            return Some(chr);
        } else {
            substr.insert(0, chr);
            for (a, b) in LOOKUP{
                if substr.rfind(a).is_some(){
                    return Some(b);
                }
            }
        }
    }
    return None
}
fn main() ->  Result<(), Error> {
    let mut result:u16 = 0;

    let file = File::open("/workspaces/rust/hello_cargo/dat/task1.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
    //for line in TEXT2.lines(){
        //let line = line.to_string);
        let line = line.unwrap();
        let rev_line = line.chars().rev().collect();

        let first_number = findNumber(line).unwrap();
        let last_number = findNumberR(rev_line).unwrap();

        let mut tmp = String::new();
        tmp.push(first_number);
        tmp.push(last_number);
        result += tmp.parse::<u16>().unwrap();
        //let real_number =(numbers[0] + numbers[1]);
        println!("{0}{1} = {2}",first_number,last_number, tmp);
    }
    println!("Result {0}", result);

    Ok(())
}