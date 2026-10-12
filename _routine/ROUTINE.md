# Routine CPC — aggiornamento appunti delle lezioni

Corso: Competitive Programming and Contests (magistrale AI, Unipi, prof. Rossano Venturini), a.a. 2026/27.
Lezioni: lunedì 11-13, mercoledì 14-16. Lingua delle lezioni: inglese. Appunti prodotti: in inglese (come L2/L3).
Pagina del corso: https://pages.di.unipi.it/rossano/competitive/ (leggila con WebFetch: curl è bloccato dal proxy,
sia nel cloud sia sul Mac). Contiene l'elenco lezioni con i link alle note del prof (blog) e agli esercizi.

Esempio completo di lezione già sistemata: `Lessons/L3/` (L3.tex/L3.pdf, board/, figs/, src/, claude_notes.md,
L3_transcript.txt). Usa `Lessons/L3/L3.tex` come modello: stesso preambolo, stessi comandi `\board`, `\extra`,
stesse box. È lo standard da mantenere.

## Dove stanno le cose
- Mac, shell `device_bash`: `$HOME/mnt/CPC` (cartella del corso) e `$HOME/mnt/Downloads`.
  Per device_stage_files / device_commit_files i percorsi sono
  `/Users/piccoletto/Desktop/Everything/pisa/corsi/magistrale/secondo_anno/CPC/...` e `/Users/piccoletto/Downloads/...`.
- La shell del Mac ha ffmpeg e python3 ma NON cargo, e ha solo 3 GB di RAM: trascrizione, LaTeX e `cargo test` si fanno nel cloud.
- Scratch sul Mac: `CPC/_routine/tmp/` (ignorata da git). Mai scratch in Lessons/ o in Download.

## Regole
1. Non modificare mai i file scritti da Octech: `notes.md` di ogni lezione, il suo codice (es. `Lessons/L4/src/lib.rs`,
   `naive_sol.py`, ...). Sono FONTI: leggili, spesso contengono cose dette a lezione o sue soluzioni.
   Se una lezione ha già un `src/lib.rs` suo, il tuo codice va in `src/claude.rs` e in `lib.rs` aggiungi solo la riga
   `pub mod claude;` in fondo (unica modifica ammessa ai suoi file).
2. File generati dalla routine (tex/pdf/claude_notes.md/board/figs/transcript, e il codice in claude.rs o nei lib.rs creati da te):
   se `check_new.py` li segnala in `generated_edited_by_user`, Octech li ha modificati a mano: NON sovrascriverli,
   scrivi la nuova versione accanto con suffisso `_new` e dillo nel messaggio finale.
3. Distingui sempre cosa viene dalla lezione e cosa no: tutto ciò che non è stato detto/scritto dal prof va marcato `\extra{}`.
   Il codice del prof (dalle sue note sul blog) va copiato identico, con solo commenti aggiunti, e attribuito.
   Non inventare contenuti: se un punto della trascrizione non è chiaro, dillo nel PDF ("unclear in the recording").
4. Niente git commit, niente push.
5. Lo stato (`check_new.py --segna`) si aggiorna SOLO a lavoro finito e verificato. Se qualcosa fallisce a metà,
   non segnare: la prossima esecuzione riprende.

## Procedura
1. `python3 $HOME/mnt/CPC/_routine/check_new.py` → JSON con `new`, `changed`, `generated_edited_by_user`.
   Se `new` e `changed` sono vuoti: fine, messaggio "Nessun materiale nuovo per CPC".
2. Raggruppa per lezione (campo `lesson`; per i file senza numero deducilo dalla data e dall'elenco lezioni del corso:
   L1 16/09, L2 21/09, L3 23/09, L4 28/09, L5 30/09, poi lun/mer). Una lezione alla volta, in ordine.
3. Registrazioni in Download (es. `L_05.MP4`): spostale in `Lessons/L<n>/` — `cp -n`, confronta `sha1sum`, poi `rm`
   dell'originale. Se `rm` dà "Operation not permitted", chiedi il permesso di eliminare in Download
   (device_request_delete_permission); se non arriva, lascia l'originale e dillo nel messaggio finale.
   Crea `Lessons/L<n>/` se manca. Aggiungi `*.mp4`-simili a .gitignore se non già coperti.
4. Se c'è una registrazione nuova:
   a. sul Mac, in `_routine/tmp/L<n>/`:
      `ffmpeg -v error -y -i REC -vn -ac 1 -ar 16000 -c:a libopus -b:a 24k audio.ogg`
      `mkdir frames && ffmpeg -v error -y -skip_frame nokey -i REC -vf "fps=1/20,scale=1366:-1" -q:v 5 frames/f_%04d.jpg`
      `tar cf frames.tar frames` (ogni comando < 180 s; per video > 2h dividi con -ss/-t).
   b. device_stage_files di audio.ogg e frames.tar → nel cloud.
   c. nel cloud: setup e `transcribe.py` come descritto nel file stesso (copialo dal Mac leggendolo con device_bash `cat`).
      Mentre gira, `sheets.py frames/ sheets/` e guarda TUTTI i fogli con Read: sono le pagine scritte a mano del prof.
   d. leggi TUTTA la trascrizione (non a campione). Termini storpiati tipici: "DAC/deck"=deque, "hip/ape"=heap,
      "B3"=BTree, "Rossano's notes" = blog del prof.
   e. scegli 5-10 frame significativi (algoritmi, dimostrazioni, esempi, figure) e ritagliali in `board/*.jpg`
      (ritaglia via la barra degli strumenti in alto, ~primi 110 px a 1366 di larghezza; controlla ogni ritaglio a occhio).
5. Se NON c'è registrazione ma ci sono note/codice di Octech: prepara comunque una versione provvisoria dagli appunti di
   Octech + pagina del corso + note del prof, con in testa una box rossa "Provvisorio: registrazione non ancora disponibile".
   Quando arriva la registrazione, la lezione va rifatta sulla trascrizione.
6. Scrivi `Lessons/L<n>/L<n>.tex` sul modello di L3: box fonti in testa, indice, sezioni nell'ordine della lezione,
   immagini `\board`, figure tue (matplotlib, script `figs/make_figs.py`, output PDF vettoriale) marcate `\extra`,
   compiti/esercizi assegnati dal prof ben visibili, "cosa viene la prossima volta" se detto.
   Compila con `xelatex` due volte nel cloud, niente errori; guarda le pagine renderizzate (pdftoppm) prima di consegnare.
   Poi `claude_notes.md` = versione markdown del PDF (pandoc da una copia del tex senza preambolo, come fatto per L3)
   e `L<n>_transcript.txt` con una riga d'intestazione che dice che è automatica e non rivista.
7. Codice: crate `Lessons/L<n>` (Cargo.toml con `version.workspace = true`, `edition.workspace = true`) aggiunto ai
   `members` del Cargo.toml radice. Dentro: codice del prof dalle note (se c'è), implementazioni degli algoritmi visti,
   test (esempi della lezione + confronto con forza bruta su input casuali), eventuali binari in `src/bin/`.
   Se Octech ha scritto una sua soluzione, NON toccarla: aggiungi test in claude.rs che la confrontano con la forza bruta
   e riporta nel messaggio finale se trovi un bug (con l'input che lo mostra).
   `cargo test -p l<n>` nel cloud (copia lì il workspace minimo: Cargo.toml radice + le lezioni necessarie). Deve passare.
8. Riporta i file sul Mac con device_commit_files (da /mnt/user-data/outputs/...), poi dal Mac:
   `cd $HOME/mnt/CPC && python3 _routine/check_new.py --segna-generati <file generati>` e infine
   `python3 _routine/check_new.py --segna`. Pulisci `_routine/tmp/L<n>/` (se rm non è permesso, lascia: è ignorata da git).
9. Messaggio finale, in italiano e breve: lezioni sistemate (e se provvisorie), compiti assegnati dal prof, bug trovati nel
   codice di Octech, file lasciati accanto con `_new`, cosa non è stato possibile fare.

## Aggiunte (01/10/2026)
- Trascrizione Teams: se nella cartella della lezione c'è un `.docx` tipo `L_04-en-US.docx` (trascrizione automatica di Teams),
  usalo come trascrizione (estrai il testo da word/document.xml) e salta la trascrizione con sherpa-onnx; servono comunque i frame.
- Note del prof in PDF (esportate dal suo iPad): `Lessons/L3/SlidingWindowMaxima.pdf` (TRW + SWM, 2026) e `Lessons/L5/Pearls_2025.pdf`
  (note dell'ANNO SCORSO sulle "pearls": 100 prigionieri, duplicato, Floyd, majority/Boyer-Moore, Misra-Gries, scacchiera/domino).
  Se arrivano nuovi PDF del prof, spostali (mv, nomi senza spazi) nella lezione a cui si riferiscono e includine le pagine nel tex
  con `\includegraphics[page=N,trim=...,clip]` (vedi i comandi `\profpage`/`\oldnotes` in L3.tex/L4.tex), al posto dei frame del video
  quando coprono lo stesso contenuto. Le note 2025 vanno sempre marcate come "last year" ed `\extra` se non viste a lezione.
- L4 è finita con un compito (sistemare il bug della soluzione "destroy A"): controlla in L5 come l'ha risolto il prof.
- Todoist: per ogni lezione sistemata (non provvisoria) aggiungi un'attività nel progetto "📚 Studio esami", sezione
  "Competitive Programming and Contests" (sectionId 6hg5Fq9xJpmgFX43), etichetta `cpc`, sul modello di "[CPC L4] ...":
  titolo "[CPC L<n>] <argomenti>", descrizione con durata stimata · pagine · data lezione, riga "Apri: file:///Users/piccoletto/Desktop/Everything/pisa/corsi/magistrale/secondo_anno/CPC/Lessons/L<n>/L<n>.pdf",
  riga "Codice: ..." e riga "Fatto quando: ...". Compiti assegnati dal prof: attività separata "[CPC L<n> compito] ..." con priorità p2.
  Prima controlla con find-tasks che l'attività non esista già (non creare doppioni).
- L5 (30/09) PROVVISORIA è la "Part II" di `Lessons/L4/L4.pdf` (Octech vuole L4 e L5 insieme nella cartella L4, con chiaro
  dove si è fermato il prof). Quando arrivano registrazione/trascrizione di L5: scrivi L5 per bene in `Lessons/L5/`
  (PDF, codice, test) e in L4.tex sostituisci la Part II con un rimando breve a `Lessons/L5/L5.pdf`, aggiornando la box "Where we are".
- Esercizi: `CPC/Esercizi/` è un crate di esercizi per Octech (tracce con `todo!()` in src/lN.rs, test, soluzioni in
  `Esercizi/soluzioni/` fuori dal crate, README con elenco e suggerimenti). Per ogni lezione nuova sistemata (non provvisoria)
  aggiungi `src/l<n>.rs` con 2-5 esercizi sugli algoritmi della lezione (traccia, obiettivo di complessità, cosa si allena in Rust,
  test con esempi della lezione + confronto con forza bruta), la soluzione in `soluzioni/l<n>.rs`, la riga `pub mod l<n>;` in lib.rs
  e le righe in README. Verifica nel cloud che i test passino con le soluzioni copiate al posto delle tracce e che le tracce compilino.
  Non toccare mai i file src/lN.rs già esistenti: potrebbero contenere le soluzioni di Octech. Quando L5 sarà rifatta sulla
  registrazione, se gli esercizi E19-E22 non corrispondono alla lezione vera, aggiungine di nuovi invece di modificarli.
- (04/10) Octech ha rinominato `Lessons/L4` in `Lessons/L4_and_L5` e il PDF in `L4_and:L5.pdf` (nel Finder appare come "L4_and/L5.pdf"),
  e ha aggiunto i suoi appunti `notes_L5.md` (fonte, non toccarli). Se ricompili L4.tex, rinomina l'output come il suo PDF;
  in generale rispetta nomi e cartelle che sceglie lui e aggiorna di conseguenza i `members` del Cargo.toml radice.

## Struttura aggiornata (04/10/2026) — vale più delle note precedenti dove sono in conflitto
- Una cartella per lezione `Lessons/L<n>/` con: `L<n>.tex/.pdf`, `claude_notes.md`, `board/`, `figs/`, il crate della lezione
  (`Cargo.toml` + `src/`, nome `l<n>`), le note e il codice di Octech, e la sottocartella **`practice/`** con gli esercizi.
- L4 e L5 sono di nuovo separate: `Lessons/L4` (L4 vera) e `Lessons/L5` (L5 PROVVISORIA dalle note 2025; `Pearls_2025.pdf` è in L5;
  `Lessons/L5/notes.md` sono gli appunti di Octech su L5, ex notes_L5.md). Quando arriva la registrazione di L5, rifai `Lessons/L5`
  sulla registrazione (togli "provisional"), e aggiorna il rimando in fondo a L4.tex se serve.
- Esercizi: NON c'è più `CPC/Esercizi`. Per ogni lezione `Lessons/L<n>/practice/` è un crate `practice_l<n>`:
  `src/lib.rs` (tracce con `todo!()` + test), `src/util.rs` (Rng), `soluzioni/lib.rs` (fuori dal crate), `README.md` (tabella + suggerimenti).
  Per una lezione nuova crea `practice/` così (2-5 esercizi), aggiungi `"Lessons/L<n>/practice"` e `"Lessons/L<n>"` ai members del
  Cargo.toml radice, verifica nel cloud che le soluzioni passino i test e che le tracce compilino senza warning.
  Non sovrascrivere mai un `practice/src/lib.rs` esistente: può contenere le soluzioni di Octech.

## Stato al 05/10/2026
- L5 è stata rifatta sulla registrazione (non è più provvisoria). Il prof è arrivato fino a Boyer-Moore (inserimenti sì,
  cancellazioni annunciate per L6).
- L6 (lunedì 05/10) NON è ancora integrata: in `Lessons/L6/` ci sono già `notes.md` di Octech (sezione [L5] ripasso + [L6]),
  `Binary_search_2025.pdf` (note 2025 del prof: puzzle scacchiera/Alice&Bob, binary search for the answer, exponential search,
  two pointers, sweep line, closest pair, frogs and mosquitoes), e un codice/practice PROVVISORI su maggioranza con
  cancellazioni e Misra-Gries (`src/lib.rs`, `practice/` E22-E23, dalle note 2025). Quando arriva la registrazione di L6:
  scrivi L6.tex/pdf sulla registrazione partendo da dove finisce L5, includi le pagine di `Pearls_2025.pdf` (in Lessons/L5) e di
  `Binary_search_2025.pdf` per ciò che è stato fatto, sistema `src/lib.rs` (generato da te) e AGGIUNGI esercizi in practice
  (non toccare E22-E23 se Octech li ha modificati: controlla generated_edited_by_user / confronta con soluzioni).
- Gli esercizi ora sono numerati: L5 = E19-E21, L6 = E22-E23 (+ nuovi da E24).
- (05/10) Creato `Lessons/L6/L6.pdf` PROVVISORIO (da notes.md di Octech + Pearls_2025 p.14-17): maggioranza con
  inserimenti/cancellazioni (contatori per bit), Misra-Gries, puzzle scacchiera. Quando arriva la registrazione di L6,
  rifallo sulla registrazione mantenendo questo inizio e aggiungendo ciò che segue (probabilmente Binary_search_2025.pdf).
- (05/10) L6.pdf provvisorio ora include anche puzzle (scacchiera, Alice & Bob) e binary search (Binary_search_2025.pdf p.1-4 +
  blog del prof): ricerca binaria, binary search for the answer (sqrt, social distancing), exponential search, problema
  dell'auto/carburante (solo enunciato). practice L6 = E22-E27. Trovato un bug nel codice del blog (select_intervals: limite
  superiore l = numero di punti invece dell'ampiezza), corretto in L6/src/lib.rs. Le pagine 5-13 (two pointers, sweep line,
  closest pair, frogs) NON sono in L6: vanno nella lezione in cui il prof le fa.
- (05/10, 13:30) L6 provvisorio riallineato agli appunti presi da Octech DURANTE la lezione (notes.md 12:57): fatti a lezione, in ordine,
  maggioranza con cancellazioni → Misra-Gries → scacchiera → auto/carburante (enunciato completo: rifornimento f_i in ogni punto) →
  binary search for the answer → social distancing (le note si fermano qui). Alice & Bob, codice binary search/sqrt, exponential search
  NON sono nelle note: marcati "[not in your notes]". Aggiunti l6::min_initial_fuel (BS) e min_initial_fuel_linear [extra] con test.
  Ancora nessuna registrazione L6 in Download.
- (09/10) Creato `Lessons/L7/` PROVVISORIO (registrazione non ancora uscita): dalla pagina del corso (07/10 = "Two pointers
  trick and BST", obbligatori Longest k-Good Segment e Frogs and Mosquitoes) + Binary_search_2025.pdf p.5 e p.10-13.
  Codice l7 + practice E28-E30. Quando arriva la registrazione di L7, rifallo su quella. La pagina del corso indica L8 (12/10)
  = Sweep Line (blog 2023/sweepline, note p.6-9: max overlapping intervals, closest pair of points).
- Il problema dell'auto/carburante in L6 è chiarito dalle note di Octech (L6/notes.md): a ogni punto si rifornisce di f_i litri,
  G* = carburante minimo iniziale; da integrare quando L6 viene rifatta sulla registrazione.
- (09/10) Evidenziazione nei PDF (richiesta di Octech, provata su L7.pdf): legenda in testa e quattro colori con il pacchetto
  `soul` -> \hly{} giallo = generale/da ricordare, \hlg{} verde = cose buone (funziona, efficiente, corretto),
  \hlp{} viola = cose cattive (errori, trappole, costi), \hlb{} blu = definizioni e risultati chiave. Macro e legenda:
  copia il preambolo e la riga "Highlights" da Lessons/L7/L7.tex. Dentro \hl* la matematica va in \mbox{$...$}.
  Nel cloud soul.sty non c'è: curl -sLo /tmp/soul.zip https://mirrors.ctan.org/macros/generic/soul.zip, unzip,
  `latex soul.ins`, copia i .sty in ~/texmf/tex/latex/soul/. Usala per le lezioni nuove (frasi brevi, senza esagerare).
- (12/10) L7 rifatta sulla registrazione (Lessons/L7/L_07.MP4, trascrizione L7_transcript.txt). Contenuto REALE del 07/10:
  fine binary search for the answer (social distancing O((n+c) log l), d* <= l/c; auto/carburante con BS) -> puzzle scacchiera
  mutilata (parità) + follow-up aperto "togli 1 nera e 1 bianca: tiling sempre possibile? algoritmo?" (soluzione in L8) ->
  two pointers (longest k-good, palindromo, alternativa BS for the answer Θ(n log n); l'esempio del prof dà 5 ma la risposta
  giusta è 6) -> SWEEP LINE: max intervalli sovrapposti (contatori -> eventi ordinati Θ(n log n) -> parentesi bilanciate).
  Frogs and Mosquitoes NON spiegato: resta come appendice "preview" in L7.pdf (e E30). Il prof annuncia per L8: closest pair
  of points (sweep line) + "un problema con molti BST" (probabilmente Frogs). In L8 quindi NON rifare max overlapping intervals:
  parti da closest pair, e se fa Frogs integra/sposta l'appendice di L7 in L8. practice L7 = E28-E34.
- (12/10) Hands-On 1 uscito: scaffold in CPC/HandsOn/HO1 (crate handson1, membro del workspace) con SOLO il codice del prof.
  Octech lo risolve A MANO; la correzione con Claude solo quando lo chiede. NON scrivere soluzioni/suggerimenti, NON toccare
  HandsOn/ nelle routine (solo segnalare se cambia), NON pubblicarlo (pubblica.json): condividere codice è vietato.
- (12/10) Su richiesta di Octech: L7/notes.md (sue note) riordinate e integrate con tutta la lezione 07/10; la parte su
  Frogs and Mosquitoes spostata in Lessons/L8/notes.md (creata, solo notes.md per ora). Backup dell'originale in
  _routine/tmp/L7/notes_backup_2026-10-12.md. Quando arriva L8: crea L8.tex/pdf, sposta l'appendice Frogs da L7.pdf
  (e frogs_and_mosquitoes / E30 da L7 se Octech lo vuole) e integra L8/notes.md.
- (12/10) L8 fatta dalla registrazione (Lessons/L8/L_08.MP4): soluzione puzzle tiling (catena/ciclo, Gomory), puzzle Alice & Bob
  (pari/dispari, albero di gioco -> DP), closest pair of points (sweep line per x + BST per y con cancellazione dei punti a
  sinistra della striscia). Crate l8 + practice E35-E38 (workspace aggiornato). Frogs and Mosquitoes ANCORA non spiegato
  ("siamo 1 ora indietro"): resta in L7 (appendice, E30) e in fondo a L8/notes.md; quando lo fa, mettilo nella lezione giusta.
  L8/notes.md (di Octech) integrato su richiesta; backup in _routine/tmp/L8/notes_backup_2026-10-12.md.
- (12/10) Su richiesta di Octech: Frogs and Mosquitoes spostato in Lessons/L9 (PROVVISORIA): L9.tex/pdf (dalle note 2025),
  crate l9 (frogs_and_mosquitoes + test), practice_l9 E39 (ex E30, tolto da L7), notes.md (la sua sezione, tolta da L8/notes.md).
  L7.pdf senza più appendice. Quando arriva la registrazione L9: rifai L9 sulla registrazione (Frogs se lo fa, più il resto).
  Download: cancellato L_08.MP4 (copia in Lessons/L8). L_07.MP4 NON cancellato: Lessons/L7/L_07.MP4 non esiste più (rimosso da
  Octech), quindi quella in Download è l'unica copia -> chiedere prima di cancellarla.
