use std::collections::HashMap;

fn main() {
    let mut res: u64 = 0;
    let mut pos :u32 = 0; // {0, 1, 2}
    let mut current_set: HashMap<char, u32> = HashMap::new();
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("read line failed");
        let rucksack = String::from(line.trim());
        if rucksack.is_empty() {
            break;
        }
        let mut rucksack = rucksack.chars();
        loop {
            let item = match rucksack.next() {
                Some(item) => item,
                None => break,
            };
            match pos {
                0 => {
                    current_set.insert(item, 0);
                }
                1 => {
                    match current_set.get(&item) {
                        Some(value) => {
                            if *value == 0 {
                                current_set.insert(item, 1);
                            }
                        }
                        None => ()
                    }
                }
                2 => {
                    match current_set.get(&item) {
                        Some(value) => {
                            if *value == 1 {
                                res += priority(item);
                                current_set.clear();
                                break;
                            }
                        }
                        None => ()
                    }
                }
                _ => {
                    panic!("invalid pos {}, must be 0 1 2", pos)
                }
            }
        }
        pos = (pos + 1)%3
    }
    println!("result is {}", res)
}

fn priority(item: char) -> u64 {
    if 'A' <= item && item <= 'Z' {
        27 + (item as u64 - 'A' as u64)
    } else if 'a' <= item && item <= 'z' {
        1 + (item as u64 - 'a' as u64)
    } else {
        panic!("bad item: {}", item)
    }
}