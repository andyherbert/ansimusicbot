use super::{frequencies::FREQUENCIES, music::*};
use rand::prelude::*;

#[derive(Debug)]
pub struct Player<T: PlayerPassThrough> {
    tempo: usize,
    length: usize,
    octave: usize,
    articulation: Articulation,
    rng: StdRng,
    pass_through: T,
}

pub trait PlayerPassThrough {
    fn play_frequency(&mut self, frequency: f32, volume: f32, ms: usize);
    fn pause(&mut self, ms: usize);
    fn wav_16_bytes(&mut self) -> Vec<u8>;
    fn wav_8_bytes(&mut self) -> Vec<u8>;
}

impl<T: PlayerPassThrough> Player<T> {
    pub fn new(pass_through: T) -> Player<T> {
        Player {
            tempo: 120,
            length: 1,
            octave: 4,
            articulation: Articulation::Normal,
            rng: StdRng::seed_from_u64(random()),
            pass_through,
        }
    }

    fn get_frequency(&self, note: Note, sign: NoteSign) -> Option<&f32> {
        let mut index = self.octave * 12;
        index += match note {
            Note::A => 9,
            Note::B => 11,
            Note::C => 0,
            Note::D => 2,
            Note::E => 4,
            Note::F => 5,
            Note::G => 7,
        };
        match sign {
            NoteSign::Sharp => FREQUENCIES.get(index + 1),
            NoteSign::Natural => FREQUENCIES.get(index),
            NoteSign::Flat => FREQUENCIES.get(index - 1),
        }
    }

    fn calculate_length(&self, length: Option<usize>, dots: usize) -> (usize, usize) {
        let length = length.unwrap_or(self.length);
        let full_note = 60.0 * 1000.0 / self.tempo as f32 * 4.0 / length as f32;
        let mut note_length = full_note;
        match self.articulation {
            Articulation::Legato => note_length *= 7.0 / 8.0,
            Articulation::Normal => note_length *= 3.0 / 4.0,
            Articulation::Stacato => {}
        }
        let mut extra = 0.0;
        for dot in 0..dots {
            extra += match dot {
                0 => note_length * 1.0 / 2.0,
                _ => 1.0 / 2.0,
            }
        }
        (
            (note_length + extra).ceil() as usize,
            (full_note - note_length).ceil() as usize,
        )
    }

    fn play_sound_code(&mut self, info: SoundCodeInfo) {
        if let (Some(mut frequency), Some(duration)) = (info.frequency, info.duration) {
            let play_ms = (duration / 18.2 * 1000.0).ceil() as usize;
            let pause_ms = info.delay.unwrap_or(0);
            let cycles = info.cycles.unwrap_or(1);
            if cycles == 0 {
                self.pass_through.play_frequency(frequency, 0.2, play_ms);
                self.pass_through.pause(pause_ms);
            } else {
                for _ in 0..cycles {
                    self.pass_through.play_frequency(frequency, 0.2, play_ms);
                    frequency += match info.variation {
                        Some(Variation::Value(value)) => value,
                        Some(Variation::Random) => self.rng.gen_range(-512.0..=512.0),
                        None => 0.0,
                    };
                }
                self.pass_through.pause(pause_ms);
            }
        }
    }

    fn pause(&mut self, quarter_notes: usize) {
        let pause_ms = 60.0 * 1000.0 / self.tempo as f32 * 4.0 / quarter_notes as f32;
        self.pass_through.pause(pause_ms as usize);
    }

    fn play_note(&mut self, note: Note, info: NoteInfo) {
        let (play_ms, pause_ms) = self.calculate_length(info.length, info.dots);
        if let Some(frequency) = self.get_frequency(note, info.sign) {
            self.pass_through.play_frequency(*frequency, 0.2, play_ms);
            self.pass_through.pause(pause_ms);
        }
    }

    fn play_raw_note(&mut self, value: usize) {
        let (play_ms, pause_ms) = self.calculate_length(None, 0);
        if let Some(frequency) = FREQUENCIES.get(value) {
            self.pass_through.play_frequency(*frequency, 0.2, play_ms);
            self.pass_through.pause(pause_ms);
        }
    }

    /// Plays an atomic element of music through the supplied [Sink] and blocks the current thread.
    pub fn play_entity(&mut self, entity: MusicEntity) {
        match entity {
            MusicEntity::Operation(MusicOperation::Articulation(articulation)) => {
                self.articulation = articulation;
            }
            MusicEntity::Operation(_operation) => {}
            MusicEntity::Tempo(value) => self.tempo = value,
            MusicEntity::Octave(value) => self.octave = value,
            MusicEntity::Length(value) => self.length = value,
            MusicEntity::RawNote(value) => self.play_raw_note(value),
            MusicEntity::Pause(value) => self.pause(value),
            MusicEntity::IncreaseOctave => self.octave += 1,
            MusicEntity::DecreaseOctave => self.octave -= 1,
            MusicEntity::Note { note, info } => self.play_note(note, info),
            MusicEntity::SoundCode(info) => self.play_sound_code(info),
        }
    }

    /// Plays [Music] through the supplied [Sink] and blocks the current thread.
    pub fn play(&mut self, music: &Music) {
        for entity in music {
            self.play_entity(entity);
        }
    }

    #[allow(dead_code)]
    pub fn wav_16_bytes(&mut self) -> Vec<u8> {
        self.pass_through.wav_16_bytes()
    }

    #[allow(dead_code)]
    pub fn wav_8_bytes(&mut self) -> Vec<u8> {
        self.pass_through.wav_8_bytes()
    }
}
