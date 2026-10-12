# Practice — L8 (12/10) — Gomory, Alice & Bob, closest pair of points

Esercizi dalla registrazione della L8. Soluzioni in `soluzioni/lib.rs` (fuori dal crate; scritte da Claude, non dal prof).

```
cargo test -p practice_l8            # tutta la lezione
cargo test -p practice_l8 e38        # un esercizio
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E35 | Domino sulla scacchiera senza 2 celle di colore diverso (puzzle, teorema di Gomory) | Θ(nm) | costruire un ciclo, indici modulo len |
| E36 | Alice & Bob: strategia pari/dispari (Alice non perde mai) | Θ(n) | `step_by`, `skip` |
| E37 | Alice & Bob: valore ottimo del gioco (anteprima di DP) | Θ(n²) | tabella `Vec<Vec<i64>>` o memo `HashMap` |
| E38 | Closest pair of points (sweep line + BST per y) | Θ(n log n) | `BTreeSet::range` su tuple, cancellazioni |

## Suggerimenti (solo se sei bloccato)

- **E35** Ciclo per m pari: riga 0 da sinistra a destra, serpentina sulle colonne m-1..1 (righe 1..n-1), risalita sulla colonna 0.
  Togliendo due celle di colore diverso restano due catene di lunghezza pari: prendi le celle a coppie.
- **E36** Con lunghezza pari le due estremità hanno indici di parità diversa: prendi sempre quella della tua classe.
- **E37** `dp[l][r] = max(c[l] - dp[l+1][r], c[r] - dp[l][r-1])`, riempita per lunghezze crescenti (o `l` decrescente).
- **E38** δ diminuisce soltanto: un punto con `x' < x - δ` non servirà mai più, quindi cancellalo quando lo incontri nel range.
