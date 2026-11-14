// Hands-On 2 — CPC 2026/27
// Statement: https://pages.di.unipi.it/rossano/blog/2024/handson2/

// Usage: cargo run -p handson2 -- 1 < testsets/p1/input0.txt   (Problem 1: Min and Max)
//        cargo run -p handson2 -- 2 < testsets/p2/input0.txt   (Problem 2: Is There)
// The package must be named `handson2` (Cargo.toml): this file imports from lib.rs through that name.

use handson2::{parse_p1, parse_p2, solve_p1, solve_p2, tokenize_line};
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::{env, fs, io, process};

fn help() {
    eprintln!(
        "usage: cargo run -p handson2 -- <problem> [input]

  <problem>   1 = Min and Max, 2 = Is There

  input, one of:
    (nothing)        read from stdin: `-- 1 < testsets/p1/input0.txt`,
                     or type it in the terminal (stops after the last query)
    <file.txt>       read from a file: `-- 2 testsets/p2/input0.txt`
    <tokens...>      the whole input as arguments, newlines not needed:
                     `-- 1 5 3  5 1 4 3 2  0 1 2 2  1 2 4  1 1 2`

  Format (text after `//` on a line is ignored):
    problem 1: n m / A[1..n] / m queries `0 i j T` (update) or `1 i j` (max)
    problem 2: n m / n segments `l r` / m queries `i j k`"
    );
}

fn fail(msg: &str) -> ! {
    eprintln!("error: {msg}\n");
    help();
    process::exit(2);
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some((problem, rest)) = args.split_first() else {
        fail("missing problem number");
    };
    if problem == "-h" || problem == "--help" {
        help();
        return;
    }

    // Every source becomes the same stream of tokens.
    let mut tokens: Box<dyn Iterator<Item = String>> = match rest {
        [] => Box::new(
            io::stdin()
                .lock()
                .lines()
                .map_while(Result::ok)
                .flat_map(|l| tokenize_line(&l).collect::<Vec<_>>()),
        ),
        [path] if Path::new(path).is_file() => {
            let text = fs::read_to_string(path)
                .unwrap_or_else(|e| fail(&format!("cannot read {path}: {e}")));
            let toks: Vec<String> = text.lines().flat_map(tokenize_line).collect();
            Box::new(toks.into_iter())
        }
        [path] if path.ends_with(".txt") => fail(&format!("file not found: {path}")),
        toks => Box::new(toks.iter().cloned()),
    };

    match problem.as_str() {
        "1" => {
            let input = parse_p1(&mut tokens).unwrap_or_else(|e| fail(&e));
            print_lines(solve_p1(&input));
        }
        "2" => {
            let input = parse_p2(&mut tokens).unwrap_or_else(|e| fail(&e));
            print_lines(solve_p2(&input));
        }
        other => fail(&format!("problem must be 1 or 2, got {other:?}")),
    }
}

/// One answer per line, buffered (one write per line would be slow on large outputs).
fn print_lines<T: std::fmt::Display>(answers: Vec<T>) {
    let mut out = BufWriter::new(io::stdout().lock());
    for a in answers {
        writeln!(out, "{a}").expect("cannot write to stdout");
    }
}
