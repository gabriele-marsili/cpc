# Practice — L4 (28/09) — 100 prigionieri, elemento duplicato

Esercizi per riscrivere da zero, in Rust, gli algoritmi della lezione. Le tracce sono in `src/lib.rs`
(una funzione per esercizio, con `todo!()` al posto del codice); i test confrontano il tuo codice con gli
esempi della lezione e con una forza bruta su input casuali.

```
cargo test -p practice_l4            # tutta la lezione
cargo test -p practice_l4 e12        # un esercizio
```

All'inizio i test falliscono (il `todo!()` va in panic): è normale. Quando passano, prova a dire a voce
complessità e correttezza; poi confronta con `soluzioni/lib.rs` (fuori dal crate, non viene compilato;
soluzioni scritte da Claude, non dal prof).

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E12 | 100 prigionieri: vincono tutti? | Θ(n²) va bene | `all`, closure |
| E13 | Ciclo più lungo di una permutazione | Θ(n) | visitati |
| E14 | Probabilità esatta + simulazione | — | `f64`, `sum` |
| E15 | Duplicato con hash set | Θ(n) atteso | `HashSet` |
| E16 | Duplicato con accesso diretto | Θ(n) | `Vec<bool>`, `mem::replace` |
| E17 | Duplicato bit per bit (sola lettura, O(1)) | Θ(n log n) | ricerca binaria sui valori |
| E18 | Duplicato distruggendo A (compito di L4) | Θ(n), O(1) | `&mut [usize]`, `swap` |

## Suggerimenti (solo se sei bloccato)

- **E17** Mantieni un intervallo di valori `[lo, hi)` che contiene sicuramente un duplicato e dimezzalo contando.
- **E18** Due strade: scambiare finché `a[i] == i` (la tua `my_sol`), oppure seguire `i -> a[i]` da `n` marcando le celle.
