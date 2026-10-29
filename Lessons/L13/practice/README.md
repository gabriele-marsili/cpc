# Practice — L13 — Update the Array, range update con due Fenwick tree, segment tree

Esercizi sulla lezione del 28/10 (dalla registrazione). Soluzioni in `soluzioni/lib.rs`
(fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l13
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E53 | Update the Array ([SPOJ UPDATEIT](http://www.spoj.com/problems/UPDATEIT/); obbligatorio nella riga del 19/10/2026 della pagina del corso) | Θ(log n) per update e access | struct con Fenwick interno, `+v` / `-v` |
| E54 | Prefix sums con range update: `Sum(i) = i·FT1.Access(i) − Err.Access(i)` | Θ(log n) per operazione | composizione di struct, cast `as i64` |
| E55 | Segment tree per somme: build bottom-up, add, range sum top-down | Θ(n) build, Θ(log n) add | ricorsione su nodi `2k`, `2k+1`, `Vec` di 4n |

Per il 28/10 la pagina del corso non ha (ancora) una riga: nessun esercizio nuovo assegnato.
Compito a voce del prof (vedi L13.pdf): "il range sum top-down è lineare, o meglio?" — ragionaci prima di guardare il codice.

## Suggerimenti (solo se sei bloccato)

- **E53** `update(i, j, v)`: `+v` in i e `-v` in j + 1 (solo se j < n). `access(i)` = prefix sum fino a i.
  È il trucco di Little Girl (L10) con il Fenwick tree al posto della prefix sum statica.
- **E54** Per ogni range update fai tre cose: FT1 riceve (i, j, v); Err riceve (i, j, v·(i−1)) — dentro il range
  `i·v` conta i−1 celle che non c'erano —; Err riceve (j+1, n, −v·(j−i+1)) — dopo il range `i·0` dimentica tutto il range.
  Controlla con la lavagna: dopo RangeUpdate(2,7,10), Sum(6) = 60 − 10 = 50 e Sum(8) = 0 − (−60) = 60.
- **E55** `build(k, lo, hi)`: se foglia copia il valore, altrimenti costruisci i due figli e somma (post-ordine = bottom-up).
  In `range_sum` NON calcolare a mano in quale figlio scendere: scendi in entrambi e lascia che i casi
  "disgiunto" e "contenuto" fermino la ricorsione (come consigliato a lezione).
