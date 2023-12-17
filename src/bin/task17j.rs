use std::{
    cmp::{max, min},
    fs::File,
    io::{BufRead, BufReader}, convert,
    collections::HashMap,
};



const TEXT: &str = r#"11111111111
99999999991
99999999991
99999999991
99999999991
"#;

fn get_lines(is_test: bool) -> Box<dyn Iterator<Item = String>> {
    if is_test {
        return Box::new(TEXT.lines().map(|line| line.to_string()));
    } else {
        let file: File = File::open(FILE_PATH).unwrap();
        let reader = BufReader::new(file);
        Box::new(reader.lines().map(|line| line.unwrap().to_string()))
    }
}
const FILE_PATH: &str = "dat/task17.txt";  // 6,7


const p2_N0 : usize= 10;
#[derive(Debug,Clone,Copy)]
struct Path{
    x : i32,
    y : i32,
    dx: i32,
    dy: i32,
    cost : i32
}
#[derive(Debug,Clone,Copy)]
struct Gc{
    costn : [i32; 3],
    costw : [i32; 3],
    costs : [i32; 3],
    coste : [i32; 3],
}
#[derive(Debug,Clone,Copy)]
struct Gc2{
    costn : [i32; p2_N0],
    costw : [i32; p2_N0],
    costs : [i32; p2_N0],
    coste : [i32; p2_N0],
}
fn follow(mut p : Path , path : & mut Vec<(i32,i32)>, grid : & mut  Vec<Vec<u8>>, gridc : & mut  Vec<Vec<Gc>>, height:i32,width:i32 ) {
    
    for i in 0..3{
    p.x += p.dx;
    p.y -= p.dy;
    //println!("p : {:?}",p);
    if p.x<0 || p.x > width - 1 || p.y<0 || p.y > height -1 {
        return;
    }
    //path.push((p.x,p.y));
    p.cost += grid[p.y as usize][p.x as usize] as i32;
    let g_c = &mut gridc[p.y as usize][p.x as usize];

    let mut abort = 0;
    for j in i..3{
    if p.dx==-1 { if p.cost < g_c.costw[j]  { g_c.costw[j] = p.cost; } else {abort +=1;} }
    if p.dx==1  { if p.cost < g_c.coste[j]  { g_c.coste[j] = p.cost; } else {abort +=1;} }
    if p.dy==1  { if p.cost < g_c.costn[j]  { g_c.costn[j] = p.cost; } else {abort +=1;} }
    if p.dy==-1 { if p.cost < g_c.costs[j]  { g_c.costs[j] = p.cost; } else {abort +=1;} }
    }
    if abort==3-i {return;}

    if p.x == width -1 && p.y == height -1{
        if p.cost == 102 && false{
        println!("f {:?} {:?}",g_c,p);
            let mut tmp : Vec<Vec<char>> = Vec::new();
            for _i in 0..height{
                tmp.push(vec!['.';(width as u32).try_into().unwrap()]);
            }
            for (x,y) in path.iter(){
                tmp[*y as usize][*x as usize] = 'O';
            }
            for i in tmp.iter(){println!("{:?}",i);}
        }
        return;
    }
    //println!("p : {:?}",g_c);
    
    
    match p.dx  {
       -1 | 1 => {
            let mut p2 = p.clone();
            p2.dx = 0;
            p2.dy = 1;
            let mut p3 = p.clone();
            p3.dx = 0;
            p3.dy = -1;
            follow(p2, &mut path.clone(), grid, gridc, height, width);
            follow(p3, &mut path.clone(), grid, gridc, height, width);
            continue;
       },
       0 => (),
       _ => unreachable!() 
    }
    match p.dy  {
        -1 | 1 => {
             let mut p2 = p.clone();
             p2.dy = 0;
             p2.dx = 1;
             let mut p3 = p.clone();
             p3.dy = 0;
             p3.dx = -1;
             follow(p2, &mut path.clone(),grid, gridc, height, width);
             follow(p3, &mut path.clone(),grid, gridc, height, width);
 
        },
        _ => unreachable!() 
     }
    }

}

fn follow_p2(mut p : Path , path : & mut Vec<(i32,i32)>, grid : & mut  Vec<Vec<u8>>, gridc : & mut  Vec<Vec<Gc2>>, height:i32,width:i32 ) {
    
    for i in 0..p2_N0{
    p.x += p.dx;
    p.y -= p.dy;
    //println!("p : {:?}",p);
    if p.x<0 || p.x > width - 1 || p.y<0 || p.y > height -1 {
        return;
    }
    path.push((p.x,p.y));
    p.cost += grid[p.y as usize][p.x as usize] as i32;
    if i<3 {
        continue;
    }

    let g_c = &mut gridc[p.y as usize][p.x as usize];
    let mut abort = 0;
    for j in i..p2_N0{
    if p.dx==-1 { if p.cost < g_c.costw[j]  { g_c.costw[j] = p.cost; } else {abort +=1;} }
    if p.dx== 1 { if p.cost < g_c.coste[j]  { g_c.coste[j] = p.cost; } else {abort +=1;} }
    if p.dy== 1 { if p.cost < g_c.costn[j]  { g_c.costn[j] = p.cost; } else {abort +=1;} }
    if p.dy==-1 { if p.cost < g_c.costs[j]  { g_c.costs[j] = p.cost; } else {abort +=1;} }
    }
    if abort==p2_N0-i {return;}
    
    if p.x == width -1 && p.y == height -1{
        if p.cost < 999 && false{
        println!("f {:?}",p);
            let mut tmp : Vec<Vec<char>> = Vec::new();
            for _i in 0..height{
                tmp.push(vec!['.';(width as u32).try_into().unwrap()]);
            }
            for (x,y) in path.iter(){
                tmp[*y as usize][*x as usize] = 'O';
            }
            //for i in tmp.iter(){println!("{:?}",i.iter().collect::<String>());}
        }
        return;
    }
    //println!("p : {:?}",g_c);
    
    
    match p.dx  {
       -1 | 1 => {
            let mut p2 = p.clone();
            p2.dx = 0;
            p2.dy = 1;
            let mut p3 = p.clone();
            p3.dx = 0;
            p3.dy = -1;
            follow_p2(p2, &mut path.clone(), grid, gridc, height, width);
            follow_p2(p3, &mut path.clone(), grid, gridc, height, width);
            continue;
       },
       0 => (),
       _ => unreachable!() 
    }
    match p.dy  {
        -1 | 1 => {
             let mut p2 = p.clone();
             p2.dy = 0;
             p2.dx = 1;
             let mut p3 = p.clone();
             p3.dy = 0;
             p3.dx = -1;
             follow_p2(p2, &mut path.clone(),grid, gridc, height, width);
             follow_p2(p3, &mut path.clone(),grid, gridc, height, width);
 
        },
        _ => unreachable!() 
     }
    }

}


fn main() {
    let mut grid: Vec<Vec<u8>> = get_lines(false).map(|v| 
        v.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .filter(|v: &Vec<u8>| !v.is_empty()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    
    let mut gridc : Vec<Vec<Gc>> = Vec::new();
    let mut c0 = (height+width)*9;
    println!("max cost {}",c0);
    for y in 0..height{
        let mut tmp: Vec<Gc> = Vec::new();
        for x in 0..width{
            c0 = (x+y)*9; 
            tmp.push(Gc{coste:[c0;3],costn:[c0;3],costs:[c0;3],costw:[c0;3]})
        }
        //gridc.push(vec![Gc{straight:(3,3,3,3),coste:[c0;3],costn:[c0;3],costs:[c0;3],costw:[c0;3]};(width as u32).try_into().unwrap()]);
        gridc.push( tmp );
    }
    //for i in grid.iter(){println!("{:?}",i);}
    println!("");
    let p = Path { x: 0, y: 0, dx: 1, dy: 0, cost: 0 };
    let mut path : Vec<(i32,i32)> = Vec::new();
    follow(p,&mut path, &mut grid, &mut gridc, height, width);
    let p = Path { x: 0, y: 0, dx: 0, dy: -1, cost: 0 };
    let mut path : Vec<(i32,i32)> = Vec::new();
    follow(p,&mut path, &mut grid, &mut gridc, height, width);
    //for i in gridc.iter(){println!("{:?}",i.iter().map(|x| x.coste).collect::<Vec<i32>>());}
    //for i in gridc.iter(){println!("{:?}",i);}
    let fc = gridc[height as usize -1][width as usize - 1];
    let cost = min(min(fc.coste.iter().min(),fc.costw.iter().min()),
                                 min(fc.costs.iter().min(),fc.costn.iter().min()));
    println!("{:?} {:?}",fc,cost);

    let mut gridc2 : Vec<Vec<Gc2>> = Vec::new();
    for y in 0..height{
        let mut tmp: Vec<Gc2> = Vec::new();
        for x in 0..width{
            c0 = (x+y)*9; 
            tmp.push(Gc2{coste:[c0;p2_N0],costn:[c0;p2_N0],costs:[c0;p2_N0],costw:[c0;p2_N0]})
        }
        //gridc.push(vec![Gc{straight:(3,3,3,3),coste:[c0;3],costn:[c0;3],costs:[c0;3],costw:[c0;3]};(width as u32).try_into().unwrap()]);
        gridc2.push( tmp );
    }
    //for i in grid.iter(){println!("{:?}",i);}
    println!("");
    let p = Path { x: 0, y: 0, dx: 1, dy: 0, cost: 0 };
    let mut path : Vec<(i32,i32)> = Vec::new();
    follow_p2(p,&mut path, &mut grid, &mut gridc2, height, width);
    let p = Path { x: 0, y: 0, dx: 0, dy: -1, cost: 0 };
    let mut path : Vec<(i32,i32)> = Vec::new();
    follow_p2(p,&mut path, &mut grid, &mut gridc2, height, width);
    //for i in gridc.iter(){println!("{:?}",i.iter().map(|x| x.coste).collect::<Vec<i32>>());}
    //for i in gridc.iter(){println!("{:?}",i);}
    let fc = gridc2[height as usize -1][width as usize - 1];
    let cost = min(min(fc.coste.iter().min(),fc.costw.iter().min()),
                                 min(fc.costs.iter().min(),fc.costn.iter().min()));
    println!("{:?} {:?}",fc,cost);

}
