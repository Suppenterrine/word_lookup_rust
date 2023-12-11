# README für ConceptNet Query Tool in Rust
![app logo](media/wod.png)

## 🚀 Zu gut um wahr zu sein... WOD?
Abgesehen das der Wortwitz in der Überschrift kaum funktioniert heißen WIR dich willkommen beim ConceptNet Query Tool, dem kleinen Helfer, der dir zeigt, dass nicht nur Philosophen endlose Fragen über Worte und ihre Bedeutungen stellen können. Wir (das Kollektiv der Kontraminösen Kerngedanken) nutzen die Macht von Rust und ConceptNet, um dir zu zeigen, wie tief der Wort-Kaninchenbau wirklich geht. Also leinen los, wir gehen auf eine Wort-Entdeckungsreise!

## 🛠 Installation
Bevor du loslegst, stelle sicher, dass du Rust installiert hast. Wenn nicht, besuche die offizielle Rust-Website für Installationsanweisungen. Dann klonst du dieses Repository und führst es aus. Easy peasy, lemon squeezy!

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
