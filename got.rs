use crate::music::*;

/// Tempo and pin configuration
pub const TEMPO: u16 = 85;

/// Game of Thrones Theme
pub const MELODY: [(f64, i16); 92] = [
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_E4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_E4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_E4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 8),
    (NOTE_C4, 8),
    (NOTE_E4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, -4),
    (NOTE_C4, -4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 4),
    (NOTE_C4, 4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_D4, -1),
    (NOTE_F4, -4),
    (NOTE_AS3, -4),
    (NOTE_DS4, 16),
    (NOTE_D4, 16),
    (NOTE_F4, 4),
    (NOTE_AS3, -4),
    (NOTE_DS4, 16),
    (NOTE_D4, 16),
    (NOTE_C4, -1),
    // Repeat
    (NOTE_G4, -4),
    (NOTE_C4, -4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 4),
    (NOTE_C4, 4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_D4, -1),
    (NOTE_F4, -4),
    (NOTE_AS3, -4),
    (NOTE_DS4, 16),
    (NOTE_D4, 16),
    (NOTE_F4, 4),
    (NOTE_AS3, -4),
    (NOTE_DS4, 16),
    (NOTE_D4, 16),
    (NOTE_C4, -1),
    (NOTE_G4, -4),
    (NOTE_C4, -4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_G4, 4),
    (NOTE_C4, 4),
    (NOTE_DS4, 16),
    (NOTE_F4, 16),
    (NOTE_D4, -2),
    (NOTE_F4, -4),
    (NOTE_AS3, -4),
    (NOTE_D4, -8),
    (NOTE_DS4, -8),
    (NOTE_D4, -8),
    (NOTE_AS3, -8),
    (NOTE_C4, -1),
    (NOTE_C5, -2),
    (NOTE_AS4, -2),
    (NOTE_C4, -2),
    (NOTE_G4, -2),
    (NOTE_DS4, -2),
    (NOTE_DS4, -4),
    (NOTE_F4, -4),
    (NOTE_G4, -1),
];
