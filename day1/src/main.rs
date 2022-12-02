fn main() {
    let mut cur: u64 = 0;
    let mut highest_n: [u64; 3] = [0, 0, 0];
    let mut index_of_lowest: usize = 2;
    let mut was_last_line_blank = false;
    loop {
        let mut cal = String::new();
        match std::io::stdin().read_line(&mut cal) {
            Ok(_) => {
                if cal.trim().is_empty() {
                    if was_last_line_blank {
                        break
                    }
                    if cur > highest_n[index_of_lowest] {
                        highest_n[index_of_lowest] = cur
                    }
                    for i in 0..highest_n.len() {
                        if highest_n[i] < highest_n[index_of_lowest] {
                            index_of_lowest = i;
                        }
                    }
                    cur = 0;
                    was_last_line_blank = true;
                    continue;
                }
                cur += cal.trim().parse::<u64>().expect("must be either a number or blank line or eof");
                was_last_line_blank = false;
            }
            Err(_) => break,
        }
    }
    let mut sum :u64 = 0;
    for i in 0..highest_n.len() {
        sum += highest_n[i];
    }
    println!("the highest is {sum}");
}
