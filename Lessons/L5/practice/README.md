# Practice — L5 (30/09) — Duplicato con accesso casuale, Floyd, Boyer-Moore

Esercizi per riscrivere da zero, in Rust, gli algoritmi della lezione. Le tracce sono in `src/lib.rs`
(una funzione per esercizio, con `todo!()` al posto del codice); i test confrontano il tuo codice con gli
esempi della lezione e con una forza bruta su input casuali.

```
cargo test -p practice_l5            # tutta la lezione
cargo test -p practice_l5 e19        # un esercizio
```

All'inizio i test falliscono (il `todo!()` va in panic): è normale. Quando passano, prova a dire a voce
complessità e correttezza; poi confronta con `soluzioni/lib.rs` (fuori dal crate, non viene compilato;
soluzioni scritte da Claude, non dal prof).

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E19 | Duplicato: sposta ogni elemento nella sua cella partendo da n (soluzione del prof) | Θ(n), O(1) | `mem::replace` |
| E20 | Duplicato con Floyd (sola lettura, O(1)) | Θ(n) | puntatori come indici |
| E21 | Maggioranza (Boyer-Moore + verifica) | Θ(n), O(1) | `Option`, `then_some` |

## Suggerimenti (solo se sei bloccato)

- **E19** Tieni in mano un valore x: se `a[x] == x` è il duplicato, altrimenti scambialo con quello nella cella x.
- **E20** Si parte da `n`. Dopo l'incontro, uno torna a `n` e si muovono insieme di un passo.
- **E21** Il contatore finale non è la frequenza: serve la seconda passata.
