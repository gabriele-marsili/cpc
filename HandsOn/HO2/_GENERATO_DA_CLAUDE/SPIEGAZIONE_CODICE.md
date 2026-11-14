# Hands-On 2: spiegazione del codice (GENERATO DA CLAUDE, non si consegna)

Righe riferite a `src/lib.rs` e `src/main.rs` al 2026-11-06.

## 0. Cosa si consegna

Email a rossano.venturini@gmail.com con:

1. `src/main.rs`
2. `src/lib.rs`
3. `report/Handson_2_solution_Gabriele_Marsili.pdf`

Prima di inviare: rileggi la sezione "Use of AI" in fondo al PDF (dice che Claude ti ha aiutato con
lazy propagation e Problema 2) e cambiala se non corrisponde a come vuoi dichiararlo.
Il sorgente del PDF è `report/Handson_2_solution_Gabriele_Marsili.tex`; si ricompila con
`lualatex Handson_2_solution_Gabriele_Marsili.tex` (non xelatex: non trova il font).

Comandi di controllo, tutti verdi al 2026-11-06:

    cargo test -p handson2                        # 15 unit test + test set ufficiale (P1 11/11, P2 8/8)
    cargo clippy -p handson2 --all-targets -- -D warnings
    cargo fmt -p handson2 --check

## 1. Come è diviso il codice

- `main.rs`: solo input e output. Legge l'input (stdin, file o argomenti), chiama `solve_p1` o
  `solve_p2`, stampa una risposta per riga (`print_lines`, con `BufWriter`: stampare riga per
  riga senza buffer su 200.000 righe è lento).
- `lib.rs`, in quest'ordine:
  1. parsing (`parse_p1`, `parse_p2`): trasformano una sequenza di "parole" (token) nelle struct
     `P1Input`, `P2Input`;
  2. `SegmentTree` + `solve_p1` (Problema 1);
  3. `CountTree` + `solve_p2` (Problema 2);
  4. i test (`mod tests`).

Il tree che avevi in `main.rs` è stato spostato in `lib.rs` e completato: così i test lo vedono.

## 2. Il segment tree come array

Il tree non ha puntatori: è un `Vec` chiamato `t`. La radice sta in posizione 1, e per il nodo `k`:

    figlio sinistro = 2k      figlio destro = 2k + 1

Ogni nodo copre un pezzo contiguo di A e contiene il **max di quel pezzo**.

### Come si divide un pezzo (lib.rs:140)

Un nodo che copre `lo..hi` si taglia a metà con `mid = (lo + hi) / 2`:

- figlio sinistro: `lo..mid`
- figlio destro: `mid+1..hi`

Se il pezzo ha un numero dispari di elementi, quello in più va a **sinistra**. Si continua
finché il pezzo ha un solo elemento (foglia).

### Esempio: A = [5, 1, 4, 3, 2], posizioni 0..4

Divisione, dall'alto verso il basso:

| nodo | copre | valori      | mid | figli                      |
|------|-------|-------------|-----|----------------------------|
| 1    | 0..4  | 5 1 4 3 2   | 2   | nodo 2 = 0..2, nodo 3 = 3..4 |
| 2    | 0..2  | 5 1 4       | 1   | nodo 4 = 0..1, nodo 5 = 2..2 |
| 3    | 3..4  | 3 2         | 3   | nodo 6 = 3..3, nodo 7 = 4..4 |
| 4    | 0..1  | 5 1         | 0   | nodo 8 = 0..0, nodo 9 = 1..1 |
| 5..9 | un elemento | foglie | -  | -                          |

Il 4 resta da solo perché `[5, 1, 4]` si taglia in `[5, 1]` e `[4]`.

Ogni nodo contiene il max del suo pezzo, quindi l'array `t` è:

| indice `k` | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|------------|---|---|---|---|---|---|---|---|---|
| copre      | 0..4 | 0..2 | 3..4 | 0..1 | 2 | 3 | 4 | 0 | 1 |
| `t[k]`     | 5 | 5 | 3 | 5 | 4 | 3 | 2 | 5 | 1 |

Lo stesso, disegnato (ogni nodo ha sotto, rientrati, i suoi due figli):

    nodo 1  (0..4)  [5 1 4 3 2] → 5
    ├── nodo 2  (0..2)  [5 1 4] → 5
    │   ├── nodo 4  (0..1)  [5 1] → 5
    │   │   ├── nodo 8  (0)  [5] → 5
    │   │   └── nodo 9  (1)  [1] → 1
    │   └── nodo 5  (2)  [4] → 4
    └── nodo 3  (3..4)  [3 2] → 3
        ├── nodo 6  (3)  [3] → 3
        └── nodo 7  (4)  [2] → 2

`lo` e `hi` non sono salvati: ogni funzione li riceve come parametri, partendo da `(1, 0, n-1)`.
Per questo tutte le funzioni hanno la firma `(k, lo, hi, ...)`. L'array ha `4n` posti, che
bastano sempre.

## 3. Problema 1, Max(i, j) (`max_rec`, riga 209)

Per il nodo `k` che copre `lo..hi` ci sono tre casi:

1. **fuori**: il pezzo non tocca `[i, j]`. Restituisce `i64::MIN` (non vince mai un max).
2. **tutto dentro**: il pezzo sta tutto in `[i, j]`. Restituisce `t[k]` e non scende.
3. **a metà**: chiede ai due figli e restituisce il più grande.

### Esempio: Max(1, 3), cioè il max di A[1], A[2], A[3] = 1, 4, 3. Risposta attesa: 4

Si parte dalla radice. Ogni riga è un nodo visitato:

| nodo | copre | rispetto a [1, 3] | restituisce |
|------|-------|-------------------|-------------|
| 1    | 0..4  | a metà, scende    | max(4, 3) = **4** |
| 2    | 0..2  | a metà, scende    | max(1, 4) = 4 |
| 4    | 0..1  | a metà, scende    | max(MIN, 1) = 1 |
| 8    | 0     | fuori             | MIN |
| 9    | 1     | tutto dentro      | 1 |
| 5    | 2     | tutto dentro      | 4 |
| 3    | 3..4  | a metà, scende    | max(3, MIN) = 3 |
| 6    | 3     | tutto dentro      | 3 |
| 7    | 4     | fuori             | MIN |

Si legge dal basso: le foglie danno 1, 4, 3, e salendo vince il 4.
Per ogni livello si visitano al massimo 4 nodi, quindi O(log n).

## 4. Problema 1, Update con lazy propagation

### Perché non basta aggiornare foglia per foglia

`Update(i, j, T)` cambia tutte le posizioni da i a j. Una foglia alla volta costa
`(j - i + 1) * log n`: se l'intervallo è tutto l'array, è O(n log n) per ogni query. Misurato:
con n = m = 200.000 la versione foglia per foglia ha superato i 60 secondi (interrotta),
quella lazy impiega 0,28 s.

### L'idea

Ci si ferma ai nodi **tutto dentro**, come nella query. Si può fare perché

    max( min(A[lo], T), ..., min(A[hi], T) ) = min( max(A[lo..hi]), T )

Esempio: pezzo `[5, 1]`, T = 2. Fare il min con 2 su ogni elemento dà `[2, 1]`, il cui max è 2.
Oppure si prende il max vecchio (5) e si fa `min(5, 2) = 2`. Stesso risultato, senza toccare
i figli.

I figli però restano con i valori vecchi. Allora il nodo si segna un promemoria
`lazy[k] = Some(T)`: "ai miei figli devo ancora applicare min con T". Il promemoria si usa
solo quando qualcuno deve scendere sotto quel nodo.

### Le tre funzioni

- `apply(k, v)` (riga 157): `t[k] = min(t[k], v)` e `lazy[k] = min(lazy[k], v)`.
  Due promemoria si fondono: min con 5 e poi min con 3 equivale a min con 3.
- `push(k)` (riga 167): passa il promemoria ai figli (`apply(2k, v)`, `apply(2k+1, v)`) e
  cancella il proprio (`lazy[k] = None`). Si chiama **prima di scendere** sotto un nodo, sia
  nell'update sia nella query.
- `update_rec` (riga 181): stessi tre casi del max, più una scorciatoia:
  - fuori, oppure `t[k] <= v` (il pezzo è già tutto ≤ v): non fa niente;
  - tutto dentro: `apply(k, v)` e si ferma;
  - a metà: `push(k)`, scende nei due figli, poi `t[k] = max(t[2k], t[2k+1])`.

`max` ora prende `&mut self` perché scendendo chiama `push`, che modifica i figli.

### Esempio passo passo

Stesso A = [5, 1, 4, 3, 2] e stesso tree della sezione 2. Query della consegna, già
convertite in 0-based: **Update(0, 1, 2)**, poi **Max(1, 3)**, poi **Max(0, 1)**.

**Update(0, 1, 2)**: A[0] e A[1] diventano min con 2, cioè A vero = [2, 1, 4, 3, 2].

| nodo | copre | rispetto a [0, 1] | cosa fa |
|------|-------|-------------------|---------|
| 1    | 0..4  | a metà            | scende |
| 2    | 0..2  | a metà            | scende |
| 4    | 0..1  | **tutto dentro**  | `t[4] = min(5, 2) = 2`, `lazy[4] = 2`, si ferma |
| 5    | 2     | fuori             | niente |
| 2    | (risalendo) |             | `t[2] = max(t[4], t[5]) = max(2, 4) = 4` |
| 3    | 3..4  | fuori             | niente |
| 1    | (risalendo) |             | `t[1] = max(t[2], t[3]) = max(4, 3) = 4` |

Stato dopo l'update (cambiano solo i nodi 1, 2, 4):

| `k`      | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|----------|---|---|---|---|---|---|---|---|---|
| `t[k]`   | 4 | 4 | 3 | **2** | 4 | 3 | 2 | 5 | 1 |
| `lazy[k]`| - | - | - | **2** | - | - | - | - | - |

Il nodo 8 dice ancora 5, che è sbagliato (A[0] ora è 2). Non è un problema: il nodo 4 ha il
promemoria, e chiunque voglia scendere sotto il nodo 4 chiamerà prima `push(4)`.

**Max(1, 3)**: stessa visita della tabella della sezione 3. Quando arriva al nodo 4 (a metà),
prima di scendere chiama `push(4)`:

- `t[8] = min(5, 2) = 2`, `t[9] = min(1, 2) = 1`, `lazy[4] = None`.

Poi nodo 9 dà 1, nodo 5 dà 4, nodo 6 dà 3. Risultato **4**, come nell'output atteso.

**Max(0, 1)**: il nodo 4 copre 0..1, è tutto dentro, restituisce `t[4] = 2`. Risultato **2**.

### Complessità P1

Build O(n), ogni update e ogni query O(log n): totale O((n + m) log n). Spazio O(n).

## 5. Problema 2, Is There

### Cosa chiede

n segmenti `(l, r)` su posizioni `0..n-1`. `cover[p]` = quanti segmenti contengono `p`.
`IsThere(i, j, k)` = 1 se esiste `p` in `[i, j]` con `cover[p] == k` esattamente.

Un max o una somma su `cover` non risponde a "il valore k compare nel range?". Per questo si
rovescia il problema: si processano le query **per valore di k** (offline, cioè non nell'ordine
in cui arrivano).

### Passo 1: cover con l'array delle differenze (riga 309)

Per ogni segmento `(l, r)`: `diff[l] += 1`, `diff[r + 1] -= 1`. Sommando `diff` da sinistra si
ottiene `cover[p]`. Costo O(n) invece di O(n * lunghezza dei segmenti).

Esempio della consegna, segmenti (0,4) (1,3) (1,2) (1,1) (0,0):

    diff  = [ 2, 2, -1, -1, -1, -1 ]  (posizioni 0..5)
    cover = [ 2, 4, 3,  2,  1 ]       (somme progressive)

Intanto le posizioni vengono raggruppate per valore: `positions_with[k]` = posizioni con cover k.
Qui `positions_with[2] = [0, 3]`, `[4] = [1]`, `[3] = [2]`, `[1] = [4]`.

### Raggruppare le query per k (riga 322)

Problema: per rispondere a IsThere(i, j, k) nell'albero devono essere accese **solo** le
posizioni con cover = k. Se si rispondesse alle query nell'ordine dell'input, a ogni query
bisognerebbe spegnere le posizioni del k precedente e accendere quelle del nuovo. Se le query
alternano due k "grandi", si riaccendono le stesse posizioni a ogni query: O(n) per query.

Soluzione: si mettono in fila tutte le query con lo stesso k, così ogni gruppo di posizioni si
accende una volta sola.

`queries_with[k]` = lista degli **indici** delle query che chiedono quel k. Si salva l'indice
(0, 1, 2, ...) e non la query, per poter scrivere la risposta in `ans[indice]`: così alla fine
le risposte escono nell'ordine dell'input anche se sono state calcolate in un altro ordine.

Esempio: i segmenti della consegna (`cover = [2, 4, 3, 2, 1]`) più una terza query.

| indice | query (i, j, k) |
|--------|-----------------|
| 0      | (0, 4, 4)       |
| 1      | (1, 3, 1)       |
| 2      | (0, 1, 4)       |

Raggruppamento:

    queries_with[1] = [1]
    queries_with[4] = [0, 2]
    tutti gli altri  = []

Il ciclo `for k in 0..=n` (riga 330) allora fa:

| k | query del gruppo | posizioni accese | conti | risposte scritte |
|---|------------------|------------------|-------|------------------|
| 0 | nessuna, salta   | -                | -     | -                |
| 1 | [1]              | {4}              | q1: count(1, 3) = 0 | `ans[1] = 0` |
| 2 | nessuna, salta   | -                | -     | -                |
| 3 | nessuna, salta   | -                | -     | -                |
| 4 | [0, 2]           | {1}              | q0: count(0, 4) = 1; q2: count(0, 1) = 1 | `ans[0] = 1`, `ans[2] = 1` |
| 5 | nessuna, salta   | -                | -     | -                |

Le query sono state calcolate nell'ordine 1, 0, 2, ma `ans = [1, 0, 1]` è già nell'ordine
dell'input. Output del programma su questo input: `1`, `0`, `1`.

Una query con k > n non entra in nessun gruppo e resta a 0, il valore iniziale di `ans`.

### Passo 2: per ogni k, accendi, rispondi, spegni (riga 330)

### Perché serve un albero

Fissato k, ogni posizione è "accesa" (cover = k) o "spenta". La query chiede: in `[i, j]` c'è
almeno una posizione accesa? Con un semplice array di bool bisognerebbe scorrere tutto
`[i, j]`: O(n) per query, cioè 200.000 × 200.000 = 4 · 10^10 passi nel caso peggiore. Troppo.

Se invece si sa **quante** posizioni accese ci sono in `[i, j]`, basta guardare se sono più di 0.
"Quante in un range" è una somma su range, e la somma su range in O(log n) è esattamente quello
che fa un segment tree.

### `CountTree` è lo stesso albero del P1

Stessa forma, stessi indici (`2k`, `2k+1`), stesso `mid`, stessi tre casi nella query
(fuori, tutto dentro, a metà). Cambiano tre cose:

| | P1 `SegmentTree` | P2 `CountTree` |
|-|------------------|----------------|
| valore di una foglia | A[p] | 1 se p è accesa, 0 se spenta |
| valore di un nodo | max dei figli | **somma** dei figli |
| update | range, con lazy | una posizione sola (`set`), niente lazy |
| query fuori dal range restituisce | `i64::MIN` | 0 |

### Esempio: k = 2

Dall'esempio della consegna `cover = [2, 4, 3, 2, 1]`, quindi con k = 2 si accendono le
posizioni 0 e 3. Foglie: `[1, 0, 0, 1, 0]`. L'albero ha la stessa forma della sezione 2 (ogni nodo ha sotto, rientrati, i suoi due figli):

    nodo 1  (0..4) → 2
    ├── nodo 2  (0..2) → 1
    │   ├── nodo 4  (0..1) → 1
    │   │   ├── nodo 8  (0) → 1
    │   │   └── nodo 9  (1) → 0
    │   └── nodo 5  (2) → 0
    └── nodo 3  (3..4) → 1
        ├── nodo 6  (3) → 1
        └── nodo 7  (4) → 0

Ogni nodo dice quante posizioni accese ci sono nel suo pezzo (la radice: 2 in tutto).

Query IsThere(1, 3, 2), cioè `count(1, 3)`:

| nodo | copre | rispetto a [1, 3] | restituisce |
|------|-------|-------------------|-------------|
| 1    | 0..4  | a metà            | 0 + 1 = **1** |
| 2    | 0..2  | a metà            | 0 + 0 = 0 |
| 4    | 0..1  | a metà            | 0 + 0 = 0 |
| 8    | 0     | fuori             | 0 |
| 9    | 1     | tutto dentro      | 0 |
| 5    | 2     | tutto dentro      | 0 |
| 3    | 3..4  | a metà            | 1 + 0 = 1 |
| 6    | 3     | tutto dentro      | 1 |
| 7    | 4     | fuori             | 0 |

`count = 1 > 0`, quindi la risposta è 1 (infatti cover[3] = 2). Con `count(1, 2)` invece
uscirebbe 0: in posizione 1 e 2 cover vale 4 e 3, mai 2.

### `set(pos, attiva)` (riga 256)

Scende dalla radice verso la foglia di `pos`, sempre in un solo figlio (sinistro se
`pos <= mid`), mette la foglia a 1 o 0, e risalendo ricalcola ogni nodo come somma dei figli.
Tocca un nodo per livello: O(log n).

### Perché si spegne dopo

L'albero è uno solo per tutti i k. Finite le query di k = 2, le posizioni 0 e 3 vanno
rimesse a 0, altrimenti con k = 3 risulterebbero ancora accese. Ogni posizione ha un solo
valore di cover, quindi viene accesa e spenta una volta sola in tutto il programma.

Per ogni k da 0 a n (saltando i k senza query):

1. accende le posizioni di `positions_with[k]`;
2. ogni query con quel k: risposta 1 se `count(i, j) > 0`, cioè c'è almeno una posizione accesa
   in `[i, j]`;
3. le rispegne, così per il k successivo il tree è di nuovo vuoto.

Esempio, query `(0, 4, 4)`: con k = 4 si accende solo la posizione 1; `count(0, 4) = 1 > 0`,
quindi **1**. Query `(1, 3, 1)`: con k = 1 si accende la posizione 4; `count(1, 3) = 0`, quindi
**0**. Corretto.

Le risposte vengono salvate in `ans[indice della query]`, quindi escono nell'ordine dell'input
anche se calcolate in un altro ordine. Una query con `k > n` vale 0 senza calcoli: nessuna
posizione può essere coperta da più di n segmenti.

### Complessità P2

Ogni posizione si accende e si spegne una volta sola, quindi O(n log n). Ogni query è una somma
su range, O(log n). I raggruppamenti costano O(n + m). Totale O((n + m) log n), spazio O(n + m).
Misurato con n = m = 200.000: 0,13 s in release.

## 6. I test (in fondo a `lib.rs`)

- **esempi della consegna** per P1 e P2;
- `p1_update_covering_whole_array_then_partial_max`: update su tutto l'array (il debito resta
  sulla radice) e poi una query su una sola posizione. Se `push` non funzionasse, leggerebbe il
  valore vecchio;
- `p2_k_larger_than_n_is_never_there`;
- **test casuali**: 3000 input piccoli per problema, confrontati con una versione ingenua
  (`brute_p1`, `brute_p2`) che fa letteralmente quello che dice la consegna. È il test che
  trova gli errori nei casi a cui non hai pensato;
- **parsing**: input con commenti `//`, tutto su una riga, troncato, con token non numerici, e tutti
  i file ufficiali letti fino in fondo.

Il test set ufficiale gira con il runner `_GENERATO_DA_CLAUDE/testset_GENERATO.rs`:
`cargo test -p handson2 --test testset`.

## 7. rust-analyzer in VS Code

Il server dentro l'estensione è più nuovo del tuo Rust (1.89) e non leggeva la libreria standard:
segnava `println!`, `vec!`, `format!` come errori. In `CPC/.vscode/settings.json` è impostato il
rust-analyzer della toolchain, con il controllo al salvataggio acceso. Se torna a non mostrare
errori: `Cmd+Shift+P`, poi "Developer: Reload Window".
In alternativa `rustup update stable` aggiorna Rust per tutti i progetti (OctechCode compreso),
e allora il server dell'estensione torna compatibile.
