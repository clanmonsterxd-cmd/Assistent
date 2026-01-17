# \# Lokale KI-Assistent

# 

# Ein vollständig lokaler KI-Assistent in Rust, der ohne externe KI-Modelle arbeitet und nur öffentliche APIs für Informationen nutzt.

# 

# \## ✨ Features

# 

# \### Aktuelle Funktionen

# \- \*\*Intent-Erkennung\*\*: Verstehen von Nutzerabsichten durch Vektorähnlichkeit

# \- \*\*Kontinuierliches Lernen\*\*: Speichert neue Beispiele und verbessert sich über Zeit

# \- \*\*Wetter-Abfragen\*\*: 

# &nbsp; - Aktuelle Temperatur und Wetterlage

# &nbsp; - Wettervorhersage

# &nbsp; - Standort-basierte Abfragen (z.B. "Wetter in Berlin")

# \- \*\*Web-Recherche\*\*: 

# &nbsp; - Intelligente Extraktion von Suchbegriffen

# &nbsp; - Wikipedia-Integration

# \- \*\*Datei-Verwaltung\*\*:

# &nbsp; - Dateien öffnen: "Öffne Dokument.pdf"

# &nbsp; - Dateien im Explorer anzeigen: "Zeige mir meine Präsentation"

# \- \*\*Hintergrund-Modus\*\*:

# &nbsp; - Startet minimiert, wartet auf Begrüßung

# &nbsp; - "Tschüss"/"Danke" → Fenster schließen (läuft weiter)

# &nbsp; - "Schönen Tag noch" → Komplett beenden

# 

# \### Geplante Features

# \- 🎤 Spracherkennung (Transkription)

# \- 🔊 Text-to-Speech (Antworten vorlesen)

# \- 🖥️ GUI-Fenster statt Terminal

# \- 🔍 Erweiterte Dateisuche

# 

# \## 📁 Projektstruktur

# 

# ```

# src/

# ├── main.rs              # Hauptlogik, Event-Loop

# ├── intent.rs            # Intent-Definitionen

# ├── classifier.rs        # Intent-Klassifizierung

# ├── vector.rs            # Vektorisierung von Text

# ├── similarity.rs        # Cosine-Similarity

# ├── normalize.rs         # Text-Normalisierung \& Stemming

# ├── boost.rs             # Keyword-Boosting

# ├── learning.rs          # Persistentes Lernen

# ├── parser.rs            # Extraktion von Parametern (NEU)

# └── actions/

# &nbsp;   ├── mod.rs

# &nbsp;   ├── weather.rs       # Wetter-API (erweitert)

# &nbsp;   ├── search.rs        # Wikipedia-Suche (erweitert)

# &nbsp;   └── files.rs         # Dateiverwaltung (NEU)

# ```

# 

# \## 🚀 Installation

# 

# \### Voraussetzungen

# ```bash

# \# Rust installieren

# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 

# \# Projekt klonen

# git clone <dein-repo>

# cd ki-assistent

# ```

# 

# \### Abhängigkeiten (Cargo.toml)

# ```toml

# \[dependencies]

# serde = { version = "1.0", features = \["derive"] }

# serde\_json = "1.0"

# reqwest = { version = "0.11", features = \["blocking", "json"] }

# ```

# 

# \### Kompilieren \& Starten

# ```bash

# cargo build --release

# cargo run --release

# ```

# 

# \## 💬 Verwendung

# 

# \### Beispiel-Konversation

# 

# ```

# KI-Assistent gestartet. Warte auf Begrüßung...

# 

# Hallo

# ╔══════════════════════════════════╗

# ║   KI-Assistent aktiviert!        ║

# ╚══════════════════════════════════╝

# Hallo! Wie kann ich dir helfen?

# 

# > Wie ist das Wetter?

# Aktuell: 5.2 °C, leicht bewölkt

# Heute: 3.1 °C bis 7.8 °C

# 

# > Wetter in Berlin

# Suche Wetter für: Berlin

# Aktuell: 4.5 °C, Regen

# Heute: 2.3 °C bis 6.1 °C

# 

# > Suche nach Rust Programmierung

# Suche nach: rust programmierung

# 

# Rust ist eine Programmiersprache...

# 

# > Öffne test.txt

# Versuche zu öffnen: test.txt

# Gefunden: /home/user/Desktop/test.txt

# Datei geöffnet.

# 

# > Zeige mir meine Präsentation

# Versuche zu finden: meine Präsentation

# Gefunden: /home/user/Dokumente/präsentation.pptx

# Datei im Explorer angezeigt.

# 

# > Danke

# Bis bald! (Fenster wird minimiert, ich laufe weiter im Hintergrund)

# 

# > Schönen Tag noch

# Auf Wiedersehen! Schönen Tag noch!

# KI-Assistent wird beendet.

# ```

# 

# \## 🧠 Technische Details

# 

# \### Intent-Klassifizierung

# 1\. \*\*Vektorisierung\*\*: Text → 32-dimensionaler Vektor (Hashing-basiert)

# 2\. \*\*Similarity\*\*: Cosine-Ähnlichkeit zwischen Input und Beispielen

# 3\. \*\*Boosting\*\*: Keyword-basierte Verstärkung

# 4\. \*\*Gewichtung\*\*: Gelernte Beispiele haben höheres Gewicht (2.0)

# 5\. \*\*Decay\*\*: Alte Beispiele verlieren langsam an Gewicht

# 

# \### Parser-System

# \- \*\*Search\*\*: Entfernt Befehlswörter ("suche", "finde", etc.)

# \- \*\*Weather\*\*: Erkennt Ortsangaben ("in Berlin", "für München")

# \- \*\*Files\*\*: Extrahiert Dateinamen nach Markern

# 

# \### Datei-Suche

# Durchsucht automatisch:

# \- Home-Verzeichnis

# \- Desktop

# \- Dokumente/Documents

# \- Downloads

# 

# \### Wetter-API

# \- \*\*Geocoding\*\*: Open-Meteo Geocoding API

# \- \*\*Wetter\*\*: Open-Meteo Weather API

# \- Kostenlos, keine API-Keys nötig

# 

# \## 🔧 Anpassungen

# 

# \### Neuen Intent hinzufügen

# 

# 1\. \*\*intent.rs\*\*: Neues Enum hinzufügen

# ```rust

# pub enum Intent {

# &nbsp;   // ...

# &nbsp;   MeinIntent,

# }

# ```

# 

# 2\. \*\*boost.rs\*\*: Boosting-Regeln

# ```rust

# Intent::MeinIntent => {

# &nbsp;   if t.contains("keyword") { 1.4 } else { 1.0 }

# }

# ```

# 

# 3\. \*\*main.rs\*\*: Beispiele und Handler

# ```rust

# IntentSample {

# &nbsp;   intent: Intent::MeinIntent,

# &nbsp;   vector: vocab.sentence\_vec("beispiel wörter"),

# &nbsp;   weight: 1.0,

# }

# 

# // Im match:

# Intent::MeinIntent => {

# &nbsp;   // Deine Logik

# }

# ```

# 

# \### Standard-Koordinaten ändern

# In `weather.rs` die Koordinaten anpassen:

# ```rust

# let (lat, lon) = if let Some(loc) = location {

# &nbsp;   // ...

# } else {

# &nbsp;   (DEINE\_LAT, DEINE\_LON) // <-- Hier ändern

# };

# ```

# 

# \## 📝 Lernsystem

# 

# Die KI speichert gelernte Beispiele in `learned\_samples.json`:

# ```json

# {

# &nbsp; "samples": \[

# &nbsp;   {

# &nbsp;     "intent": "Search",

# &nbsp;     "vector": \[0.123, -0.456, ...],

# &nbsp;     "weight": 1.99

# &nbsp;   }

# &nbsp; ]

# }

# ```

# 

# \*\*Gewicht-Decay\*\*: Jede Interaktion reduziert alte Gewichte um 0.5%, Minimum 0.5

# 

# \## 🎯 Nächste Schritte (für dich)

# 

# \### 1. Spracherkennung

# \- \*\*whisper.cpp\*\*: Lokale Spracherkennung

# \- Oder: Cloud-APIs (Google Speech-to-Text)

# 

# \### 2. Text-to-Speech

# \- \*\*espeak-ng\*\*: Linux TTS

# \- \*\*SAPI\*\*: Windows TTS

# \- \*\*say\*\*: macOS TTS

# 

# \### 3. GUI

# \- \*\*egui\*\*: Einfache GUI in Rust

# \- \*\*iced\*\*: Moderne UI

# \- \*\*tauri\*\*: Web-basierte GUI

# 

# \### 4. System-Integration

# \- Tray-Icon (systemtray-rs)

# \- Globale Hotkeys (global-hotkey)

# \- Autostart

# 

# \## 🐛 Bekannte Limitierungen

# 

# \- Dateisuche nur in Standard-Ordnern

# \- Einfaches Stemming (kein NLP)

# \- Keine Kontextverfolgung

# \- Wikipedia nur auf Deutsch

# 

# \## 📄 Lizenz

# 

# MIT - Nutze es wie du willst!

# 

# \## 🤝 Beitragen

# 

# Du willst helfen? Super!

# 1\. Fork das Projekt

# 2\. Erstelle einen Feature-Branch

# 3\. Committe deine Änderungen

# 4\. Push und erstelle einen Pull Request

# 

# ---

# 

# \*\*Viel Spaß mit deinem lokalen KI-Assistenten! 🚀\*\*

