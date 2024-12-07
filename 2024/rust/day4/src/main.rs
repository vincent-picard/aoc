use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use array2d::Array2D;

fn is_xmas(mat:&Array2D<char>, i:isize, j:isize, u:isize, v:isize) -> bool {
    let n = mat.num_rows() as isize;
    let m = mat.num_columns() as isize;
    let pat:Vec<char> = "XMAS".chars().collect();
    let mut k = 0;
    let mut a = i;
    let mut b = j;
    while k < pat.len() && 0 <= a && 0 <= b && a < n && b < m {
        if mat[(a as usize,b as usize)] == pat[k] {
            k += 1;
            a += u;
            b += v;
        } else {
            break;
        }
    };
    k == pat.len()
}

fn is_cross(mat:&Array2D<char>, i:usize, j:usize) -> bool {
    let c1 = mat[(i, j)] == 'A';
    let c2 =
        (mat[(i-1,j-1)] == 'M' && mat[(i+1, j+1)] == 'S')
        || (mat[(i-1,j-1)] == 'S' && mat[(i+1, j+1)] == 'M');
    let c3 =
        (mat[(i-1,j+1)] == 'M' && mat[(i+1, j-1)] == 'S')
        || (mat[(i-1,j+1)] == 'S' && mat[(i+1, j-1)] == 'M');
    c1 && c2 && c3
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let lines:Vec<&str> = buffer.trim().split("\n").collect(); 
    let m:Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect(); 
    let mat = Array2D::from_rows(&m).unwrap();

    let n = mat.num_rows();
    let m = mat.num_columns();
   
    let mut count = 0;

    for i in 0..n {
        for j in 0..m {
            if is_xmas(&mat, i as isize, j as isize, 0, 1) { count += 1;}
            if is_xmas(&mat, i as isize, j as isize, 0, -1) { count += 1;}
            if is_xmas(&mat, i as isize, j as isize, -1, 0) { count += 1;}
            if is_xmas(&mat, i as isize, j as isize, 1, 0) { count += 1;}
            if is_xmas(&mat, i as isize, j as isize, 1, 1) { count += 1;}
            if is_xmas(&mat, i as isize, j as isize, 1, -1) { count += 1;}
            if is_xmas(&mat, i as isize, j as isize, -1, 1) { count += 1;}
            if is_xmas(&mat, i as isize, j as isize, -1, -1) { count += 1;}
        }
    }

    let mut count2 = 0;
    for i in 1..n-1 {
        for j in 1..m-1 {
            if is_cross(&mat, i, j) { count2 += 1;}
        }
    }

    println!("Occurrence : {}", count);
    println!("Occurrence cross : {}", count2);
}
