pub(crate) mod day1 {
    use std::{collections::HashMap, str::from_utf8};

    pub fn part_1(content: &[u8]) -> u32 {
        let (mut list1, mut list2): (Vec<_>, Vec<_>) = content
            .split(|&f| f == b'\n')
            .map(|pair| {
                let mut split = pair.split(|&f| f == b' ').filter(|f| !f.is_empty());
                let x = split.next();
                let y = split.next();
                (
                    x.and_then(|i| from_utf8(i).ok())
                        .and_then(|f| f.parse::<u32>().ok())
                        .unwrap(),
                    y.and_then(|i| from_utf8(i).ok())
                        .and_then(|f| f.parse::<u32>().ok())
                        .unwrap(),
                )
            })
            .unzip();

        list1.sort();
        list2.sort();
        list1
            .into_iter()
            .zip(list2.into_iter())
            .map(|(x, y)| x.abs_diff(y))
            .sum()
    }

    pub fn part_2(content: &[u8]) -> u32 {
        let (mut list1, mut list2): (Vec<_>, Vec<_>) = content
            .split(|&f| f == b'\n')
            .map(|pair| {
                let mut split = pair.split(|&f| f == b' ').filter(|f| !f.is_empty());
                let x = split.next();
                let y = split.next();
                (
                    x.and_then(|i| from_utf8(i).ok())
                        .and_then(|f| f.parse::<u32>().ok())
                        .unwrap(),
                    y.and_then(|i| from_utf8(i).ok())
                        .and_then(|f| f.parse::<u32>().ok())
                        .unwrap(),
                )
            })
            .unzip();

        let mut number_counts: HashMap<u32, u32> = HashMap::new();
        for num in list2 {
            if let Some(count) = number_counts.get_mut(&num) {
                *count += 1;
            } else {
                number_counts.insert(num, 1);
            }
        }
        list1
            .into_iter()
            .map(|mut f| {
                if let Some(count) = number_counts.get(&f) {
                    return f * *count;
                }
                0
            })
            .sum()
    }
}
