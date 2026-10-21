# Practice — L10 — Little Girl, Number of Ways, Fenwick tree

Esercizi sulla lezione del 19/10 (rifatta sulla registrazione il 21/10). Soluzioni in `soluzioni/lib.rs`
(fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l10
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E43 | Little Girl and Maximum Sum ([CF 276C](https://codeforces.com/problemset/problem/276/C)) | Θ(n log n + m) | `iter().scan`, `sort_unstable_by`, `zip` |
| E44 | Frequenze delle posizioni con la sweep line (la prima soluzione vista a lezione) | Θ(n + m log m) | vettore di eventi `(usize, i64)`, ordinamento di tuple |
| E45 | Number of Ways ([CF 466C](https://codeforces.com/problemset/problem/466/C)) | Θ(n) | cicli all'indietro, `u64::from(bool)`, casi n < 3 |
| E46 | Fenwick tree da zero (1-based, come alla lavagna) | O(log n) per `add` e `sum` | `i & i.wrapping_neg()`, struct + impl |

Prova anche a sottometterli su Codeforces (E43, E45): all'esame va consegnato in Rust.

## Suggerimenti (solo se sei bloccato)

- **E43** Array B lungo n + 1: `B[i-1] += 1`, `B[j] -= 1` (con la cella in più non serve l'`if` per j = n).
  Le prefix sums di B sono le frequenze. Poi ordina valori e frequenze nello stesso verso e somma i prodotti.
  Attenzione all'overflow su Codeforces: usa i64.
- **E44** Evento `(i, +1)` per l'inizio e `(j + 1, -1)` subito dopo la fine. Ordinati, scorri p = 1..=n applicando
  tutti gli eventi in posizione p prima di scrivere F[p-1]. Bonus: rifallo con counting sort (gli estremi sono <= n + 1)
  e accorgiti che ottieni l'array B di E43.
- **E45** Se S non è divisibile per 3 la risposta è 0. `c[k]` = quante suffix sum uguali a T iniziano in k o dopo
  (scorri da destra). Poi da sinistra: ogni volta che la prefix sum fino a i vale T aggiungi `c[i + 2]`
  (la parte centrale deve avere almeno un elemento). Non moltiplicare #blu × #rossi: conteresti coppie nell'ordine sbagliato.
- **E46** `t` lungo n + 1, la cella 0 è la radice finta. `sum`: finché i > 0, somma `t[i]` e togli il bit meno
  significativo (`i -= lsb(i)`, sali al padre). `add`: finché i <= n, aggiorna `t[i]` e `i += lsb(i)`.
  Controlla con l'esempio della lezione: i nodi devono essere [3, 5, -1, 9, 7, 4, 2, 16].
