# Practice — L6 (05/10, PROVVISORIO) — Maggioranza con cancellazioni, Misra-Gries, binary search

Argomenti annunciati alla fine di L5 per la lezione dopo, presi dalle note 2025 del prof: verranno integrati/ampliati
quando la lezione di L6 sarà sistemata sulla registrazione (gli esercizi qui non verranno toccati, se ne aggiungeranno altri).

```
cargo test -p practice_l6            # tutta la lezione
cargo test -p practice_l6 e22        # un esercizio
```

Soluzioni in `soluzioni/lib.rs` (fuori dal crate; scritte da Claude, non dal prof).

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E22 | Maggioranza con insert/delete (contatori per bit) | O(log u) per op | `struct`, `impl` |
| E23 | Heavy hitters (Misra-Gries) | Θ(n) | `HashMap::retain` |
| E24 | Prima occorrenza con ricerca binaria | Θ(log n) | `Ordering`, overflow di mid |
| E25 | Binary search for the answer (ultimo true) + radice quadrata intera | Θ(log) valutazioni | `Fn(u64) -> bool`, `checked_mul` |
| E26 | Social distancing | Θ((n+c) log ℓ) | closure, greedy |
| E27 | Exponential search | Θ(log t) | raddoppio + binaria |

## Suggerimenti (solo se sei bloccato)

- **E23** La soglia è `n / t + 1` con divisione intera; la mappa non deve mai superare `t` chiavi alla fine di un passo.
- **E24** Quando trovi `key` non fermarti: salva la posizione e continua a sinistra (`high = mid`).
- **E25** Se `pred(mid)` è vero la risposta è almeno `mid`: `low = mid + 1`, ricordando `mid`.
- **E26** Per un d fissato il greedy (piazza ogni persona nel primo punto valido) è ottimo. Il massimo d possibile è l'ampiezza totale, non il numero di punti.
- **E27** Raddoppia `hi` finché `pred(hi)` è vero, poi cerca in `[hi/2, hi)`.
