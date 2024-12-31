use crate::music::*;

pub const TEMPO: u16 = 105;

// Pacman Intro Theme
pub const MELODY: [(f64, i16); 24] = [
    (NOTE_B4, 16),
    (NOTE_B5, 16),
    (NOTE_FS5, 16),
    (NOTE_DS5, 16),
    (NOTE_B5, 32),
    (NOTE_FS5, -16),
    (NOTE_DS5, 8),
    (NOTE_C5, 16),
    (NOTE_C6, 16),
    (NOTE_G6, 16),
    (NOTE_E6, 16),
    (NOTE_C6, 32),
    (NOTE_G6, -16),
    (NOTE_E6, 8),
    (NOTE_B4, 16),
    (NOTE_B5, 16),
    (NOTE_FS5, 16),
    (NOTE_DS5, 16),
    (NOTE_B5, 32),
    (NOTE_FS5, -16),
    (NOTE_DS5, 8),
    (NOTE_DS5, 32),
    (NOTE_E5, 32),
    (NOTE_F5, 32),
];
