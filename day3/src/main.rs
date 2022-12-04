use std::collections::HashSet;

fn main() {
    let mut res: u64 = 0;
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("read line failed");
        let rucksack = String::from(line.trim());
        if rucksack.is_empty() {
            break;
        }
        let rucksack_size = rucksack.len();
        let comparment_size = rucksack_size/2;

        let mut rucksack = rucksack.chars();
        let mut first_compartment: HashSet<char> = HashSet::new();
        let mut index: usize = 0;
        while index < comparment_size {
            first_compartment.insert(
                match rucksack.next() {
                    Some(item) => item,
                    None => {
                        println!("rucksack has run out, this shouldn't happen");
                        break;
                    }
                }
            );
            index += 1;
        }
        while index < rucksack_size {
            let item = match rucksack.next() {
                Some(item) => item,
                None => {
                    println!("rucksack has run out, this shouldn't happen");
                    break;
                }
            };
            if first_compartment.contains(&item) {
                res += priority(item);
                break;
            }
            index += 1;
        }
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