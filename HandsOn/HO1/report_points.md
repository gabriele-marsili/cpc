# Hands-On 1: punti da motivare nel report

Traccia per scrivere `Handson_1_solution_YOUR_NAME.pdf`. Riferita al codice in `src/lib.rs`.

## Esercizio 1, `is_bst`

1. **Perché non basta confrontare padre e figli.** Un BST impone un vincolo su tutto il
   sottoalbero, non solo sui figli diretti. Controesempio: `10 → 5 → destro 15`.
2. **L'intervallo ammesso `(low, high)`.** Come si restringe scendendo: a sinistra
   `(low, key-1)`, a destra `(key, high)`; alla radice parte da `(u32::MIN, u32::MAX)`.
3. **Convenzione sulle chiavi uguali.** Ammesse solo a destra (`l_key >= key` → `false`).
   La traccia non lo specifica: va dichiarato.
4. **`saturating_sub(1)`.** Con `key = 0`, `key - 1` andrebbe in panic. Fermarsi a 0 resta
   corretto perché, se c'è un figlio sinistro, il controllo diretto lo rifiuta prima.
5. **Complessità.** Tempo O(N) (ogni nodo visitato una volta), spazio O(H) per la
   ricorsione (H = altezza; nel caso peggiore, un albero a catena, H = N).

## Esercizio 2, `sum_max_simple_path`

1. **L'idea della visita.** `dfs` restituisce al padre il miglior cammino che scende fino a
   una foglia, e intanto aggiorna `global_max` con il miglior cammino che "piega" sul nodo
   corrente.
2. **Perché `global_max` si aggiorna solo con entrambi i figli.** Con un figlio solo,
   `local_sum` sarebbe un cammino che finisce su un nodo che non è una foglia.
   Controesempio: `1000 → 1 → (1, 1)` deve dare 3, non 1002.
3. **Perché `return key + l_max.max(r_max)` funziona anche con un figlio solo.** Il figlio
   che manca vale 0 e le chiavi sono `u32` (non negative), quindi il massimo sceglie sempre
   il lato che esiste.
4. **Caso limite: meno di 2 foglie.** Nodo singolo e catena restituiscono 0. Dichiarare la
   scelta e il suo limite: 0 è anche la risposta di un albero con chiavi tutte a zero, quindi
   i due casi non si distinguono. Alternativa: restituire `Option<u32>`.
5. **Complessità.** Tempo O(N), spazio O(H), come per l'esercizio 1.

## Limiti e test

1. **Overflow.** Le somme sono `u32`: con chiavi grandi possono superare `u32::MAX`, e in
   debug il programma va in panic. Dichiararlo come limite, oppure spiegare perché lo si
   accetta (anche `sum()` del prof usa `u32`).
2. **Casi coperti dai test.**
   - BST: nodo singolo, BST valido, figlio diretto sbagliato, nipote che viola il nonno,
     duplicati, chiavi estreme (0 e `u32::MAX`).
   - Cammino: nodo singolo, catena, radice con due foglie, nodo con un figlio solo, cammino
     che non passa dalla radice.
3. **Requisiti del README.** Tutti i test passano; `cargo fmt` e `cargo clippy` eseguiti.
