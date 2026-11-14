# Hands-On 2 — cosa c'è qui (GENERATO DA CLAUDE)

Questa cartella contiene solo materiale generato da Claude. Non fa parte della consegna.
Spiegazione del codice delle soluzioni (P1 e P2): `SPIEGAZIONE_CODICE.md`.
Il tuo codice va in `../src/` (`main.rs`, `lib.rs`), il report in `../report/`.

Consegna: https://pages.di.unipi.it/rossano/blog/2024/handson2/ — Segment Tree, in Rust.

## Problema 1 — Min and Max
Array A di n interi positivi (≤ n), m query:
- `0 i j T` → Update(i, j, T): A[k] = min(A[k], T) per i ≤ k ≤ j
- `1 i j`   → Max(i, j): stampa max A[i..j]

Input: `n m`, poi gli n valori, poi m query. Obiettivo: O((n+m) log n).
Esempio: n=5, A = 5 1 4 3 2; `0 1 2 2`, `1 2 4`, `1 1 2` → output `4`, `2`.

## Problema 2 — Is There
n segmenti ⟨l, r⟩ con 0 ≤ l ≤ r ≤ n−1, m query IsThere(i, j, k): 1 se esiste p in [i, j] coperto da
esattamente k segmenti, altrimenti 0. Input: `n m`, n righe `l r`, m righe `i j k`. Obiettivo: O((n+m) log n).
Esempio: segmenti (0,4) (1,3) (1,2) (1,1) (0,0); query (0,4,4) (0,4,0) (1,3,1) (1,4,1) → 1 0 0 1.

## ⚠️ Indici
- **Problema 1: gli indici delle query partono da 1** (sia nell'esempio della consegna sia nel test set).
- **Problema 2: partono da 0** (segmenti e query).
Verificato con una brute-force sul test set (11/11 e 8/8 passati), poi buttata: non è in questa cartella.

## Test set
`../testsets/p1` (input0–10) e `../testsets/p2` (input0–7), scaricati dalla pagina del corso
(gli zip originali sono accanto).

## Come lanciare i test
Il runner `testset_GENERATO.rs` (agganciato in `../Cargo.toml` come `[[test]] testset`) lancia il tuo binario
su ogni `inputK.txt` e confronta l'output con `outputK.txt` (token per token).
Contratto che assume per `main.rs`: **primo argomento = numero del problema (1 o 2)**, input da stdin,
una risposta per riga su stdout. Se preferisci un'altra convenzione, cambi solo `.arg(...)` nel runner.

    cargo run  -p handson2 -- 1 < HandsOn/HO2/testsets/p1/input0.txt
    cargo test -p handson2                       # tutto (anche i tuoi #[test] in lib.rs)
    cargo test -p handson2 --test testset p1     # solo test set Problema 1
    cargo fmt -p handson2 ; cargo clippy -p handson2

In caso di errore dice per ogni file quante risposte attese/prodotte e la prima differenza.

## Da consegnare (per email a rossano.venturini@gmail.com)
`main.rs`, `lib.rs`, `Handson_2_solution_YOUR_NAME.pdf` (descrizione delle soluzioni, complessità in tempo e
spazio, fonti). Prima: test tutti verdi, `cargo fmt`, `cargo clippy`. Nessuna scadenza indicata sulla pagina.
