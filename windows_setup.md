# Windows Setup-Anleitung

## 🪟 Installation auf Windows

### 1. Rust installieren

Lade den Rust-Installer herunter:
```
https://rustup.rs/
```

Oder verwende diesen direkten Link:
```
https://win.rustup.rs/x86_64
```

Nach der Installation, öffne **PowerShell** oder **CMD** und überprüfe:
```powershell
rustc --version
cargo --version
```

### 2. Projekt kompilieren

```powershell
cd pfad\zu\deinem\projekt
cargo build --release
```

Die fertige .exe findest du dann hier:
```
target\release\ki-assistent.exe
```

### 3. Programm starten

```powershell
cargo run --release
```

Oder direkt die .exe ausführen:
```powershell
.\target\release\ki-assistent.exe
```

## 🎯 Windows-spezifische Features

### Programme finden
Die KI durchsucht automatisch diese Ordner nach Programmen:

- `C:\Program Files`
- `C:\Program Files (x86)`
- `%USERPROFILE%\AppData\Local`
- `%USERPROFILE%\AppData\Roaming`
- `C:\ProgramData`
- Alle Ordner im `PATH`

**Beispiele:**
```
> Starte Steam
> Öffne Discord
> Starte Chrome
> Öffne notepad
```

### Dateien finden
Automatische Suche in:

- Desktop (auch OneDrive\Desktop)
- Dokumente
- Downloads
- Bilder
- OneDrive-Ordner
- Aktuelles Projektverzeichnis

**Beispiele:**
```
> Öffne test.txt
> Zeige mir meine Präsentation
> Öffne boost.rs
```

## 🚀 Autostart einrichten (Optional)

### Methode 1: Startup-Ordner

1. Drücke `Win + R`
2. Gib ein: `shell:startup`
3. Erstelle eine Verknüpfung zur `ki-assistent.exe`

### Methode 2: Task Scheduler

1. Öffne "Aufgabenplanung" (Task Scheduler)
2. Erstelle neue Aufgabe:
   - **Trigger**: Bei Anmeldung
   - **Aktion**: Programm starten → `ki-assistent.exe`
   - **Bedingungen**: Im Hintergrund ausführen

## 🎤 Spracherkennung (Zukünftig)

Für Windows-Spracherkennung gibt es mehrere Optionen:

### Option 1: Windows Speech API (SAPI)
```rust
// In Cargo.toml:
[dependencies]
windows = { version = "0.52", features = ["Media_SpeechRecognition"] }
```

### Option 2: Whisper.cpp
Lokale Spracherkennung ohne Cloud:
```
https://github.com/ggerganov/whisper.cpp
```

### Option 3: Azure Speech (Cloud)
```rust
[dependencies]
cognitive_services_speech_sdk_rs = "1.35"
```

## 🔊 Text-to-Speech auf Windows

Windows hat integriertes TTS:

```rust
use std::process::Command;

fn speak(text: &str) {
    let ps_command = format!(
        "Add-Type -AssemblyName System.Speech; \
         $speak = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
         $speak.Speak('{}')",
        text
    );
    
    Command::new("powershell")
        .args(&["-Command", &ps_command])
        .output()
        .ok();
}
```

## 🖥️ GUI-Fenster (Zukünftig)

### Empfohlene Bibliotheken:

**1. egui (einfach)**
```toml
[dependencies]
eframe = "0.25"
egui = "0.25"
```

**2. Tauri (Web-basiert, modern)**
```toml
[dependencies]
tauri = "1.5"
```

**3. iced (nativ, modern)**
```toml
[dependencies]
iced = "0.12"
```

## 🔧 Häufige Windows-Probleme

### Problem: "cargo: command not found"
**Lösung**: Starte PowerShell/CMD neu nach Rust-Installation

### Problem: Programme werden nicht gefunden
**Lösung**: Verwende den vollständigen Namen, z.B. "Steam.exe" statt "Steam"

### Problem: Dateisuche ist langsam
**Lösung**: Reduziere `max_depth` in der `find_file` Funktion von 3 auf 2

### Problem: OneDrive-Dateien nicht gefunden
**Lösung**: Bereits integriert! OneDrive\Desktop und OneDrive\Dokumente werden durchsucht

## 📝 Tipps für Windows-Nutzer

1. **Projekt im Schnellzugriff**: 
   - Rechtsklick auf Projektordner → An Schnellzugriff anheften

2. **Verknüpfung erstellen**:
   ```powershell
   # PowerShell
   $WshShell = New-Object -comObject WScript.Shell
   $Shortcut = $WshShell.CreateShortcut("$Home\Desktop\KI-Assistent.lnk")
   $Shortcut.TargetPath = "C:\Pfad\zu\target\release\ki-assistent.exe"
   $Shortcut.Save()
   ```

3. **Terminal immer als Admin**:
   - Für besseren Zugriff auf System-Ordner

4. **Windows Defender ausschließen**:
   - Füge `target\release` zum Ausschluss hinzu (optional, beschleunigt Builds)

## 🎨 Farbiges Terminal (Optional)

Installiere **Windows Terminal** aus dem Microsoft Store für:
- Bessere Farben
- Tabs
- Unicode-Unterstützung (für ╔══╗ Zeichen)

## 🔐 Sicherheit

Die KI hat Zugriff auf:
- ✅ Deine Dateien (zum Öffnen)
- ✅ Programme starten
- ❌ KEINE Admin-Rechte (außer du startest sie als Admin)
- ❌ KEINE Netzwerkzugriffe (außer Wikipedia & Wetter-API)

**Wichtig**: Die KI speichert gelernte Daten in `learned_samples.json` im Projektordner.

## 🚦 Nächste Schritte

1. ✅ Projekt kompilieren
2. ✅ Grundfunktionen testen
3. 🔄 Autostart einrichten (optional)
4. 🎤 Spracherkennung integrieren (geplant)
5. 🖥️ GUI erstellen (geplant)

---

**Viel Erfolg mit deinem Windows KI-Assistenten! 🚀**
