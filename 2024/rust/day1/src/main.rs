use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage : day1 input_file");
    }

    let path = Path::new(&args[1]);
    let path_display = path.display();

    let mut file = File::open(&path)
        .unwrap_or_else(|err| panic!("Impossible d'ouvrir {} : {}", path_display, err));
    
    let mut s = String::new();
    file.read_to_string(&mut s)
        .unwrap_or_else(|err| panic!("Impossible de lire le fichier : {}", err));

    let lines:Vec<&str> = s.trim().split("\n").collect();
    
    let mut lista:Vec<isize> = Vec::new();
    let mut listb:Vec<isize> = Vec::new();

    for s in lines {
        let tokens:Vec<&str> = s.split_whitespace().collect();
        lista.push(tokens[0].parse().unwrap_or_else(|err| panic!("Parse error : {}", err)));
        listb.push(tokens[1].parse().unwrap_or_else(|err| panic!("Parse error : {}", err)));
    }
    lista.sort();
    listb.sort();

    let mut somme = 0;
    for i in 0..lista.len() {
       somme += (lista[i] - listb[i]).abs();
    }
    println!("Somme : {}", somme);

    let mut somme2 = 0;
    let mut j = 0;
    let n = lista.len();
    for i in 0..n {
        while (j < n) && (listb[j] < lista[i]) {
            j += 1;
        }
        while (j < n) && (listb[j] == lista[i]) {
            somme2 += lista[i];
            j += 1;
        }
    }
    println!("Somme 2 : {}", somme2);

}
