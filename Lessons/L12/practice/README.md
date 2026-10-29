# Practice — L12 — Inversioni con il Fenwick tree, remapping, Nested Segments

Esercizi sulla lezione del 26/10 (dalla registrazione). Soluzioni in `soluzioni/lib.rs`
(fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l12
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E50 | Inversioni con Fenwick tree sulle frequenze dei valori | Θ(max(A) + n log max(A)) | struct Fenwick privata, `wrapping_neg`, `u64` |
| E51 | Remapping (rango tra i valori distinti) + inversioni con valori fino a 1e18 | Θ(n log n) | `sort_unstable`, `dedup`, `binary_search` |
| E52 | Nested Segments ([CF 652D](https://codeforces.com/problemset/problem/652/D); obbligatorio nella riga del 21/10/2026 della pagina del corso) con la soluzione della lezione | Θ(n log n) | `flat_map`, `Reverse`, ordinare indici per chiave |

E52 è l'esercizio obbligatorio della riga del 21/10/2026 della pagina del corso (per il 26/10 la pagina non assegna nulla): prova a sottometterlo anche su Codeforces.

## Suggerimenti (solo se sei bloccato)

- **E50** F[v] = quante volte hai già visto il valore v. Per a[i] ti servono i valori > a[i] già visti: `range_sum(a[i] + 1, max)`
  (vuoto quando a[i] = max). Prima la query, poi l'Add, come alla lavagna: così F contiene solo elementi a sinistra di i.
- **E51** Ordina una copia, togli i doppioni, il rango è la posizione + 1. Gli id assegnati con una mappa hash e un contatore
  in ordine di apparizione NON preservano l'ordine: [1, 150, 1e6, 155] diventerebbe [1, 2, 3, 4] e perdi l'inversione 1e6 > 155.
- **E52** Con i segmenti in ordine di fine decrescente, quando arrivi a [l, r] i segmenti che finiscono dopo r hanno già tolto il
  loro 1: quelli che restano con inizio in [l, r] finiscono prima di r, quindi sono contenuti (più te stesso: -1).
