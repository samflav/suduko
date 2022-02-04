#![allow(warnings)]

use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

mod board;

fn main() {
    let args: Vec<String> = env::args().collect();

    //let puzz = get_puzzle(50, "p096_sudoku.txt");
    let puzz = get_puzzle(args[1].parse().unwrap(), "C:\\Users\\Samflav\\Desktop\\puzzles.txt");

    let mut brd = board::Board::new(puzz);

    //brd.trim(1);

    brd.backtrace(0);

    display_puzzle(&brd);
}

fn display_puzzle(brd: &board::Board) {
    //println!("+-+-+-+-+-+-+-+-+-+");

    for row in 0..9 {
        print!("|");
        for col in 0..9 {
            let val = &brd.board[(row * 9) + col];

            match val {
                board::Value::Set(num) => print!("{}|", num),
                board::Value::Unset(nums) => {
                    let spaces = 9 - nums.len();
                    for num in nums {
                        print!("{}", num);
                    }

                    for space in 0..spaces {
                        print!(" ");
                    }

                    print!("|");
                },
                _ => {}
            }
        }
        println!();
        //println!("+-+-+-+-+-+-+-+-+-+");
    }
}

fn get_puzzle(num: usize, path: &str) -> Vec<String> {
    let mut puzzle = vec![];

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let start = ((num - 1) * 10) + 1;

    for (index, line) in file.lines().map(|l| l.unwrap()).enumerate() {
        if index >= start {
            puzzle.push(line);
        }
        if index > start + 7 {
            break;
        }
    }

    puzzle
}

