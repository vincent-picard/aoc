use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn increments(list:&[isize]) -> Vec<isize> {
    let n = list.len();
    let mut incr = Vec::new();
    for i in 1..n {
        incr.push(list[i] - list[i-1]);
    };
    incr
}


fn good_report(report:&[isize]) -> bool {
    let incr = increments(&report);
    let condition1 = incr.iter().all(|x| *x > 0) || incr.iter().all(|x| *x < 0);
    let condition2 = incr.iter().all(|x| 1 <= (*x).abs() && (*x).abs() <= 3); 
    condition1 && condition2
}

fn good_dampener(report:&[isize]) -> bool {
    if good_report(report) {return true;}
    let n = report.len();
    for i in 0..n {
        let damp:Vec<isize> = report.iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_,x)| *x)
            .collect();
        if good_report(&damp) {return true;}
    }
    return false;
}

fn main() {

    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage day2 input.txt");
    }

    let path = Path::new(&args[1]);
    let mut file = File::open(path)
        .unwrap_or_else(|err| panic!("Unable to open {} : {}", path.display(), err));
    
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .unwrap_or_else(|err| panic!("Unable to read file : {}", err));

    let lines:Vec<&str> = buffer.trim().split("\n").collect();
   
    let mut count = 0;
    let mut count2 = 0;

    for l in lines {
        let tokens:Vec<&str> = l.split_whitespace().collect();
        let mut report:Vec<isize> = Vec::new();
        for t in tokens {
            let n = t.parse()
                .unwrap_or_else(|err| panic!("Bad format : {}", err));
            report.push(n);
        }
       if good_report(&report) {count += 1;}
       if good_dampener(&report) {count2 += 1;}
    }

    println!("Nombre de rapports corrects : {}", count);
    println!("Nombre de rapports corrects avec amortisseurs: {}", count2);

}
