/// Zentrale Fehlerdefinition für die Anwendung.
///
/// Durch die Verwendung eines einheitlichen `AppError`-Typs können Funktionen
/// konsistent `Result<…, AppError>` zurückgeben und `?`-Operatoren nutzen.
use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum AppError {
    /// I/O‑Fehler, z. B. beim Lesen/Schreiben von Dateien.
    Io(io::Error),
    /// Fehler beim Parsen von Ganzzahlen.
    ParseInt(ParseIntError),
    /// Fehler beim Konvertieren von Bytes in String.
    FromUtf8(FromUtf8Error),
    /// Fehler aus Ollama‑Client‑Operationen.
    Ollama(String),
    /// Allgemeiner Fehler mit freiem Text.
    Message(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "I/O-Fehler: {}", e),
            AppError::ParseInt(e) => write!(f, "Parse-Fehler: {}", e),
            AppError::FromUtf8(e) => write!(f, "UTF‑8-Konvertierungsfehler: {}", e),
            AppError::Ollama(msg) => write!(f, "Ollama-Fehler: {}", msg),
            AppError::Message(msg) => write!(f, "Fehler: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Konvertierungen für gängige Fehlerarten
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self { AppError::Io(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::ParseInt(e) }
}
impl From<FromUtf8Error> for AppError {
    fn from(e: FromUtf8Error) -> Self { AppError::FromUtf8(e) }
}

// Hilfsfunktion für benutzerdefinierte Nachrichten
pub fn msg<S: Into<String>>(s: S) -> AppError {
    AppError::Message(s.into())
}
