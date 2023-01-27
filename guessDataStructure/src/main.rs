/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */

use std::io::stdin;

mod priority;
mod queue;
mod stack;

/// Kattis calls main function to run your solution.
fn main() {
    /*
    PLAN:
    Take input as a while loop.

    Simulate each type of data-structure
    and check if they followed the process given.

    For input: we need to while w
    */

    //we want to read
    /* add code here ... */

    let mut line = String::new();
    let mut _stack = stack::Stack::new();
    let mut _queue = queue::Queue::new();
    let mut _priority = priority::Priority::new();

    stdin().read_line(&mut line).unwrap();
    let mut lines_left_of_case: usize = line.trim().parse::<usize>().unwrap();
    line.clear();
    while stdin().read_line(&mut line).unwrap() > 0 {
        line = line.trim().to_string();
        if lines_left_of_case > 0 {
            let (command, value) = line.split_once(" ").unwrap();
            let x = value.parse::<u8>().unwrap();
            if command == "1" {
                _stack.insert(x);
                _queue.insert(x);
                _priority.insert(x as u16);
            } else {
                _stack.is_correct_retrieve(x);
                _queue.is_correct_retrieve(x);
                _priority.is_correct_retrieve(x as u16);
            }
        } else {
            print_type(_stack.alive, _queue.alive, _priority.alive);
            lines_left_of_case = line.parse::<usize>().unwrap() + 1;
            // plus one since we are removing one in same loop.

            _stack = stack::Stack::new();
            _queue = queue::Queue::new();
            _priority = priority::Priority::new();
        }
        line.clear();
        lines_left_of_case -= 1;
    }
    //one last for last one.
    print_type(_stack.alive, _queue.alive, _priority.alive);
}

fn print_type(s: bool, q: bool, p: bool) {
    // just a bunch of if statements
    let sum = s as u8 + q as u8 + p as u8;
    if sum == 0 {
        println!("impossible");
    } else if sum == 1 {
        if s {
            println!("stack");
        } else if q {
            println!("queue");
        }
        else {
            println!("priority queue");
        }
    }
    else {
        println!("not sure");
    }
}