fn main() {
    let mut res: i32 = 0;
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("read line failed");
        let opponent_shape = match line.chars().nth(0) {
            Some(opponent_input) => {
                match opponent_input {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    _ => {
                        println!("read invalid opponent move: {}", opponent_input);
                        break;
                    }
                }
            }
            None => {
                println!("none opponent move read");
                break;
            }
        };
        let expected_outcome = match line.chars().nth(2) {
            Some(second_input) => {
                match second_input {
                    'X' => 2,
                    'Y' => 0,
                    'Z' => 1,
                    _ => {
                        println!("read invalid own outcome: {}", second_input);
                        break;
                    }
                }
            }
            None => {
                println!("none own outcome read");
                break;
            }
        };
        res += match expected_outcome {
            0 => 3,
            1 => 6,
            2 => 0,
            _ => {
                break;
            }
        };
        res += (expected_outcome + opponent_shape)%3 + 1;
    }
    println!("result is {}", res)
}

// fn get_shape_point(shape: i32) -> i32 {
//     shape + 1
// }

// fn get_outcome_point(opponent_shape: i32,own_shape: i32) -> i32 {
//     match (own_shape - opponent_shape + 3)%3 {
//         0 => 3,
//         1 => 6,
//         2 => 0,
//         _ => {
//             panic!("impossible");
//         }
//     }
// }