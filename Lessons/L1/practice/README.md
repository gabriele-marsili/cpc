# Practice — L1 (16/09) — Introduzione

Esercizi per riscrivere da zero, in Rust, gli algoritmi della lezione. Le tracce sono in `src/lib.rs`
(una funzione per esercizio, con `todo!()` al posto del codice); i test confrontano il tuo codice con gli
esempi della lezione e con una forza bruta su input casuali.

```
cargo test -p practice_l1            # tutta la lezione
cargo test -p practice_l1 e01        # un esercizio
```

All'inizio i test falliscono (il `todo!()` va in panic): è normale. Quando passano, prova a dire a voce
complessità e correttezza; poi confronta con `soluzioni/lib.rs` (fuori dal crate, non viene compilato;
soluzioni scritte da Claude, non dal prof).

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E01 | Leaders in array | Θ(n) | iteratori, `rev` |
| E02 | Kadane (max subarray) | Θ(n), O(1) | fold/loop, casi limite |
| E03 | Missing number | Θ(n), O(1) | overflow, XOR |

## Suggerimenti (solo se sei bloccato)

- **E01** Scorri da destra tenendo il massimo visto finora; alla fine inverti.
- **E02** Tieni la miglior somma che *finisce* in i: o estendi la precedente o riparti da `a[i]`.
- **E03** `0 ^ 1 ^ ... ^ n` XOR tutti gli elementi = il mancante.
