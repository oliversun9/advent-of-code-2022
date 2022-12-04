fn main() {
    let mut res: u64 = 0;
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("read line failed");
        let input = String::from(line.trim());
        if input.is_empty() {
            break;
        }
        let mut ranges = input.split(",");
        let range_1 = match ranges.next() {
            Some(range) => get_bounds(String::from(range)),
            None => break,
        };
        let range_2 = match ranges.next() {
            Some(range) => get_bounds(String::from(range)),
            None => break,
        };
        // if contains_range(range_1, range_2) || contains_range(range_2, range_1) {
        //     res += 1
        // }
        if ranges_overlap(range_1, range_2) {
            res += 1;
        }
    }
    println!("result is {}", res)
}

fn get_bounds(s: String) -> (u32, u32) {
    let mut bounds = s.split("-");
    let lower = match bounds.next() {
        Some(lower) => String::from(lower)
            .parse::<u32>()
            .expect("lower not a number"),
        None => panic!("no lower: {}", s),
    };
    let higher = match bounds.next() {
        Some(higher) => String::from(higher)
            .parse::<u32>()
            .expect("higher not a number"),
        None => panic!("no higher: {}", s),
    };
    (lower, higher)
}

// fn contains_range(r1: (u32, u32), r2: (u32, u32)) -> bool {
//     return r1.0 <= r2.0 && r2.1 <= r1.1
// }

fn ranges_overlap(r1: (u32, u32), r2: (u32, u32)) -> bool {
    return !(
        r1.1 < r2.0 ||
        r2.1 < r1.0
    )
}
