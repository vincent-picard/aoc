use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::iter;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage : day3 input.txt");
    }
    let path = Path::new(&args[1]);
    let mut file = File::open(path)
        .unwrap_or_else(|err| panic!("Unable to open file {} : {}", path.display(), err));

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .unwrap_or_else(|err| panic!("Unable to read file : {}", err));
   
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")
        .unwrap_or_else(|err| panic!("Regexp error : {}", err));

    let mut buffer2 = String::new();
    let mut enabled = true;
    for c in buffer.chars() {
        if c == 'A' {
            enabled = true;
        } else if c == 'B' {
            enabled = false;
        } else if enabled {
            buffer2.push(c);
        }
    }

    let values:Vec<(isize, isize)> = re.captures_iter(&buffer2)
        .map(|caps| {
            let (_, [a, b]) = caps.extract();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();

    for (i, (a,b)) in values.iter().enumerate() {
        println!("{} : {}, {}", i, a , b);
    }

    let s:isize = values.iter().map(|(a, b)| a * b).sum();
    println!("Somme : {}", s);

    
}


