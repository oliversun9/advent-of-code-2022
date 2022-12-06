fn main() {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new()];
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("read line failed");
        let input = String::from(line.trim());
        if input.is_empty() {
            break;
        }
        if input.chars().next().expect("yea") == '1' {
            continue;
        }
        let stack_count = (line.len() + 1)/4;
        if stacks.len() == 1 {
            for _ in 1..=stack_count {
                stacks.push(Vec::new());
            }
        }
        for i in 1..=stack_count {
            let c = input.chars().nth(4 * i - 3).expect("msg");
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    for i in 1..stacks.len() {
        stacks[i].reverse();
    }
    println!("stacks are {:#?}", stacks);
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("read line failed");
        let input = String::from(line.trim());
        if input.is_empty() {
            break;
        }
        let mut tokens = input.split(" ");
        let amount: usize = tokens
            .nth(1)
            .expect("bad token")
            .parse::<usize>()
            .expect("not an int");
        let from = tokens
            .nth(1)
            .expect("bad token")
            .parse::<usize>()
            .expect("not an int");
        let to = tokens
            .nth(1)
            .expect("bad token")
            .parse::<usize>()
            .expect("not an int");
        // for _ in 1..=amount {
        //     let top = stacks[from].pop().expect("popping from empty");
        //     stacks[to].push(top);
        // }
        for i in stacks[from].len()-amount..stacks[from].len() {
            let item = stacks[from][i];
            stacks[to].push(item)
        }
        stacks[from] = stacks[from][0..stacks[from].len()-amount].to_vec()
    }
    println!("result is:");
    let mut res = String::new();
    for i in 1..stacks.len() {
        res.push(*stacks[i].last().expect("empty stack in the end"))
    }
    println!("{}", res)
}