# Practice — L9 — Frogs and Mosquitoes e prefix sums statiche

E39 = Frogs and Mosquitoes, problema **obbligatorio** per l'esame (ex E30 di L7), spiegato nella lezione del 14/10.
E40-E42 (aggiunti il 19/10) sulla seconda parte della lezione: prefix sums statiche, Ilya and Queries, e il caso
semplificato di Frogs (point location statica). Soluzioni in `soluzioni/lib.rs` (fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l9
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E39 | Frogs and Mosquitoes ([CF 609F](https://codeforces.com/contest/609/problem/F)) | Θ((n+m) log(n+m)) | `BTreeMap::range`, predecessore |
| E40 | Prefix sums statiche, `Sum(i)` e `RangeSum(i, j)` 1-based | Θ(n) costruzione, O(1) query | `iter().scan`, `Vec::with_capacity`, struct + impl |
| E41 | Ilya and Queries ([CF 313B](https://codeforces.com/problemset/problem/313/B)) | Θ(n + m) | `as_bytes().windows(2)`, indici off-by-one |
| E42 | 1D point location statica (intervalli disgiunti) | O(log n) per punto | `slice::partition_point` |

Prova anche a sottometterlo su Codeforces: all'esame va consegnato in Rust.

## Suggerimenti (solo se sei bloccato)

- **E39** Ordina le rane per x e dai a ognuna l'intervallo `[max(x, fine precedente + 1), x + t]`: così il predecessore di b
  nel BST è l'unica candidata. Quando una lingua cresce, togli dal BST le rane successive coperte del tutto e sposta
  l'inizio di quella coperta in parte; poi fai mangiare alla rana le zanzare in attesa nel suo intervallo.
- **E40** Metti uno 0 all'inizio del vettore delle somme: `p[i] = Sum(i)` e `RangeSum(i, j) = p[j] - p[i-1]` senza `if`.
- **E41** Vettore binario B[k] = 1 se s[k] == s[k+1] (lungo n - 1), poi prefix sums su B: la risposta è il numero di 1
  in B[l..r-1], cioè una range sum. Attenzione a non andare oltre: l'ultimo k utile è r - 1.
- **E42** `partition_point(|&(s, _)| s <= p)` restituisce quanti intervalli iniziano a sinistra di p (o in p):
  il predecessore è l'ultimo di questi; contiene p solo se la sua fine è >= p.
