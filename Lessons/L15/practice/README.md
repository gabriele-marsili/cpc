# Practice — L15 — segment tree (esercizi delle note del prof) e Mo's algorithm

Tracce degli esercizi elencati alla fine di <https://pages.di.unipi.it/rossano/blog/2026/segmenttree/>,
con i controlli numerici scritti dal prof come test. Soluzioni in `soluzioni/lib.rs`
(fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l15
```

| # | Esercizio (note del prof) | Obiettivo | Rust che alleni |
|---|---|---|---|
| E59 | Segment tree **generico** (tipo, combine, neutro) usato per somme, RMQ e minimo con occorrenze | Θ(log n) combine per operazione | generics `T: Clone`, `F: Fn(T, T) -> T`, closure come campo |
| E60 | `search(p, s)`: la più piccola i ≥ p con A[p] + … + A[i] ≥ s | O(log n) | visita ricorsiva con stato `&mut u64` |
| E61 | Range add + range minimum con **lazy propagation** | O(log n) per operazione | `&mut self` anche nella query, `std::mem::take` |
| E62 | Persistente, **online**: quanti valori di A[i..j] sono ≤ x? | O(log n) a query | arena di nodi con indici, `partition_point` |
| E63 | **Powerful Array** con Mo (compito a voce del prof, fine della lezione registrata) | Θ((n + q)·√n) | `sort_by_key` con tupla `(i / √n, j)`, risposte nell'ordine originale |
| E64 | Valori distinti in A[l..r], offline, con Mo | Θ((n + q)·√n) | stesso schema, add/remove diversi |

Gli altri due esercizi delle note sono già nelle righe della pagina del corso:
*Nested Segments con segment tree* (opzionale, riga del 26/10/2026) e *Circular RMQ* (opzionale, riga del 02/11/2026).
Nessuno di questi è segnato come obbligatorio.

## Suggerimenti (solo se sei bloccato)

- **E59** Il neutro serve solo per il "no overlap". Combina SEMPRE sinistra prima, destra dopo: il test con le
  stringhe lo controlla. Per RMQ basta il minimo sulle tuple (valore, posizione).
- **E60** Visita i nodi di [p, n−1] da sinistra a destra. Un nodo tutto dentro con somma < s: sottrai e passa oltre
  senza scendere. Il primo nodo tutto dentro con somma ≥ s: scendi come nel search delle note. Controllo: search(2, 8) = 4.
- **E61** Come la `LazySegmentTree` del prof, ma un nodo contiene il minimo: applicare +v a un nodo = +v al suo minimo
  (non v·(r−l+1)), e il neutro della query è +∞.
- **E62** Ordina le posizioni per valore e inseriscile una alla volta (+1 alla posizione) in un segment tree persistente
  indicizzato per posizione. La versione k contiene le posizioni dei k valori più piccoli: con k = #valori ≤ x
  (binary search sui valori ordinati) la risposta è una range_sum(i, j) sulla versione k.
- **E63** Come in Three or More: finestra [cur_i, cur_j), quattro while, e ordina le query con chiave (i / √n, j)
  ricordando l'indice originale. Aggiungere un e che compare già K volte cambia la somma di e·((K+1)² − K²) = e·(2K+1);
  togliere: e·(2K−1) con K prima della rimozione. Controllo della slide: power(1, 6) = 25.
- **E64** add: se il contatore passa da 0 a 1, +1; remove: se torna a 0, −1.
