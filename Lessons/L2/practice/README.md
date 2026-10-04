# Practice — L2 (21/09) — Sliding Window Maxima: banale, heap, BST

Esercizi per riscrivere da zero, in Rust, gli algoritmi della lezione. Le tracce sono in `src/lib.rs`
(una funzione per esercizio, con `todo!()` al posto del codice); i test confrontano il tuo codice con gli
esempi della lezione e con una forza bruta su input casuali.

```
cargo test -p practice_l2            # tutta la lezione
cargo test -p practice_l2 e04        # un esercizio
```

All'inizio i test falliscono (il `todo!()` va in panic): è normale. Quando passano, prova a dire a voce
complessità e correttezza; poi confronta con `soluzioni/lib.rs` (fuori dal crate, non viene compilato;
soluzioni scritte da Claude, non dal prof).

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E04 | SWM banale | Θ(nk) | `windows` |
| E05 | SWM heap + lazy delete | Θ(n log n) | `BinaryHeap`, `while let` |
| E06 | SWM con BST (multiset) | Θ(n log k) | `BTreeMap`, `entry` |

## Suggerimenti (solo se sei bloccato)

- **E05** Push di `(valore, posizione)`; la tupla si confronta prima sul valore.
- **E06** Quando il contatore di un valore arriva a 0, va rimosso dalla mappa, altrimenti `last_key_value` sbaglia.
