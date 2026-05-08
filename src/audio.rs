use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::io::Cursor;

/// Esdeveniments de so per a l'aplicació Pomodoro.
pub enum SoundEvent {
    WorkEnd,
    ShortBreakEnd,
    LongBreakEnd,
}

/// Gestor d'àudio que s'encarrega de reproduir els sons encastats.
pub struct AudioManager {
    // Mantenim el stream viu perquè la reproducció funcioni
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
}

impl AudioManager {
    /// Inicialitza el gestor d'àudio. Si falla en obrir el dispositiu d'àudio,
    /// s'inicialitza com a None i l'aplicació continua en silenci.
    pub fn new() -> Self {
        match OutputStream::try_default() {
            Ok((stream, handle)) => Self {
                _stream: Some(stream),
                handle: Some(handle),
            },
            Err(_) => {
                // Si no hi ha dispositiu d'àudio disponible, continuem en silenci
                Self {
                    _stream: None,
                    handle: None,
                }
            }
        }
    }

    /// Reprodueix el so corresponent a l'esdeveniment al volum indicat.
    /// No fa res si el dispositiu no està disponible o si el volum és 0.0.
    pub fn play(&self, event: SoundEvent, volume: f32) {
        // Si el volum és 0.0, no fem res (considerat com a desactivat)
        if volume <= 0.0 {
            return;
        }

        if let Some(handle) = &self.handle {
            let bytes: &'static [u8] = match event {
                SoundEvent::WorkEnd => include_bytes!("../assets/work_end.wav"),
                SoundEvent::ShortBreakEnd => include_bytes!("../assets/short_break_end.wav"),
                SoundEvent::LongBreakEnd => include_bytes!("../assets/long_break_end.wav"),
            };

            let cursor = Cursor::new(bytes);
            
            // Si el fitxer no és un WAV vàlid (com els marcadors de posició), 
            // el Decoder fallarà i no farà res (evitant el pànic).
            if let Ok(decoder) = Decoder::new(cursor) {
                if let Ok(sink) = Sink::try_new(handle) {
                    sink.set_volume(volume);
                    sink.append(decoder);
                    sink.detach(); // Reprodueix en segon pla sense bloquejar
                }
            }
        }
    }
}
