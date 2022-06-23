extern crate image;
extern crate itertools;
extern crate rand;

use std::env;

mod amg;

fn main() {
    parse_args();
}

fn parse_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        print_help(&args[0]);
    } else {
        let width = match args[1].parse::<usize>() {
            Ok(x) => x,
            Err(_) => {
                print_help(&args[0]);
                return;
            }
        };
        let height = match args[2].parse::<usize>() {
            Ok(x) => x,
            Err(_) => {
                print_help(&args[0]);
                return;
            }
        };
        let mut builder = amg::Maze::builder(width, height);
        args.iter().skip(3).for_each(|x| {
            if x.chars().count() < 2 {
                return;
            }
            let mut iter = x.chars().peekable();
            if iter.next().unwrap() != '-' {
                builder.filename(x.clone());
                return;
            }
            let next = *iter.peek().unwrap();
            if next == '-' {
                if x == "--help" {
                    print_help(&args[0]);
                } else {
                    builder.parse_word(&x[2..]);
                }
            } else {
                iter.for_each(|x| {
                    if x == 'h' {
                        print_help(&args[0]);
                    } else {
                        builder.parse_letter(x);
                    }
                })
            }
        });
        builder.build();
    }
}

fn print_help(bin: &String) {
    println!();
    println!("A Maze Generator");
    println!();
    println!("Usage:");
    println!("  {} width height [options] [filename]", bin);
    println!("  {} -h | --help", bin);
    println!();
    println!("Options:");
    println!("  -h --help       Show this screen");
    println!("  -s --no-struct  Don't generate structures");
    println!("  -e --no-exit    Don't add an entrance and an exit to the maze");
    println!("  -l --no-loops   Disallow loops");
    println!("  -t --no-stubs   Don't remove stubs");
    println!("  -i --image      Save the maze as an image");
    println!("  -o --solve      Save the maze with solved paths as an image");
    println!();
    println!("Parameters:");
    println!("  filename        filename for storing images (without ending, default: 'maze')");
    println!();
}
