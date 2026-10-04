# Practice — L5 (30/09, provvisoria) — Floyd, maggioranza, Misra-Gries

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
| E19 | Duplicato con Floyd (sola lettura, O(1)) | Θ(n) | puntatori come indici |
| E20 | Maggioranza (Boyer-Moore + verifica) | Θ(n), O(1) | `Option`, `then_some` |
| E21 | Maggioranza con insert/delete (contatori per bit) | O(log u) per op | `struct`, `impl` |
| E22 | Heavy hitters (Misra-Gries) | Θ(n) | `HashMap::retain` |

## Suggerimenti (solo se sei bloccato)

- **E19** Si parte da `n`. Dopo l'incontro, uno torna a `n` e si muovono insieme di un passo.
- **E20** Il contatore finale non è la frequenza: serve la seconda passata.
- **E22** La soglia è `n / t + 1` con divisione intera; la mappa non deve mai superare `t` chiavi alla fine di un passo.

> L5 è provvisoria (registrazione non ancora uscita): questi esercizi vengono dalle note 2025 del prof.
