// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
use std::io;

fn main() {
    println!("Input Integers!");

    let mut user_input_list = String::new();
    let mut integer_list: Vec<u32> = Vec::new();

    'input_user: loop {
        io::stdin()
            .read_line(&mut user_input_list)
            .expect("Failed to read line");

        for number in user_input_list.split_whitespace() {
            match number.trim().parse() {
                Ok(num) => integer_list.push(num),
                Err(_) => {
                    println!("There is not integer in your inputs. Please re-type.");
                    user_input_list = String::new();
                    continue 'input_user;
                }
            };
        }

        break;
    }

    integer_list.sort();

    let medium_index = integer_list.len() / 2;

    let mut hashm = HashMap::new();

    for n in &integer_list {
        let count = hashm.entry(n).or_insert(0);
        *count += 1;
    }

    let (mode_num, count) = hashm.iter().max_by_key(|entry| entry.1).unwrap();

    for (key, value) in &hashm {
        println!("{key}: {value}");
    }

    println!("{medium_index}");
    println!("{mode_num}, {count}");
}
