# Practice — L7 (07/10) — Two pointers trick, binary search for the answer, sweep line

E28-E29 dalla versione provvisoria, E31-E34 dalla registrazione. Problema **obbligatorio** per l'esame: Longest k-Good Segment (E29).
**Frogs and Mosquitoes (ex E30) è stato spostato in `Lessons/L9/practice` (E39)**: non spiegato né in L7 né in L8.

```
cargo test -p practice_l7            # tutta la lezione
cargo test -p practice_l7 e29        # un esercizio
```

Soluzioni in `soluzioni/lib.rs` (fuori dal crate; scritte da Claude, non dal prof). Per E29 prova anche a
sottometterlo su Codeforces (link sotto): all'esame va consegnato in Rust.

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E28 | Coppia con somma data in array ordinato | Θ(n), O(1) | due indici |
| E29 | Longest k-good segment ([CF 616D](https://codeforces.com/contest/616/problem/D)) | Θ(n) | `HashMap`, finestra scorrevole |
| E31 | Longest k-good con binary search for the answer (solo la lunghezza) | Θ(n log n) | finestra di lunghezza fissa, `div_ceil` |
| E32 | Massimo numero di intervalli sovrapposti (sweep line) | Θ(n log n) | eventi `(coord, tipo)`, `sort_unstable` |
| E33 | Profondità massima di parentesi bilanciate | Θ(n) | `chars()`, `match` |
| E34 | Auto e carburante: minimo carburante iniziale G* | Θ(n log(cons·Σd)) | closure come predicato |

## Suggerimenti (solo se sei bloccato)

- **E28** Se `a[i] + a[j] < target`, nessuna coppia con `a[i]` funziona (a[j] è il più grande rimasto): `i += 1`.
- **E29** `r` avanza sempre; finché i distinti sono più di k, togli `a[l]` e avanza `l`. Ogni indice entra ed esce una volta.
- **E31** Se esiste un k-good di lunghezza L, ne esiste uno di ogni lunghezza minore: il predicato è monotono. Cerca l'ULTIMO sì.
- **E32** Eventi `(s, 0)` e `(e, 1)`: ordinando le tuple, a parità di coordinata gli inizi vengono prima (intervalli chiusi).
- **E34** Predicato "con G arrivo?": simula. Limite superiore sicuro: `cons * Σd`. Bonus Θ(n): il massimo deficit sui prefissi.
