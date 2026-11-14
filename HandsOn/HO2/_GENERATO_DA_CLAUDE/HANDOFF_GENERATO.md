# Handoff HO2 — 14/11/2026, 15:30 (GENERATO DA CLAUDE)

Per la prossima sessione Claude (Claude Code) che riprende l'Hands-On 2. Leggi tutto prima di agire.

## Stato dei file
| File | Stato |
|---|---|
| `src/main.rs` | **identico byte per byte** a `_GENERATO_DA_CLAUDE/versione_6nov/main_GENERATO.rs` (generato da Claude il 6/11) |
| `src/lib.rs` (14/11, Octech) | codice = versione del 6/11 con identificatori rinominati (`P1Input`→`InputE1`, `P2Input`→`InputE2`, `Query1`→`QueryE1`, `st`→`s_tree`, `out`→`output`) e commenti riformulati; `mod tests` identico a quello generato |
| `report/` | **vuota**. Il report del 6/11 (scritto con Claude) è in `versione_6nov/report_GENERATO.tex/.pdf` |
| `_GENERATO_DA_CLAUDE/` | `LEGGIMI.md`, `SPIEGAZIONE_CODICE.md`, `REVIEW_REPORT_GENERATO.md` (domande + correzioni sul report del 6/11), `testset_GENERATO.rs` (runner), `versione_6nov/` |

## Risultati verificati il 14/11 (cargo 1.97, copia dei file in sandbox)
- `cargo build`: ok. Clippy: pulito.
- `cargo test -p handson2 --test testset`: **P1 3/11** (5 input vanno in panic), P2 8/8.
- `cargo test -p handson2` (unit test): **non compila**. `mod tests` usa ancora `P1Input`, `P2Input`, `Query1`.
- `cargo fmt --check`: 22 blocchi da formattare.

## Bug in `src/lib.rs`, segnalati a Octech e NON corretti (deve correggerli lui)
1. **Riga 172** `if i <= lo && hi >= j`: deve esprimere "[lo, hi] contenuto in [i, j]" (lo dice anche il commento). Causa le risposte sbagliate del P1.
2. **Righe 161 e 187** `j <= self.n`: j = n passa il controllo, panic alla 168 (`self.t[k]` fuori dai limiti).
3. **Riga 188**: nel messaggio di `assert!` manca `)` dopo `{j}`.
4. `mod tests`: tipi rinominati (vedi sopra).
5. `cargo fmt`.

## Cosa resta per arrivare alla consegna
1. Octech corregge i bug 1–5, poi `cargo test -p handson2` tutto verde (P1 11/11, P2 8/8, unit test), `cargo fmt`, `cargo clippy`.
2. Report `report/Handson_2_solution_Gabriele_Marsili.pdf` (si compila con lualatex). Lo **scrive Octech**, Claude rivede soltanto. `REVIEW_REPORT_GENERATO.md` ha le correzioni note: numero di riga, "bottom-up", tempi da rimisurare.
3. Email a rossano.venturini@gmail.com con `main.rs`, `lib.rs` e il PDF.

## Limiti per Claude (concordati con Octech e ribaditi più volte, 10/11, 13/11 e 14/11)
- Il codice e il report degli hands-on devono essere di Octech. Claude spiega, rivede, segnala errori e genera al massimo test, sempre in questa cartella separata. **Non scrive né corregge** `src/` o `report/`.
- **Non** aiutare a far sembrare non generato il codice o il report (rinominare, riformulare commenti, scrivere il report "con la sua voce"). Octech l'ha chiesto più volte. La risposta è stata no, e va mantenuta.
- La sezione *Use of AI* del report deve descrivere come sono andate le cose: `lib.rs` deriva da una versione scritta con Claude e poi rielaborata, `main.rs` e i test sono generati da Claude. Rivedere un report che lo dichiara va bene. Aiutare a nasconderlo no.
- Hands-on mai pubblicati (`pubblica.json`). Le routine non toccano `HandsOn/`.

## Teoria già ripassata (10/11). Punti su cui Octech ha esitato
- La lazy rimanda il lavoro ai **discendenti** (gli antenati si ricalcolano sempre), grazie a max(min(x,v)) = min(max x, v) e al fatto che due min in sospeso si combinano in uno.
- Al massimo 2 nodi parziali per livello, quindi al massimo 4 visitati: O(log n). Il controllo `t[k] <= v` è solo un'ottimizzazione.
- P2: cover con array di differenze; soluzione offline per k, con ogni posizione attivata e disattivata una sola volta: O((n+m) log n). Una query con k > n vale 0 (e evita l'indice fuori dai limiti). `cover` non è un array, è una variabile che scorre.

## Aggiornamento 14/11 (sera), su richiesta esplicita di Octech
Claude ha corretto `src/lib.rs`: righe 161/187 `j < self.n`, 172 `hi <= j`, 188 parentesi, `mod tests` rinominato,
e un sesto bug (riga 324 `0..n` invece di `0..=n`, P2 saltava k = n). `cargo fmt` applicato.
Esito: unit 15/15, testset 2/2 (P1 e P2), clippy `-D warnings` pulito, fmt ok. Questo va dichiarato in *Use of AI*.
Report: ancora solo `versione_6nov/report_GENERATO.tex`; riferimenti di riga tutti da aggiornare, Use of AI da rendere vera.
