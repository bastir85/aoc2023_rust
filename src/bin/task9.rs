use core::num;
use std::fs::File;
use std::io::{self, BufRead};
use std::net::TcpListener;
use std::ops::RangeBounds;
use std::path::Path;
use std::{println, vec};
use std::collections::HashMap;
use std::cmp::max;


struct Lr{
    left : String,
    right : String,
}

fn fac(i: u64,mut j : u64) -> f64 {
    if j>i {println!("error to high {} for {}",j,i);return 0.; }
    j = i - j + 1;
    (j..=i).product::<u64>() as f64
}

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./dat/task9.txt") {
        // Consumes the iterator, returns an (Optional) String
        
        let mut res : i64 = 0;
        let mut res2 : i64 = 0;
        for (i,line) in lines.enumerate(){
            if let Ok(line) = line {
                let mut diff_v : Vec<Vec<i64>> = Vec::new();
                diff_v.push(
                    line.split(' ').filter_map(|x| x.parse::<i64>().ok()).collect()
                );
                let mut degree : usize = 0;
                loop{
                    let numbers : &Vec<_> = &diff_v[degree];
                    let mut tmp : Vec<i64> = vec![0;(numbers.len()-1 ) as usize];
                    for n in 0..numbers.len()-1{
                        tmp[n] = numbers[n+1] - numbers[n];
                    }
                    if tmp.iter().all(|&x| x==0){
                        break;
                    }
                    diff_v.push(tmp);
                    degree += 1;
                }
                /* 
                let mut last : i64 = 0;
                for tmp in diff_v.iter().rev(){
                    last = tmp.last().unwrap() + last;
                } 
                */
                res += diff_v.iter().rev().fold(0,|acc, x| x.last().unwrap() + acc);
                res2 += diff_v.iter().rev().fold(0,|acc, x| x.first().unwrap() - acc);
            }
        }
        println!("{} {}",res,res2);
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

/*
                let mut poly : Vec<f64> = vec![0.;degree+1];
                for d in (0..degree+1).rev(){
                    let numbers : &Vec<_> = &diff_v[d];
                    println!("d {} maxd {}: {:?} : {:?}",d,degree,numbers,poly);
                    if d==degree {
                        poly[d] = numbers[0] as f64 / fac(degree as u64, d as u64);
                        println!("fac {:?}",fac(degree as u64, d as u64));
                    }else{
                    println!("poly {:?}",
                    poly.iter().enumerate().filter(|(pd,p_val)| pd>&d ).map(
                        |(p,p_val)| fac(p as u64,d as u64)*p_val
                        ).collect::<Vec<f64>>()
                    );
                    println!("fac {:?}",
                    poly.iter().enumerate().filter(|(pd,p_val)| pd>&d ).map(
                        |(p,p_val)| fac(p as u64,d as u64)
                        ).collect::<Vec<f64>>()
                    );
                    

                    let tmp : Vec<_> = numbers.iter().enumerate().map(|(n,n_val)|
                    poly.iter().enumerate().filter(|(pd,p_val)| pd>&d ).map(
                        |(p,p_val)| fac(p as u64,d as u64)*p_val*f64::powf(n as f64 + d as f64*0.5,(p - d) as f64) 
                        ).sum()
                    ).collect::<Vec<f64>>();
                    let mut tmp2 : Vec<f64> = Vec::new();
                    for n in 0..numbers.len(){
                        tmp2.push( numbers[n] as f64 - tmp[n] );
                    } 
                    println!("func {:?}",tmp);
                    println!("res {:?}", tmp2);
                    poly[d] = tmp2[0] / fac(d as u64,d as u64);
                }
            }

 */