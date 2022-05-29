use core::num;
use std::{fs::File, path::Path, io::{BufReader, BufRead}};

fn main() {
    let input = "input2.txt";
    let file = File::open(Path::new(input)).expect("Error opening file...");
    let reader: Vec<_> = BufReader::new(file).lines()
        .map(|s| s.unwrap().chars().collect::<Vec<_>>())
        .collect();

    part1(reader.clone());
    part2(reader)
}

fn part1(src: Vec<Vec<char>>) {
    let func_copy = src;

    let gamma_rate = find_gamma(&func_copy);
    let epsilon_rate = invert_binary_val(&gamma_rate);
    
    println!("gamma_rate * epsilon_rate = {}", u32::from_str_radix(&gamma_rate.into_iter().collect::<String>(), 2).unwrap() * u32::from_str_radix(&epsilon_rate.into_iter().collect::<String>(), 2).unwrap())
}

fn find_gamma(func_copy: &[Vec<char>]) -> Vec<char> {
    let mut gamma_rate = Vec::<char>::new();
    for row in 0..func_copy[0].len() {
        let mut zero = 0;
        let mut one = 0;
         for col_val in 0..func_copy.len() {
            if func_copy[col_val][row] == '1' {
                one += 1;
            } else {
                zero += 1;
            }
         }
         if zero > one {
             gamma_rate.push('0');
         } else {
             gamma_rate.push('1');
         }
    }
    gamma_rate
}

fn invert_binary_val(binary_vec: &[char]) -> Vec<char> {
    let epsilon_rate = binary_vec.iter()
        .fold(Vec::new(), |mut acc, x| {
            if *x == '1' { acc.push('0') } else { acc.push('1') };
            acc
        });
    epsilon_rate
}

fn part2(src: Vec<Vec<char>>) {
    // Calculate oxygen generator rate
    let mut data_copy = src.clone();
    let num_of_bits = data_copy[0].len();
    for bit in 0..num_of_bits {
        let current_gamma = find_gamma(&data_copy);
        // data_copy = data_copy.drain_filter(|f| -> bool { f[bit] == current_gamma[bit]})
        //                     .collect();
        data_copy = {
            let mut new_vec = Vec::<Vec<char>>::new();

            for arr in data_copy {
                if arr[bit] == current_gamma[bit]{
                    new_vec.push(arr);
                }
            }
            new_vec
        };
        if data_copy.len() == 1 { break; }
    };
    let oxygen_rate = &data_copy[0];
    println!("oxygen generator rate {:?}", oxygen_rate);

    let mut data_copy_2 = src;
    for bit in 0..num_of_bits {
        let current_gamma = invert_binary_val(&find_gamma(&data_copy_2));
        // data_copy = data_copy.drain_filter(|f| -> bool { f[bit] == current_gamma[bit]})
        //                     .collect();
        data_copy_2 = {
            let mut new_vec = Vec::<Vec<char>>::new();

            for arr in data_copy_2{
                if arr[bit] == current_gamma[bit]{
                    new_vec.push(arr);
                }
            }
            new_vec
        };
        if data_copy_2.len() == 1 { break; }
    };
    let co2_rate = &data_copy_2[0];
    println!("co2 rate {:?}", co2_rate);

    println!("Product of oxygen generator rate and co2 scrubber rate = {:?}",
    u32::from_str_radix(&oxygen_rate.iter().collect::<String>(), 2).unwrap() * u32::from_str_radix(&co2_rate.iter().collect::<String>(), 2).unwrap())

}