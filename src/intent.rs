use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Intent {
    Greeting,
    Weather,
    Search,
    FileOpen,      // Neue Intents
    FileShow,
    Goodbye,       // Fenster schließen, aber weiter laufen
    Shutdown,      // Komplett beenden
    Unknown,
}