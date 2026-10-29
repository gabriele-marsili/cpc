# Practice — L11 — Fenwick tree (Sum/Add con i bit) e conteggio delle inversioni

Esercizi sulla lezione del 21/10 (rifatta sulla registrazione il 27/10). Soluzioni in `soluzioni/lib.rs`
(fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l11
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E47 | Navigazione del Fenwick tree: `lsb`, `parent`, `level`, nodi visitati da Sum e da Add | O(1) / Θ(log n) | `i & i.wrapping_neg()`, `trailing_zeros`, `count_ones` |
| E48 | Inversioni con la variante del mergesort | Θ(n log n) | ricorsione su `&mut [i64]`, `split_at_mut`, `copy_from_slice` |
| E49 | Inversioni contando gli scambi dell'insertion sort | Θ(n + #inv) | `slice::swap`, cicli con condizione composta |

E46 (L10) era il Fenwick tree completo: se non l'hai fatto, fallo prima di E47.
Il conteggio delle inversioni CON il Fenwick tree è in L12 (non fatto il 21/10).

## Suggerimenti (solo se sei bloccato)

- **E47** `-i` in complemento a 2 = bit invertiti + 1: ha lo stesso bit meno significativo di `i` e tutti gli altri
  sopra invertiti, quindi l'AND lascia solo quello. In Rust `usize` non ha il meno unario: `wrapping_neg()`.
  `sum_path`: togli lsb finché arrivi a 0. `add_path`: aggiungi lsb finché resti <= n (il bit si sposta
  almeno di una posizione a sinistra a ogni passo, per questo i passi sono <= log n).
- **E48** Quando nel merge prendi l'elemento da DESTRA, tutti quelli ancora da prendere a SINISTRA sono più grandi
  e stanno prima: aggiungi `l.len() - i`. Con `<=` nel confronto gli uguali non contano come inversione.
  Un solo buffer riusato per tutte le chiamate evita allocazioni.
- **E49** Ogni scambio di due vicini fuori ordine elimina esattamente un'inversione: il numero di scambi è la risposta.
