// cargo run -p l3 --bin swm
// Confronta le 4 soluzioni della SWM sull'esempio e le cronometra su un input grande.
use l3::extra::Rng;
use l3::swm::{brute_force, bst, heap, linear};
use std::time::Instant;

fn main() {
    let v = vec![1, 2, 3, 1, 4, 5, 2, 3, 1];
    let k = 3;
    println!("A = {v:?}, k = {k}");
    println!("brute_force: {:?}", brute_force(&v, k));
    println!("bst:         {:?}", bst(&v, k));
    println!("heap:        {:?}", heap(&v, k));
    println!("linear:      {:?}", linear(&v, k));

    // Tempi indicativi: compilare con --release per numeri sensati.
    let mut rng = Rng(7);
    let n = 1_000_000;
    let big: Vec<i32> = (0..n).map(|_| rng.next_u64() as i32).collect();
    for k in [10, 1_000] {
        println!("\nn = {n}, k = {k}");
        let fs: [(&str, fn(&Vec<i32>, usize) -> Vec<i32>); 4] =
            [("brute_force", brute_force), ("bst", bst), ("heap", heap), ("linear", linear)];
        for (name, f) in fs {
            let t = Instant::now();
            let r = f(&big, k);
            println!("  {name:<12} {:>8.1?}  (len {})", t.elapsed(), r.len());
        }
    }
}
