use std::cmp::max;
use std::env::args;
use std::process::exit;

fn main() {
    let mut args = args();
    let exe = args.next().unwrap();

    let Some(puzzle_id) = args.next() else {
        eprintln!("Usage: {exe} <puzzle ID> <max size>");
        exit(1);
    };
    let Ok(puzzle_id) = puzzle_id.parse::<isize>() else {
        eprintln!("Puzzle ID must be an integer");
        exit(1);
    };

    let Some(max_size) = args.next() else {
        eprintln!("Usage: {exe} <puzzle ID> <max size>");
        exit(1);
    };
    let Ok(max_size) = max_size.parse::<isize>() else {
        eprintln!("Max size must be an integer");
        exit(1);
    };

    let mut squares = Vec::new();
    for i in 0..max_size {
        squares.push(rand::random::<u8>() <= 50);
    }
    let squares = squares;
    // let squares: Vec<bool> = (0..max_size).map(|_| rand::random::<u8>() == 1).collect();
    // let squares = (0..max_size).map(|idx| idx == 0);

    let pool: Vec<usize> = squares.iter().enumerate()
        .filter(|(idx, hidden)| !**hidden)
        .map(|(idx, hidden)| idx + 1)
        .collect();

    fn factorExists(num: &usize, pool: &Vec<usize>) -> bool {
        for i in pool {
            if num != i && num % i == 0 {
                return true
            }
        }
        return false
    }
    fn removeFactors(num: &usize, pool: &Vec<usize>) -> Vec<usize> {
        let mut new = vec![];
        for i in pool {
            if num % i != 0 {
                new.push(*i);
            }
        }
        return new;
    }
    fn solve(pool: &Vec<usize>) -> Vec<Vec<usize>> {
        if pool.len() <= 1 { return vec![vec![]]; }

        let mut res: Vec<Vec<usize>> = vec![vec![]];
        for i in pool {
            if !factorExists(i, pool) {
                continue;
            }
            let newPool = removeFactors(i, pool);
            let results = solve(&newPool);

            for mut r in results {
                r.insert(0, *i);
                res.push(r);
            }
            // let results_: Vec<Vec<usize>> = results.iter().map(|mut x| x.insert(0, *i)).collect();
            // res.append(results_);
        }
        return res
    }

    let solutions = solve(&pool);
    let best: usize = solutions.iter().map(|sol| sol.iter().sum()).max().unwrap();

    print!(r#"{{"puzzleID":{puzzle_id},"squares":["#);
    print!("{}", squares.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
    println!(r#"],"best":{best}}}"#);
}
