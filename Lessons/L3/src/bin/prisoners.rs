// cargo run -p l3 --release --bin prisoners
// Simula il puzzle dei 100 prigionieri: strategia casuale vs "segui il ciclo".
use l3::extra::{all_succeed, exact_success_probability, Rng};

fn main() {
    let n = 100;
    let trials = 100_000;
    let mut rng = Rng(2026);

    let mut ok = 0;
    for _ in 0..trials {
        if all_succeed(&rng.permutation(n)) {
            ok += 1;
        }
    }
    println!("segui il ciclo: {ok}/{trials} = {:.4}", ok as f64 / trials as f64);
    println!("valore esatto:  1 - sum_(l=51..100) 1/l = {:.4}", exact_success_probability(n));
    println!("limite n->inf:  1 - ln 2 = {:.4}", 1.0 - 2f64.ln());
    println!("strategia casuale: (1/2)^100 = {:.2e}", 0.5f64.powi(100));
}
