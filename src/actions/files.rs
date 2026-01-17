use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;
use crate::parser::extract_file_path;

#[cfg(target_os = "linux")]
use std::os::unix::fs::PermissionsExt;

/// Findet eine Datei/Ordner rekursiv
fn find_file(name: &str, max_depth: usize) -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    let home = env::var("USERPROFILE").ok()?;
    #[cfg(not(target_os = "windows"))]
    let home = env::var("HOME").ok()?;
    
    let home_path = Path::new(&home);
    
    // Erstelle Pfade die länger leben
    let current_dir = env::current_dir().ok()?;
    
    // Windows-spezifische Suchpfade
    #[cfg(target_os = "windows")]
    let desktop = home_path.join("Desktop");
    #[cfg(target_os = "windows")]
    let dokumente = home_path.join("Dokumente");
    #[cfg(target_os = "windows")]
    let documents = home_path.join("Documents");
    #[cfg(target_os = "windows")]
    let downloads = home_path.join("Downloads");
    #[cfg(target_os = "windows")]
    let bilder = home_path.join("Bilder");
    #[cfg(target_os = "windows")]
    let pictures = home_path.join("Pictures");
    #[cfg(target_os = "windows")]
    let onedrive = home_path.join("OneDrive");
    #[cfg(target_os = "windows")]
    let onedrive_desktop = home_path.join("OneDrive\\Desktop");
    #[cfg(target_os = "windows")]
    let onedrive_docs = home_path.join("OneDrive\\Dokumente");
    
    #[cfg(target_os = "windows")]
    let search_dirs = vec![
        &desktop,
        &dokumente,
        &documents,
        &downloads,
        &bilder,
        &pictures,
        &onedrive,
        &onedrive_desktop,
        &onedrive_docs,
        &current_dir,
        home_path,
    ];
    
    // Linux/Mac Suchpfade
    #[cfg(not(target_os = "windows"))]
    let desktop = home_path.join("Desktop");
    #[cfg(not(target_os = "windows"))]
    let documents = home_path.join("Documents");
    #[cfg(not(target_os = "windows"))]
    let downloads = home_path.join("Downloads");
    #[cfg(not(target_os = "windows"))]
    let pictures = home_path.join("Pictures");
    
    #[cfg(not(target_os = "windows"))]
    let search_dirs = vec![
        &desktop,
        &documents,
        &downloads,
        &pictures,
        &current_dir,
        home_path,
    ];
    
    for dir in search_dirs {
        if !dir.exists() {
            continue;
        }
        
        if let Some(found) = search_recursive(dir, name, 0, max_depth) {
            return Some(found);
        }
    }
    
    None
}

fn search_recursive(dir: &Path, name: &str, depth: usize, max_depth: usize) -> Option<PathBuf> {
    if depth > max_depth {
        return None;
    }
    
    let entries = std::fs::read_dir(dir).ok()?;
    
    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_lowercase();
        
        // Exakte Übereinstimmung oder Teilstring
        if file_name == name.to_lowercase() || file_name.contains(&name.to_lowercase()) {
            return Some(path);
        }
        
        // Rekursiv in Unterordner suchen
        if path.is_dir() && depth < max_depth {
            if let Some(found) = search_recursive(&path, name, depth + 1, max_depth) {
                return Some(found);
            }
        }
    }
    
    None
}

/// Sucht nach installierten Programmen (Windows)
#[cfg(target_os = "windows")]
fn find_program(name: &str) -> Option<PathBuf> {
    // Suche in Program Files und weiteren Standard-Ordnern
    let user_profile = env::var("USERPROFILE").ok()?;
    
    // Erstelle alle Pfade mit längerer Lebensdauer
    let local_appdata = format!("{}\\AppData\\Local", user_profile);
    let roaming_appdata = format!("{}\\AppData\\Roaming", user_profile);
    
    let program_dirs = vec![
        Path::new("C:\\Program Files"),
        Path::new("C:\\Program Files (x86)"),
        Path::new(&local_appdata),
        Path::new(&roaming_appdata),
        Path::new("C:\\ProgramData"),
    ];
    
    for base in program_dirs {
        if !base.exists() {
            continue;
        }
        
        if let Some(found) = search_recursive(base, name, 0, 3) {
            // Wenn es ein Ordner ist, suche nach .exe darin
            if found.is_dir() {
                if let Some(exe) = find_exe_in_dir(&found, name) {
                    return Some(exe);
                }
                return Some(found);
            }
            return Some(found);
        }
    }
    
    // Suche auch im PATH
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(';') {
            let dir_path = Path::new(dir);
            if dir_path.exists() {
                if let Some(found) = search_recursive(dir_path, name, 0, 1) {
                    return Some(found);
                }
            }
        }
    }
    
    None
}

#[cfg(target_os = "windows")]
fn find_exe_in_dir(dir: &Path, name: &str) -> Option<PathBuf> {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) == Some("exe") {
                let file_name = p.file_name()?.to_string_lossy().to_lowercase();
                if file_name.contains(&name.to_lowercase()) {
                    return Some(p);
                }
            }
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
fn find_program(_name: &str) -> Option<PathBuf> {
    // Auf Linux/Mac: Programme sind meist im PATH
    None
}

pub fn handle_file_open(query: &str) {
    let file_name = extract_file_path(query);
    
    if file_name.is_empty() {
        println!("Welche Datei soll ich öffnen?");
        return;
    }
    
    println!("Versuche zu öffnen: {}", file_name);
    
    // Zuerst normale Dateisuche
    let path = find_file(&file_name, 3)
        .or_else(|| find_program(&file_name));
    
    match path {
        Some(path) => {
            println!("Gefunden: {}", path.display());
            
            // Öffne die Datei mit dem Standardprogramm
            #[cfg(target_os = "windows")]
            let result = {
                if path.extension().and_then(|s| s.to_str()) == Some("exe") {
                    // Direktes Ausführen von .exe
                    Command::new(&path).spawn()
                } else {
                    // Öffne mit Standardprogramm
                    Command::new("cmd")
                        .args(&["/C", "start", "", &path.to_string_lossy()])
                        .spawn()
                }
            };
            
            #[cfg(target_os = "macos")]
            let result = Command::new("open")
                .arg(&path)
                .spawn();
            
            #[cfg(target_os = "linux")]
            let result = {
                if path.is_file() && path.metadata().ok().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false) {
                    // Ausführbare Datei
                    Command::new(&path).spawn()
                } else {
                    Command::new("xdg-open")
                        .arg(&path)
                        .spawn()
                }
            };
            
            match result {
                Ok(_) => println!("✓ Geöffnet."),
                Err(e) => println!("✗ Fehler beim Öffnen: {}", e),
            }
        }
        None => {
            println!("✗ Datei oder Programm '{}' nicht gefunden.", file_name);
            println!("  Tipp: Versuche den vollständigen Namen oder eine Dateiendung anzugeben.");
        }
    }
}

pub fn handle_file_show(query: &str) {
    let file_name = extract_file_path(query);
    
    if file_name.is_empty() {
        println!("Was soll ich anzeigen?");
        return;
    }
    
    println!("Versuche zu finden: {}", file_name);
    
    // Finde die Datei
    match find_file(&file_name, 3) {
        Some(path) => {
            println!("Gefunden: {}", path.display());
            
            // Öffne den Explorer/Finder an diesem Ort
            let _parent = path.parent().unwrap_or(&path);
            
            #[cfg(target_os = "windows")]
            let result = Command::new("explorer")
                .arg("/select,")
                .arg(&path)
                .spawn();
            
            #[cfg(target_os = "macos")]
            let result = Command::new("open")
                .arg("-R")
                .arg(&path)
                .spawn();
            
            #[cfg(target_os = "linux")]
            let result = Command::new("xdg-open")
                .arg(_parent)
                .spawn();
            
            match result {
                Ok(_) => println!("✓ Im Explorer angezeigt."),
                Err(e) => println!("✗ Fehler beim Anzeigen: {}", e),
            }
        }
        None => {
            println!("✗ Datei '{}' nicht gefunden.", file_name);
            println!("  Tipp: Versuche einen präziseren Dateinamen.");
        }
    }
}