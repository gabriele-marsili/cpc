# Practice — L14 — segment tree: RMQ dinamico, minimo con occorrenze, Search(s)

Esercizi sulla lezione del 02/11 (dalla registrazione). Soluzioni in `soluzioni/lib.rs`
(fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l14
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E56 | Dynamic RMQ: `add(i, v)`, `rmq(l, r)` = posizione del minimo (a parità la più a sinistra) | Θ(log n) per operazione | nodi che contengono solo la posizione, `Option<usize>` per "no overlap" |
| E57 | RMQ con occorrenze: `query(l, r)` = (minimo, numero di occorrenze) | Θ(log n) per operazione | funzione `combine` su tuple, `Vec<(i64, usize)>` |
| E58 | `Search(s)` su valori non negativi: discesa Θ(log n) e binary search for the answer Θ(log² n) | Θ(log n) vs Θ(log² n) | `u64` senza underflow, ciclo iterativo sulla discesa |

Per il 02/11 la pagina del corso ha la riga "Segment Trees: Lazy Propagation" con *Circular RMQ*
(CF 52C) **opzionale**: a lezione la lazy propagation non è stata fatta (vedi L15 provvisoria).
Compito a voce del prof: pensare a una soluzione di **#occs(i, j, x)** prima della lezione successiva.

## Suggerimenti (solo se sei bloccato)

- **E56** La query è identica alla range sum di E55: cambia solo come combini (il minimo, non la somma)
  e cosa restituisci con "no overlap" (`None`). Per la parità confronta prima i valori, poi le posizioni.
- **E57** Combinare (m, c) e (m', c'): stesso minimo → (m, c + c'); altrimenti vince il minimo più piccolo
  e il suo contatore. Controlla con la lavagna: dopo `add(4, 13)` la query [2, 5] dà (0, 1), non (0, 2).
- **E58** In ogni nodo guarda SOLO il figlio sinistro: se la sua somma è >= s scendi a sinistra, altrimenti
  vai a destra cercando s − somma sinistra ("si sottrae solo andando a destra"). Lavagna: Search(22) = 5.
  Per `search_bs` il predicato "somma del prefisso fino a mid >= s" è monotono perché i valori sono >= 0.
