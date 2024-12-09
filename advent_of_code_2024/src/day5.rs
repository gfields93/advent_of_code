use std::collections::{HashMap, HashSet};

fn get_rules(input: &str) -> HashMap<u32, Vec<u32>> {
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        let left = parts[0].parse::<u32>().unwrap();
        let right = parts[1].parse::<u32>().expect("msg rule split error");
        if rules.contains_key(&right) {
            rules.get_mut(&right).unwrap().push(left);
        } else {
            rules.insert(right, vec![left]);
        }
    }

    rules
}

fn get_rules_v(input: &str) -> HashMap<u32, HashSet<u32>> {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        let left = parts[0].parse::<u32>().unwrap();
        let right = parts[1].parse::<u32>().unwrap();
        if rules.contains_key(&left) {
            rules.get_mut(&left).unwrap().insert(right);
        } else {
            let mut new_set = HashSet::new();
            new_set.insert(right);
            rules.insert(left, new_set);
        }
    }

    rules
}

fn get_updates(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.split(',').map(|s| s.parse::<u32>().unwrap()).collect())
        .collect()
}

fn check_rule(not_allowed: &[u32], list_of_pages: &[u32]) -> bool {
    let page_set: HashSet<u32> = list_of_pages.iter().cloned().collect();
    for &page in not_allowed {
        if page_set.contains(&page) {
            return false;
        }
        // page_set.extend(rules.get(&page).unwrap().iter().cloned());
    }
    true
}

fn part_1(new_rules: &str, new_updates: &str) -> u32 {
    let rules = get_rules(new_rules);
    let updates = get_updates(new_updates);
    let mut sum = 0;
    for update in updates {
        let invalid = is_update_valid(&update, &rules);
        if !invalid {
            let mid = update.len() / 2;
            sum += update[mid];
        }
    }
    sum
}

fn is_update_valid(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut invalid = false;
    for i in 0..update.len() - 1 {
        let rule = rules.get(&update[i]);
        if rule.is_some_and(|f| !check_rule(&f[..], &update[i + 1..])) {
            invalid = true;
            break;
        }
    }
    invalid
}

fn part_2(new_rules: &str, new_updates: &str) -> u32 {
    let rules = get_rules(new_rules);
    let mut updates = get_updates(new_updates);
    let mut sum = 0;

    for idx in 0..updates.len() {
        let current_update = updates.get_mut(idx).unwrap();
        let mut invalid = is_update_valid(&current_update[..], &rules);
        // Check if current is update is invalid, and rotate it to the left until it's valid
        if !invalid {
            continue;
        }

        // Get idx pointing to the first character of the update
        let mut update_i = 0;
        while invalid && update_i < current_update.len() {
            let rule_maybe = rules.get(&current_update[update_i]);
            let mut move_i_forward = true;
            if let Some(rule) = rule_maybe {
                let rule_set = rule.iter().cloned().collect::<HashSet<u32>>();
                let mut current_idx = current_update.len() - 1;
                while current_idx > update_i {
                    if rule_set.contains(&current_update[current_idx]) {
                        dbg!(&current_update);
                        current_update[update_i..=current_idx].rotate_left(1);
                        dbg!(&current_update);
                        move_i_forward = false;
                        break;
                    }
                    current_idx -= 1;
                }
            }
            invalid = is_update_valid(&current_update[..], &rules);
            if move_i_forward {
                update_i += 1;
            }
        }
        let mid = updates[idx].len() / 2;
        sum += updates[idx][mid];
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let new_rules = include_str!("input/day5/test_rules.txt");
        let new_updates = include_str!("input/day5/test_updates.txt");
        println!("{}", part_1(new_rules, new_updates)); // 12
    }

    // #[test]
    // fn test_part_2() {
    //     let new_rules = include_str!("input/day5/test_rules.txt");
    //     let new_updates = include_str!("input/day5/test_updates.txt");
    //     println!("{}", part_2(new_rules, new_updates)); // 19208
    // }

    #[test]
    fn test_part_2_2() {
        let new_rules = include_str!("input/day5/new_rules.txt");
        let new_updates = include_str!("input/day5/new_updates.txt");
        println!("{}", part_2(new_rules, new_updates)); // 19208
    }
}
