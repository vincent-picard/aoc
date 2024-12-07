use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use array2d::Array2D;
use std::collections::HashSet;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage : day6 input.txt");
    }

    let path = Path::new(&args[1]);

    let mut file = File::open(path)
        .unwrap_or_else(|err| panic!("Unable to open file {} : {}", path.display(), err));

    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Unable read file to string");

    let lines:Vec<&str> = buf.trim().split("\n").collect();

    let n = lines.len();
    let m = lines[0].len();

    println!("Grid detected : {} rows x {} columns", n, m);

    let data:Vec<char> = buf.chars().filter(|c| !c.is_whitespace()).collect();

    let mat = Array2D::from_row_major(&data, n, m).expect("Unable to matrix data");

    let mut visited:HashSet<(usize,usize)> = HashSet::new();

    let (i, j) = locate_start(&mat).unwrap();
    println!("Starting point located at : {}, {}", i, j);
    visited.insert((i,j));

    GuardIterator::make(&mat).for_each(|(u, v)| {visited.insert((u,v)); ()});
    println!("Number of visited : {}", visited.len());

    let mut c = 0;
    for a in 0..n {
        for b in 0..m {
            if (a, b) != (i, j) && *mat.get(a,b).expect("get") != '#' && *mat.get(a,b).expect("get") != '^' {
                let mut newmat = mat.clone();
                newmat.set(a, b, '#').unwrap();
                let l = GuardIterator::make(&newmat).take(n*m + 10).count();
                if l > n*m {
                    c += 1;
                }
            }

        }
    }
    println!("Number of loops : {}", c);


}

fn locate_start(mat: &Array2D<char>) -> Option<(usize, usize)> {
    for (i, r) in mat.rows_iter().enumerate() {
        for (j, &x) in r.enumerate() {
            if x == '^' {
                return Option::Some((i, j));
            }
        }
    }
    return None;
}

#[derive(Clone, Copy)]
enum Direction {North, East, South, West}

fn next_case(i : usize, j :usize, n : usize, m : usize, d : Direction) -> Option<(usize, usize)> {
    match d {
        Direction::North if i > 0 => Some((i-1, j)),
        Direction::South if i < n-1 => Some((i+1, j)),
        Direction::West if j > 0 => Some((i, j-1)),
        Direction::East if j < m-1 => Some((i, j+1)),
        _ => None
    }
}

fn turn_right(d : Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

struct GuardIterator<'a> {
    grid: &'a Array2D<char>,
    i: usize,
    j: usize,
    d: Direction,
}

impl<'a> GuardIterator<'a> {

    fn make(mat: &'a Array2D<char>) -> GuardIterator<'a> {
        let (a, b) = locate_start(mat).expect("Unable to locate start point");
        GuardIterator {
            grid: mat, 
            i: a, 
            j: b, 
            d: Direction::North
        }
    }
}

impl<'a> Iterator for GuardIterator<'a> {
    type Item = (usize,usize);

    fn next(&mut self) -> Option<Self::Item> {
        match next_case(self.i, self.j, self.grid.num_rows(), self.grid.num_rows(), self.d) {
            Some((u, v)) => {
                if *(self.grid.get(u,v).expect("get")) == '#' {
                    self.d = turn_right(self.d);
                    self.next()
                } else {
                    self.i = u;
                    self.j = v;
                    Some((u,v))
                }
            },
            None => None
        }
    }
}

