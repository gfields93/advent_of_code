use std::str::from_utf8;

struct DiskMap {
    input: &'static str,
    disk_layout: Vec<Option<Disk>>,
}

#[derive(Debug, Clone)]
struct Disk {
    id: u64,
    size: u64,
}

impl DiskMap {
    fn new(input: &'static str) -> DiskMap {
        let total_size: usize = input
            .chars()
            .map(|d| d.to_digit(10).unwrap() as usize)
            .sum();
        let mut disk_layout = vec![None; total_size];
        let mut current_space = 0;
        let mut empty_space = false;
        let mut id = 0;
        for i in input.chars() {
            let size = i.to_digit(10).unwrap() as usize;
            if size == 0 {
                fun_name(&mut empty_space, &mut id);
                continue;
            }
            if empty_space {
                disk_layout[current_space..current_space + size].fill(None);
            } else {
                disk_layout[current_space..current_space + size].fill(Some(Disk {
                    id,
                    size: size as u64,
                }));
            }
            empty_space = !empty_space;
            if !empty_space {
                id += 1;
            }
            current_space += size;
        }
        DiskMap { input, disk_layout }
    }

    fn part_1(&self) -> u64 {
        let mut left = 0;
        let mut right = self.disk_layout.len() - 1;
        let mut file_layout: Vec<Option<Disk>> = self.disk_layout.clone();
        // self.println_disk_layout(&file_layout);
        while left < right {
            if let None = file_layout[left] {
                if let Some(disk) = &file_layout[right] {
                    file_layout[left] = Some(disk.clone());
                    file_layout[right] = None;
                    left += 1;
                    right -= 1;
                    // println!("Move disk {} from {} to {}", disk.id, right, left);
                    // self.println_disk_layout(&file_layout);
                } else {
                    right -= 1;
                    continue;
                }
            } else {
                left += 1;
            }
        }
        file_layout
            .iter()
            .enumerate()
            .map(|(idx, disk)| disk.as_ref().map_or(0, |f| f.id * idx as u64))
            .sum()
    }

    fn part_2(&self) -> u64 {
        let mut file_layout: Vec<Option<Disk>> = self.disk_layout.clone();
        let mut i = file_layout.len() - 1;
        // self.println_disk_layout(&file_layout);
        while i > 0 {
            if let Some(ref disk) = file_layout[i] {
                let slice_size = disk.size as usize;
                // let file_block = &mut file_layout[i - slice_size..i];
                let split = i.checked_sub(slice_size + 1);
                if let None = split {
                    break;
                }
                let (total_block, file_block) = file_layout.split_at_mut(i - slice_size + 1);
                for j in 0..total_block.len() {
                    if let None = total_block[j] {
                        let free_space = get_empty_block_size(total_block, j);
                        if free_space >= slice_size {
                            total_block[j..j + slice_size]
                                .swap_with_slice(&mut file_block[..slice_size]);
                            // self.println_disk_layout(&file_layout);
                            break;
                        }
                    }
                }
                i -= slice_size;
            } else {
                i -= 1;
            }
        }
        file_layout
            .iter()
            .enumerate()
            .map(|(idx, disk)| disk.as_ref().map_or(0, |f| f.id * idx as u64))
            .sum()
    }

    fn println_disk_layout(&self, file_layout: &[Option<Disk>]) {
        for disk in file_layout {
            if let Some(disk) = disk {
                print!("{}", disk.id);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn fun_name(empty_space: &mut bool, id: &mut u64) {
    *empty_space = !*empty_space;
    if !*empty_space {
        *id += 1;
    }
}

fn swap_disks(empty_slice: &mut [Option<Disk>], other: &mut [Option<Disk>]) {
    empty_slice[..].swap_with_slice(other);
}

fn get_empty_block_size(file_layout: &[Option<Disk>], idx: usize) -> usize {
    let mut empty_blocks = 0;
    for disk in &file_layout[idx..] {
        if disk.is_none() {
            empty_blocks += 1;
        } else {
            break;
        }
    }
    empty_blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("input/day9/test_input.txt");
        let disk_map = DiskMap::new(input);
        assert_eq!(disk_map.part_1(), 1928);
    }

    #[test]
    fn run_part_1() {
        let input = include_str!("input/day9/input.txt");
        let disk_map = DiskMap::new(input);
        println!("{}", disk_map.part_1());
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("input/day9/test_input.txt");
        let disk_map = DiskMap::new(input);
        assert_eq!(disk_map.part_2(), 2858);
    }

    #[test]
    fn run_part_2() {
        let input = include_str!("input/day9/input.txt");
        let disk_map = DiskMap::new(input);
        println!("{}", disk_map.part_2());
    }
}
