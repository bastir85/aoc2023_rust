use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, num,
    collections::HashMap, usize,
};

const TEXT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
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
const FILE_PATH: &str = "dat/task15.txt";


fn gen_hash(acc:u8,c:char) ->u8 {
    ((acc as u16 + c as u16)*17 % 256) as u8
}

fn main() {
    let mut total = 0;
    let mut total2 = 0;

    let mut nn = 0;
    let binding = get_lines(false).collect::<String>();
    let cmds : Vec<&str>= binding.split(',').collect::<Vec<&str>>();    
    let hash  : Vec<u8>= cmds.iter().map(|s| 
            s.chars().fold(0,gen_hash )
        ).collect();
    println!("cmds {:?}",cmds); 
    println!("hash {:?}",hash); 
    println!("res {:?}",hash.iter().map(|&x| x as u32).sum::<u32>()); 
    let mut boxes : Vec<Vec<(&str,u8)>> = Vec::new();
    for _i in 0..256{
        boxes.push(Vec::new());
    }
    for cmd in cmds.iter(){
        if let Some(p) = cmd.find('-'){
            let label = cmd.get(0..p).unwrap();
            //let f = cmd.get(p+1..cmd.len()).unwrap_or("0").parse::<u8>().unwrap_err(0);
            let n_box = label.chars().fold(0,gen_hash) as usize;
            if let Some(n_remove) = boxes[n_box].iter().position(|x| x.0==label){
                boxes[n_box].remove(n_remove);
            }
        }
        if let Some(p) = cmd.find('='){
            let label = cmd.get(0..p).unwrap();
            let f = cmd.get(p+1..cmd.len()).unwrap_or("0").parse::<u8>().unwrap();
            let n_box = label.chars().fold(0,gen_hash) as usize;
            if let Some(n_change) = boxes[n_box].iter().position(|x| x.0==label){
                boxes[n_box][n_change].1 = f;
            }else{
                boxes[n_box].push((label,f));
            }
        }
    }
    for (bi,b) in boxes.iter().enumerate(){
        if b.len()!=0{
            //println!("box {}: {:?}",bi,b);
        }
    }
    println!("res2 {}",boxes.iter().enumerate().map(|(xi,x)| 
                x.iter().enumerate().map(|(li,l)| (xi+1)*(li+1)*l.1 as usize ).sum::<usize>() 
            ).sum::<usize>())
}
