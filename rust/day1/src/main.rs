use std::{fs::File, path::Path, io::{BufReader, BufRead}};


fn main() {
    // println!("Hello, world!");
    let input = "input.txt";
    let fp = File::open(Path::new(input)).expect("Error opening file");

    let buff_read = BufReader::new(fp);
    let data: Vec<u16> = buff_read.lines()
        .map(|x| x.unwrap().parse::<u16>().unwrap())
        .into_iter()
        .collect();

    let mut increases = 0;
    for i in 0..data.len()-1 {
        if data.get(i) < data.get(i+1) {
            increases += 1;
        }
    }
    println!("{:?} single increases", increases);

    let mut arr3: Vec<[u16; 3]> = Vec::new();
    for n in 0..data.len()-2 {
        arr3.push([data[n], data[n+1], data[n+2]]);
    }
    increases = 0;
    for n in 0..arr3.len()-1 {
        if arr3[n].iter().sum::<u16>() < arr3[n+1].iter().sum() {
            increases += 1;
        }
    } 

    println!("{:?} group increases", increases);

}
