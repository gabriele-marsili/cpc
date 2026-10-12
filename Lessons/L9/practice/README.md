# Practice — L9 (PROVVISORIA) — Frogs and Mosquitoes

Problema **obbligatorio** per l'esame, annunciato a fine L7 ma non ancora spiegato (né in L7 né in L8).
Era l'E30 di L7. Soluzione in `soluzioni/lib.rs` (fuori dal crate; scritta da Claude, non dal prof). Quando arriva la
lezione verrà verificato e ampliato (questo esercizio non verrà toccato se lo modifichi).

```
cargo test -p practice_l9
```

| # | Esercizio | Obiettivo | Rust che alleni |
|---|---|---|---|
| E39 | Frogs and Mosquitoes ([CF 609F](https://codeforces.com/contest/609/problem/F)) | Θ((n+m) log(n+m)) | `BTreeMap::range`, predecessore |

Prova anche a sottometterlo su Codeforces: all'esame va consegnato in Rust.

## Suggerimenti (solo se sei bloccato)

- **E39** Ordina le rane per x e dai a ognuna l'intervallo `[max(x, fine precedente + 1), x + t]`: così il predecessore di b
  nel BST è l'unica candidata. Quando una lingua cresce, togli dal BST le rane successive coperte del tutto e sposta
  l'inizio di quella coperta in parte; poi fai mangiare alla rana le zanzare in attesa nel suo intervallo.
