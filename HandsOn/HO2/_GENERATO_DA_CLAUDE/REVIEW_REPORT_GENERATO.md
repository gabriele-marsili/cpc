# Review guidata del report HO2 (GENERATO DA CLAUDE)

Scopo: arrivare a poter difendere ogni frase del report a voce, come all'orale.
Metodo: per ogni blocco del report leggi la frase, apri il codice alla riga indicata, rispondi alle
domande **senza guardare** `SPIEGAZIONE_CODICE.md`. Se una risposta non ti viene, quel punto non è ancora tuo.
Le risposte non sono qui di proposito.

File: `report/Handson_2_solution_Gabriele_Marsili.tex`, `src/lib.rs`, `src/main.rs`.

---

## 0. Cose da correggere o decidere nel report (trovate rileggendo report vs codice)

1. **Use of AI (riga 124-126 del .tex) — da decidere tu.** Il testo dice che Claude ti ha aiutato con lazy
   propagation e P2 e ha fatto "review of *my* code". Secondo lo stato al 10/11 il codice in `src/` è stato
   scritto in gran parte con Claude (sessione 6/11). La sezione deve descrivere com'è andata davvero: o la
   riformuli tu, o riscrivi a mano le parti e allora la frase torna vera. Io non la riscrivo.
2. **Numero di riga sbagliato (riga 55 del .tex):** "the update recomputes the node ... (line 181)". In `lib.rs`
   la riga 181 è la firma di `update_rec`; il ricalcolo `t[k] = max(figli)` è alla **riga 195**.
   Gli altri riferimenti (112, 157, 167, 226, 241, 309, 318, 330) corrispondono.
3. **"built bottom-up in O(n)" (riga 36-37):** `build` è ricorsivo top-down che calcola i nodi in post-ordine.
   Il costo O(n) è giusto, ma a lezione "bottom-up" era la costruzione iterativa dalle foglie. Scegli tu la
   parola (es. "recursively, each node from its two children").
4. **Numeri di performance (riga 111-113):** 0,28 s, 0,13 s, ">60 s" vengono dalla sessione del 6/11. Se li
   lasci, devi averli misurati tu: rilancia la misura prima di consegnare oppure toglili.
5. **Riferimenti alle righe in generale:** se modifichi `lib.rs` (anche solo `cargo fmt`), ricontrolla tutti
   i "line N" del report.

---

## 1. Introduzione (righe 19-27)

- Cosa contiene `lib.rs` e cosa `main.rs`? Perché `main.rs` importa da `handson2`?
- Quali tre modi di dare l'input supporta `main.rs` (righe 49-65)? Cosa succede se passi un `.txt` che non esiste?
- Perché l'output è scritto con un `BufWriter` (riga 80-86)?

## 2. Problema 1 — Min and Max

### Indici (riga 31-32 / `solve_p1`, lib.rs 226-237)
- Perché `i - 1` e `j - 1`? Cosa succederebbe con un input che usa indici da 0?

### Struttura (righe 34-37 / lib.rs 112-144)
- Disegna l'albero per A = [5, 1, 4, 3, 2]: per ogni nodo k scrivi [lo, hi] e t[k].
- Perché 4n posizioni bastano sempre? (Saperlo argomentare, non solo dirlo.)
- Perché `t` è inizializzato a `i64::MIN`?

### Max(i, j) (righe 39-41 / lib.rs 199-221)
- I tre casi della visita: nessuna sovrapposizione, contenuto, parziale. Cosa ritorna ciascuno?
- Perché al massimo 4 nodi per livello? (È la domanda di L13/L14: devi saperla dimostrare.)
- Perché `max` prende `&mut self`?

### Update con lazy propagation (righe 43-58 / lib.rs 154-196)
- Dimostra a voce: max_p min(A[p], T) = min(max_p A[p], T). Perché questa identità è ciò che rende possibile il lazy?
- Cosa contiene `lazy[k]` e a chi si riferisce: al nodo k o ai suoi figli?
- `apply` (157): perché aggiorna sia `t[k]` sia `lazy[k]`? Perché due min pendenti si fondono in uno?
- `push` (167): quando viene chiamato, e perché **prima** di scendere? Perché non viene mai chiamato su una foglia?
- Riga 183, `self.t[k] <= v`: perché è corretto fermarsi senza registrare niente? È solo un'ottimizzazione o
  serve per la complessità?
- Riga 195: perché va ricalcolato `t[k]` dopo le chiamate ricorsive?
- Simula a mano l'esempio della consegna (`0 1 2 2`, `1 2 4`, `1 1 2` → `4`, `2`) sull'albero disegnato sopra,
  segnando quali nodi ricevono un lazy.
- Simula il test `p1_update_covering_whole_array_then_partial_max` (lib.rs 521-525): dove finisce il lazy e
  come arriva alla foglia 3?

### Complessità (righe 60-64)
- Perché l'update costa O(log n) e non O(numero di nodi toccati dal range)?
- Spazio: da dove viene O(n)?

## 3. Problema 2 — Is There

### Coverage (righe 68-75 / lib.rs 308-319)
- Spiega il difference array sull'esempio: segmenti (0,4) (1,3) (1,2) (1,1) (0,0) → cover = ?
- Perché `diff` ha n+1 posizioni? Perché `r + 1` non va mai fuori?
- Perché 0 ≤ cover[p] ≤ n?

### Query offline per k (righe 77-90 / lib.rs 321-345, CountTree 239-289)
- Perché un segment tree di max o di somma su `cover` **non** risponde a "c'è il valore k in [i, j]"?
- Cosa vuol dire "offline" qui, e cosa si perderebbe se le query arrivassero una alla volta?
- Cosa memorizza `CountTree` in ogni nodo?
- Il ciclo su k: attiva, rispondi, disattiva. Perché bisogna disattivare? Cosa succederebbe senza?
- Perché le risposte restano nell'ordine dell'input?
- Perché saltare i k senza query non cambia la complessità ma è comunque utile?
- Simula l'esempio della consegna e ottieni 1 0 0 1.

### Complessità (righe 92-97)
- Perché ogni posizione è attivata e disattivata **al più una volta** in tutto il ciclo? Da qui O(n log n).
- Il ciclo `for k in 0..=n` da solo quanto costa?
- Esiste una soluzione online? (Domanda possibile all'orale: pensaci, non serve nel report.)

## 4. Test (righe 99-114)
- Cosa confronta il runner `testset_GENERATO.rs`, e perché non va consegnato?
- Come funziona il confronto con brute force (lib.rs 480-586)? Perché n piccolo?
- Rilancia tu: `cargo test -p handson2`, `cargo fmt -p handson2`, `cargo clippy -p handson2` (al 13/11 erano tutti verdi).

## 5. Fonti e Use of AI (righe 116-126)
- Vedi punto 0.1. La frase deve essere vera parola per parola.

---

## Checklist prima della consegna
- [ ] Ogni domanda sopra: risposta data a voce senza aiuti
- [ ] Use of AI riformulata da te e vera
- [ ] Riga 181 → 195 corretta, "bottom-up" deciso, numeri di performance rimisurati o tolti
- [ ] Test verdi, `cargo fmt`, `cargo clippy` puliti
- [ ] PDF ricompilato dal .tex
- [ ] Email a rossano.venturini@gmail.com con `main.rs`, `lib.rs`, `Handson_2_solution_Gabriele_Marsili.pdf` entro il 22/11/2026
