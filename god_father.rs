use crate::music::*;

pub const TEMPO: u16 = 80;

// The Godfather Theme
pub const MELODY: [(f64, i16); 129] = [
    (REST, 4),
    (REST, 8),
    (REST, 8),
    (REST, 8),
    (NOTE_E4, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_A4, 8),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_F4, 8),
    (NOTE_G4, 8),
    (NOTE_E4, 2),
    (NOTE_E4, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_A4, 8),
    (NOTE_E4, 8),
    (NOTE_DS4, 8),
    (NOTE_D4, 2),
    (NOTE_D4, 8),
    (NOTE_F4, 8),
    (NOTE_GS4, 8),
    (NOTE_B4, 2),
    (NOTE_D4, 8),
    (NOTE_F4, 8),
    (NOTE_GS4, 8),
    (NOTE_A4, 2),
    (NOTE_C4, 8),
    (NOTE_C4, 8),
    (NOTE_G4, 8),
    (NOTE_F4, 8),
    (NOTE_E4, 8),
    (NOTE_G4, 8),
    (NOTE_F4, 8),
    (NOTE_F4, 8),
    (NOTE_E4, 8),
    (NOTE_E4, 8),
    (NOTE_GS4, 8),
    (NOTE_A4, 2),
    (REST, 8),
    (NOTE_A4, 8),
    (NOTE_A4, 8),
    (NOTE_GS4, 8),
    (NOTE_G4, 2),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_F4, 8),
    (NOTE_E4, 2),
    (NOTE_E4, 8),
    (NOTE_G4, 8),
    (NOTE_E4, 8),
    (NOTE_D4, 2),
    (NOTE_D4, 8),
    (NOTE_D4, 8),
    (NOTE_F4, 8),
    (NOTE_DS4, 8),
    (NOTE_E4, 2),
    (REST, 8),
    (NOTE_E4, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_A4, 8),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_F4, 8),
    (NOTE_G4, 8),
    (NOTE_E4, 2),
    (NOTE_E4, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_A4, 8),
    (NOTE_C5, 8),
    (NOTE_A4, 8),
    (NOTE_E4, 8),
    (NOTE_DS4, 8),
    (NOTE_D4, 2),
    (NOTE_D4, 8),
    (NOTE_F4, 8),
    (NOTE_GS4, 8),
    (NOTE_B4, 2),
    (NOTE_D4, 8),
    (NOTE_F4, 8),
    (NOTE_GS4, 8),
    (NOTE_A4, 2),
    (NOTE_C4, 8),
    (NOTE_C4, 8),
    (NOTE_G4, 8),
    (NOTE_F4, 8),
    (NOTE_E4, 8),
    (NOTE_G4, 8),
    (NOTE_F4, 8),
    (NOTE_F4, 8),
    (NOTE_E4, 8),
    (NOTE_E4, 8),
    (NOTE_GS4, 8),
    (NOTE_A4, 2),
    (REST, 8),
    (NOTE_A4, 8),
    (NOTE_A4, 8),
    (NOTE_GS4, 8),
    (NOTE_G4, 2),
    (NOTE_B4, 8),
    (NOTE_A4, 8),
    (NOTE_F4, 8),
    (NOTE_E4, 2),
    (NOTE_E4, 8),
    (NOTE_G4, 8),
    (NOTE_E4, 8),
    (NOTE_D4, 2),
    (NOTE_D4, 8),
    (NOTE_D4, 8),
    (NOTE_F4, 8),
    (NOTE_DS4, 8),
    (NOTE_E4, 2),
];