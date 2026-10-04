# Practice — L3 (23/09) — SWM con deque, Next Larger, Trapping Rain Water, BST

Esercizi per riscrivere da zero, in Rust, gli algoritmi della lezione. Le tracce sono in `src/lib.rs`
(una funzione per esercizio, con `todo!()` al posto del codice); i test confrontano il tuo codice con gli
esempi della lezione e con una forza bruta su input casuali.

```
cargo test -p practice_l3            # tutta la lezione
cargo test -p practice_l3 e07        # un esercizio
```

All'inizio i test falliscono (il `todo!()` va in panic): è normale. Quando passano, prova a dire a voce
complessità e correttezza; poi confronta con `soluzioni/lib.rs` (fuori dal crate, non viene compilato;
soluzioni scritte da Claude, non dal prof).

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E07 | SWM con deque | Θ(n) | `VecDeque` |
| E08 | Next larger element | Θ(n) | stack con `Vec` |
| E09 | Trapping rain water (max prefissi/suffissi) | Θ(n), Θ(n) spazio | indici, vettori |
| E10 | Trapping rain water (due puntatori) | Θ(n), O(1) | due indici |
| E11 | È un BST? | Θ(n) | `Option<Box<Node>>`, ricorsione, tuple |

## Suggerimenti (solo se sei bloccato)

- **E07** Nella deque metti posizioni, non valori. Due `while`: uno sulla testa, uno sulla coda.
- **E08** Lo stack contiene le posizioni ancora senza risposta; il nuovo elemento risponde a tutti i più piccoli in cima.
- **E10** Se `max_sinistra <= max_destra`, l'acqua sulla posizione di sinistra è `max_sinistra - h[l]`.
- **E11** Per il sottoalbero vuoto restituisci `(true, i64::MAX, i64::MIN)`: così i confronti funzionano da soli.
