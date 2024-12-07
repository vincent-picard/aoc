use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use std::iter;
use std::collections::HashSet;

fn main() {

    let args:Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Usage : day5 rules.txt orders.txt");
    }

    /* Parsing rules */

    let path_rules = Path::new(&args[1]);

    let mut file_rules = File::open(path_rules)
        .unwrap_or_else(|err| panic!("Unable to open file {} : {}", path_rules.display(), err));

    let mut buffer_rules = String::new();
    file_rules.read_to_string(&mut buffer_rules)
        .unwrap_or_else(|err| panic!("Unable to read file {} : {}", path_rules.display(), err));

    let vrules:Vec<&str> = buffer_rules.trim().split("\n").collect();

    let mut drules:Vec<Vec<usize>> = iter::repeat_with(|| Vec::new()).take(100).collect();
    
    for r in &vrules {
        let tokens:Vec<usize> = (*r).split("|").map(|lit| lit.parse().unwrap()).collect();
        assert_eq!(tokens.len(), 2);
        drules[tokens[1]].push(tokens[0]);
    }

    println!("{} rules parsed", vrules.len());


    /* Parsing orders */
    let path_orders = Path::new(&args[2]);

    let mut file_orders = File::open(path_orders)
        .unwrap_or_else(|err| panic!("Unable to open file {} : {}", path_orders.display(), err));

    let mut buffer_orders = String::new();
    file_orders.read_to_string(&mut buffer_orders)
        .unwrap_or_else(|err| panic!("Unable to read file {} : {}", path_orders.display(), err));

    let vorders:Vec<&str> = buffer_orders.trim().split("\n").collect();

    let mut dorders:Vec<Vec<usize>> = Vec::new();
    for s in &vorders {
        let vals:Vec<usize> = (*s).split(",").map(|lit| lit.parse().unwrap()).collect();
        dorders.push(vals);
    }
    println!("{} print order lists parsed", dorders.len());

    let mut count = 0;
    let mut count_err = 0;
    let mut sum = 0;
    let mut sum_err = 0;

    for l in &dorders {
        if check_order(&l, &drules) {
            count += 1;
            sum += middle_val(&l);
        } else {
            println!("Incorrect order : {:?}", l);
            count_err += 1;
            let correct = sort_order(&l, &drules);
            println!("Corrected order : {:?}", correct);
            sum_err += middle_val(&correct);
        }
    }
    println!("Correct orders : {}", count);
    println!("Middle val sum : {}", sum);
    println!("Incorrect orders : {}", count_err);
    println!("Middle val sum corrected : {}", sum_err);

}

fn check_order(l:&Vec<usize>, rules:&Vec<Vec<usize>>) -> bool {
    let mut seen:HashSet<usize> = HashSet::new();
    for x in l {
        for y in &(rules[*x]) {
            if !seen.contains(&y) && l.contains(&y) {
                return false;
            }
        }
        seen.insert(*x);
    }
    true
}

fn sort_order(l:&Vec<usize>, rules:&Vec<Vec<usize>>) -> Vec<usize> {
    let mut res:Vec<usize> = Vec::new();
    let mut seen:Vec<usize> = Vec::new();
    fn dfs(x:usize, l:&Vec<usize>, seen:&mut Vec<usize>, rules:&Vec<Vec<usize>>, res:&mut Vec<usize>) {
        if !seen.contains(&x) {
            seen.push(x);
            for y in &(rules[x]) {
                if l.contains(&y) {
                    dfs(*y,l,seen,rules,res);
                }
            }
            res.push(x)
        }
    }
    for x in l {
        dfs(*x, l, &mut seen, &rules, &mut res);
    }
    res
}

fn middle_val(l:&Vec<usize>) -> usize {
    let n = l.len();
    l[n/2]
}

