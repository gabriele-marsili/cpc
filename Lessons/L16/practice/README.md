# Practice — L16 — static RMQ (sparse table, blocchi) e alberi in 2n bit

Tracce in `src/lib.rs` (sostituisci i `todo!()`), soluzioni in `soluzioni/lib.rs`
(fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l16
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E65 | Sparse table **generica** per operazioni idempotenti (min, max, gcd) | build Θ(n log n), query O(1) | generics `T: Copy`, `F: Fn(T, T) -> T`, underflow di `usize` |
| E66 | msb con la **tabella da 256** del prof (niente `leading_zeros`) | ≤ 8 accessi | `const` array calcolato a compile time con `while` |
| E67 | Solution #3: **blocchi** di log n + sparse table su A' | query Θ(log n), spazio ≤ 3n parole | casi limite (stesso blocco, blocchi adiacenti), indici su indici |
| E68 | **Domanda del prof** (fine registrazione): albero livello per livello con i gradi in unario. Quanti bit? | encode/decode in Θ(n) | `VecDeque`, `Vec<bool>` |

La pagina del corso non ha righe per il 16/11 né per il 18/11/2026: nessun esercizio assegnato per questa lezione.

## Suggerimenti (solo se sei bloccato)

- **E65** Riga k = risultati sugli intervalli di lunghezza 2^k: `t[k][i] = f(t[k-1][i], t[k-1][i + 2^(k-1)])`.
  Query: k = posizione del bit più significativo di `j - i + 1`, poi `f(t[k][i], t[k][j + 1 - 2^k])`
  (scritto così non va in underflow quando i = 0 e j + 1 = 2^k: il test apposta lo controlla).
  Perché la somma NON funziona? Perché i due intervalli si sovrappongono.
- **E66** `MSB8[b] = MSB8[b / 2] + 1` per b ≥ 2. In `msb`, scorri i byte dal più alto: il primo non nullo decide.
- **E67** b = numero di bit di n (≈ log2 n + 1). Il minimo di ogni blocco è una POSIZIONE in A; la sparse table
  su A' contiene posizioni in A' (indici di blocco). Se `i / b == j / b` basta una scansione.
- **E68** Stessa idea della decodifica a parentesi: leggendo i bit, un `1` crea un nuovo figlio (id = prossimo id libero,
  è l'ordine BFS), uno `0` passa al nodo successivo. Conta: uno `0` per nodo, un `1` per arco.
