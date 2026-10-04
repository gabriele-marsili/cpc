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
- Note del prof in PDF (esportate dal suo iPad): `Lessons/L3/SlidingWindowMaxima.pdf` (TRW + SWM, 2026) e `Lessons/L4/Pearls_2025.pdf`
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
