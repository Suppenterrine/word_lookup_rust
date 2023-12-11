# README für ConceptNet Query Tool in Rust
![app logo](media/wod.png)

## 🚀 Zu gut um wahr zu sein... WOD?
Abgesehen das der Wortwitz in der Überschrift kaum funktioniert heißen WIR dich willkommen beim ConceptNet Query Tool, dem kleinen Helfer, der dir zeigt, dass nicht nur Philosophen endlose Fragen über Worte und ihre Bedeutungen stellen können. Wir (das Kollektiv der Kontraminösen Kerngedanken) nutzen die Macht von Rust und ConceptNet, um dir zu zeigen, wie tief der Wort-Kaninchenbau wirklich geht. Also leinen los, wir gehen auf eine Wort-Entdeckungsreise!

## 🛠 Installation
Bevor du loslegst, stelle sicher, dass du Rust installiert hast. Wenn nicht, besuche die offizielle Rust-Website für Installationsanweisungen. Dann klonst du dieses Repository und führst es aus. Easy peasy, lemon squeezy!

<details>
    <summary>Noch detailliertere Installationsanweisungen 👀</summary>

## 🌟 Rust Installation

### Schritt 1: Rust Installieren
Rust ist eine moderne Programmiersprache, die für ihre Sicherheit und Schnelligkeit bekannt ist. Um unser Tool nutzen zu können, musst du zuerst Rust auf deinem Computer installieren. Keine Sorge, es ist einfacher als ein IKEA-Regal aufzubauen!

#### Für Windows-Nutzer:
1. Gehe zu [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) und lade das Installationsprogramm herunter.
2. Führe das heruntergeladene `.exe`-Installationsprogramm aus und folge den Anweisungen auf dem Bildschirm.
3. Sobald die Installation abgeschlossen ist, starte deinen Computer neu, um sicherzustellen, dass alle Änderungen übernommen werden.

### Schritt 2: Überprüfen der Installation
Um zu überprüfen, ob Rust erfolgreich installiert wurde, öffne ein Terminal und gib ein:
```powershell
rustc --version
```

Wenn alles geklappt hat, solltest du die Version von Rust sehen, die gerade installiert wurde.

## 🚀 Das Tool Global Verfügbar Machen

### Schritt 1: Kompilieren des Programms
1. Navigiere im Terminal zu dem Verzeichnis, in dem du das `ConceptNet Query Tool` gespeichert hast.
2. Führe den Befehl `cargo build --release` aus. Cargo, der Rust-Paketmanager, wird das Programm kompilieren.
3. Nach dem Kompilieren findest du die ausführbare Datei im `target/release`-Verzeichnis innerhalb deines Projektordners.

### Schritt 2: Hinzufügen zum Systempfad
Damit du das Programm von überall ausführen kannst, musst du den Pfad zur ausführbaren Datei deinem Systempfad hinzufügen.

#### Für Windows-Nutzer:
1. Suche nach "Umgebungsvariablen bearbeiten" in der Windows-Suche und öffne es.
2. Wähle "Umgebungsvariablen" und dann unter "Systemvariablen" die Variable "Path".
3. Klicke auf "Bearbeiten" und dann "Neu", füge den Pfad zum `target/release`-Verzeichnis hinzu.
4. Bestätige mit "OK" und starte deinen Computer neu.


### Schritt 3: Überprüfen
Nachdem du deinen Computer neu gestartet oder dein Terminal neu geladen hast, kannst du überprüfen, ob das Tool korrekt installiert wurde, indem du einfach `word_lookup` (oder den Namen deiner ausführbaren Datei) in dein Terminal eingibst.

---

Glückwunsch! Jetzt kannst du das `ConceptNet Query Tool` von überall aus in deinem Terminal ausführen und die Welt der Worte erkunden! 🎉

</details>

## 📜 Wie man ES benutzt
UNSER Tool ist so einfach zu benutzen wie das Finden eines Kaffeehauses in Berlin. Führe einfach das Programm mit einigen Argumenten aus, und voilà, Wörter und ihre Beziehungen werden dir offenbart!

## Kommandozeilen-Argumente
- `--language` (kurz `-l`): Der Sprachcode, wie 'de' für Deutsch.
- `--word` (kurz `-w`): Das Wort, das du erforschen willst.
- `--relation` (kurz `-r`): Die Art der Beziehung zwischen den Wörtern. Optionen sind:
  - typen: Für 'Is A' Beziehungen.
  - ableitung: Für 'Derived From' Beziehungen.
  - synonym: Na, für Synonyme halt.
  - form: Für 'Form Of' Beziehungen.
  - unterschied: Für 'Distinct From' Beziehungen.
  - antonym: Für Gegensätze.
- `--limit_api` (kurz `-a`): Wie viele Ergebnisse du von der API willst.
- `--limit` (kurz `-m`): Wie viele Wörter am Ende ausgegeben werden sollen.


### Beispiel
Führe das Tool aus und lass dich von der Magie der Worte verzaubern:

```powershell
cargo run -- --language de --word Glück --relation synonym --limit_api 1000 --limit 10
```

Du wirst nie wieder etwas anderes benutzen wollen. Und wenn dir doch dieser Gedanken kommt: Hinterfrage ernsthaft deine Loyalität!

---
***Dieses ReadMe ist maschinell erstellt. Nix Garantie. 🔆***