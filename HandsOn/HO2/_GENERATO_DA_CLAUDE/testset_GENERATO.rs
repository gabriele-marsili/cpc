// GENERATO DA CLAUDE — runner del test set ufficiale. Non fa parte della consegna.
//
// Per ogni testsets/pX/inputK.txt lancia il binario `handson2 X` con il file su stdin
// e confronta l'output (token per token, quindi spazi/newline finali non contano)
// con testsets/pX/outputK.txt.
//
// Contratto con src/main.rs: il primo argomento è il numero del problema (1 o 2),
// l'input arriva da stdin, una risposta per riga su stdout.
//
//   cargo test -p handson2                       # entrambi i problemi
//   cargo test -p handson2 --test testset p1     # solo Problema 1
//   cargo test -p handson2 --test testset p2     # solo Problema 2

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn run_testset(problem: u32) {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("testsets/p{problem}"));
    let mut inputs: Vec<(u32, PathBuf)> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
        .filter_map(|e| {
            let p = e.ok()?.path();
            let name = p.file_name()?.to_str()?;
            let k = name
                .strip_prefix("input")?
                .strip_suffix(".txt")?
                .parse()
                .ok()?;
            Some((k, p))
        })
        .collect();
    inputs.sort();
    assert!(!inputs.is_empty(), "no input files in {}", dir.display());

    let mut failed = Vec::new();
    for (k, input_path) in &inputs {
        let input = fs::read(input_path).unwrap();
        let expected = fs::read_to_string(dir.join(format!("output{k}.txt"))).unwrap();

        let mut child = Command::new(env!("CARGO_BIN_EXE_handson2"))
            .arg(problem.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("cannot start the handson2 binary");
        child.stdin.take().unwrap().write_all(&input).unwrap();
        let out = child.wait_with_output().unwrap();
        let got = String::from_utf8_lossy(&out.stdout);

        let exp_tok: Vec<&str> = expected.split_whitespace().collect();
        let got_tok: Vec<&str> = got.split_whitespace().collect();
        if !out.status.success() {
            failed.push(format!(
                "input{k}: process exited with {} — stderr: {}",
                out.status,
                String::from_utf8_lossy(&out.stderr).trim()
            ));
        } else if exp_tok != got_tok {
            let first_diff = exp_tok
                .iter()
                .zip(&got_tok)
                .position(|(a, b)| a != b)
                .unwrap_or(exp_tok.len().min(got_tok.len()));
            failed.push(format!(
                "input{k}: {} answers expected, {} produced; first difference at answer #{first_diff} (expected {:?}, got {:?})",
                exp_tok.len(),
                got_tok.len(),
                exp_tok.get(first_diff),
                got_tok.get(first_diff)
            ));
        }
    }
    println!(
        "problem {problem}: {}/{} passed",
        inputs.len() - failed.len(),
        inputs.len()
    );
    assert!(failed.is_empty(), "\n{}\n", failed.join("\n"));
}

#[test]
fn p1_min_and_max() {
    run_testset(1);
}

#[test]
fn p2_is_there() {
    run_testset(2);
}
