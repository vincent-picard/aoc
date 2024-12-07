use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: day7 input.txt");
    }

    let path = Path::new(&args[1]);

    let mut file = File::open(path)
        .unwrap_or_else(|err| panic!("Unable to open file {} : {}", path.display(), err));

    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Unable to read_to_string");

    let rows:Vec<&str> = buf.trim().split("\n").collect();

    let equations:Vec<Equation> = rows.iter().map(|s| Equation::new(s)).collect(); 

    let mut c = 0;
    let mut sum = 0;
    for e in equations {
        if e.has_solution() {
            c += 1;
            sum += e.0;
        }
    }
    println!("Number of solvable equations : {}", c);
    println!("Sum : {}", sum);
}

#[derive(Debug)]
struct Equation(usize, Vec<usize>);

impl Equation {
    fn new(s: &str) -> Self {
        let parts:Vec<&str> = s.split(":").collect();
        if parts.len() != 2 {
            panic!("equation : syntax error");
        }
        let res:usize = parts[0].trim().parse().unwrap();
        let mut numbers:Vec<usize> = parts[1].trim().split(" ").map(|s| s.parse().unwrap()).collect();
        numbers.reverse();
        Equation(res, numbers)
    }

    fn cut_plus(&self) -> Option<Equation> {
        let res = self.0;
        let numbers = &self.1;

        if numbers.len() > 0 && numbers[0] <= res {
            let e = Equation(res - numbers[0], numbers.iter().skip(1).map(|x| *x).collect());
            Some(e) 
        } else {
            None
        }
    }

    fn cut_mult(&self) -> Option<Equation> {
        let res = self.0;
        let numbers = &self.1;

        if numbers.len() > 0 && numbers[0] != 0 && res % numbers[0] == 0 {
            let e = Equation(res / numbers[0], numbers.iter().skip(1).map(|x| *x).collect());
            Some(e) 
        } else {
            None
        }
    }

    fn cut_concat(&self) -> Option<Equation> {
        let res = self.0;
        let numbers = &self.1;

        if numbers.len() == 0 || numbers[0] == 0 {return None;}


        let reste = cut(res, numbers[0]);
        match reste {
            None => None,
            Some(r) => {
                let e = Equation(r, numbers.iter().skip(1).map(|x| *x).collect());
                Some(e) 
            }  
        }
    }

    fn is_equality(&self) -> bool {
        let res = self.0;
        let numbers = &self.1;
        numbers.len() == 1 && res == numbers[0]
    }

    fn has_solution(&self) -> bool {
        self.is_equality() ||
            match self.cut_plus() {
                None => false,
                Some(e) => e.has_solution(),
            } ||
            match self.cut_mult() {
                None => false,
                Some(e) => e.has_solution(),
            } ||
            match self.cut_concat() {
                None => false,
                Some(e) => e.has_solution(),
            }
    }
}


// Example : cut(123456, 456) = Some(123)

fn cut(a: usize, b: usize) -> Option<usize> {
    let mut x = a;
    let mut y = b;
    while y > 0 {
        if x % 10 != y % 10 {
            return None;
        }
        x = x / 10;
        y = y / 10;
    };
    Some(x)
}

#[test]
fn test_cut() {
    assert_eq!(cut(123456, 456), Some(123));
    assert_eq!(cut(123656, 456), None);
}

