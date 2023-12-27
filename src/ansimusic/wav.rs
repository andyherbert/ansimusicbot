use super::player::PlayerPassThrough;
use std::f32::consts::TAU;
use std::mem::swap;

struct WavHeader {
    chunk_size: u32,
    subchunk1_size: u32,
    audio_format: u16,
    num_channels: u16,
    sample_rate: u32,
    byte_rate: u32,
    block_align: u16,
    bits_per_sample: u16,
    subchunk2_size: u32,
}

impl WavHeader {
    pub fn new_pcm(
        num_channels: usize,
        sample_rate: usize,
        bits_per_sample: usize,
        num_of_samples: usize,
    ) -> WavHeader {
        let subchunk2_size = (num_of_samples * num_channels * bits_per_sample / 8) as u32;
        WavHeader {
            chunk_size: subchunk2_size + 40,
            subchunk1_size: 16,
            audio_format: 1,
            num_channels: num_channels as u16,
            sample_rate: sample_rate as u32,
            byte_rate: (sample_rate * num_channels * bits_per_sample / 8) as u32,
            block_align: (num_channels * bits_per_sample / 8) as u16,
            bits_per_sample: bits_per_sample as u16,
            subchunk2_size,
        }
    }

    pub fn to_bytes(&self) -> [u8; 44] {
        let mut bytes = [0; 44];
        bytes[0..=3].copy_from_slice("RIFF".as_bytes());
        bytes[4..=7].copy_from_slice(&self.chunk_size.to_le_bytes());
        bytes[8..=11].copy_from_slice("WAVE".as_bytes());
        bytes[12..=15].copy_from_slice("fmt ".as_bytes());
        bytes[16..=19].copy_from_slice(&self.subchunk1_size.to_le_bytes());
        bytes[20..=21].copy_from_slice(&self.audio_format.to_le_bytes());
        bytes[22..=23].copy_from_slice(&self.num_channels.to_le_bytes());
        bytes[24..=27].copy_from_slice(&self.sample_rate.to_le_bytes());
        bytes[28..=31].copy_from_slice(&self.byte_rate.to_le_bytes());
        bytes[32..=33].copy_from_slice(&self.block_align.to_le_bytes());
        bytes[34..=35].copy_from_slice(&self.bits_per_sample.to_le_bytes());
        bytes[36..=39].copy_from_slice("data".as_bytes());
        bytes[40..=43].copy_from_slice(&self.subchunk2_size.to_le_bytes());
        bytes
    }
}

pub struct SquareWave {
    value: f32,
    sample_rate: usize,
    samples: Vec<f32>,
}

impl Default for SquareWave {
    fn default() -> SquareWave {
        SquareWave {
            value: 0.0,
            sample_rate: 44100,
            samples: Vec::new(),
        }
    }
}

impl SquareWave {
    pub fn new(sample_rate: usize) -> SquareWave {
        SquareWave {
            sample_rate,
            ..Default::default()
        }
    }

    fn sample(&mut self, frequency: f32, volume: f32) {
        self.value += (TAU / self.sample_rate as f32) * frequency;
        if self.value >= TAU {
            self.value -= TAU;
        }
        if self.value.sin() > 0.0 {
            self.samples.push(1.0 * volume);
        } else {
            self.samples.push(-1.0 * volume);
        }
    }

    #[allow(dead_code)]
    fn export_8_bits(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(44 + self.samples.len() * 2);
        let header = WavHeader::new_pcm(1, self.sample_rate, 8, self.samples.len());
        bytes.append(&mut header.to_bytes().to_vec());
        for sample in &self.samples {
            let value = if *sample < 0.0 {
                (sample.abs() * i8::MIN as f32) as i8
            } else {
                (sample * i8::MAX as f32) as i8
            };
            bytes.append(&mut value.to_le_bytes().to_vec());
        }
        bytes
    }
}

impl PlayerPassThrough for SquareWave {
    fn play_frequency(&mut self, frequency: f32, volume: f32, ms: usize) {
        for _ in 0..(self.sample_rate / 1000) * ms {
            self.sample(frequency, volume);
        }
    }

    fn pause(&mut self, ms: usize) {
        for _ in 0..(self.sample_rate / 1000) * ms {
            self.sample(0.0, 0.0);
        }
    }

    fn wav_16_bytes(&mut self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(44 + self.samples.len() * 2);
        let header = WavHeader::new_pcm(1, self.sample_rate, 16, self.samples.len());
        bytes.append(&mut header.to_bytes().to_vec());
        let mut samples = Vec::new();
        swap(&mut self.samples, &mut samples);
        for sample in samples.into_iter() {
            let value = if sample < 0.0 {
                (sample.abs() * i16::MIN as f32) as i16
            } else {
                (sample * i16::MAX as f32) as i16
            };
            bytes.append(&mut value.to_le_bytes().to_vec());
        }
        bytes
    }

    fn wav_8_bytes(&mut self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(44 + self.samples.len());
        let header = WavHeader::new_pcm(1, self.sample_rate, 8, self.samples.len());
        bytes.append(&mut header.to_bytes().to_vec());
        let mut samples = Vec::new();
        swap(&mut self.samples, &mut samples);
        for sample in samples.into_iter() {
            let value = if sample < 0.0 {
                (sample.abs() * i8::MIN as f32) as i8
            } else {
                (sample * i8::MAX as f32) as i8
            };
            bytes.append(&mut value.to_le_bytes().to_vec());
        }
        bytes
    }
}

#[test]
fn test() {
    use std::{fs::File, io::Write};
    let mut wave = SquareWave::new(22050);
    wave.play_frequency(440.0, 0.3, 250);
    let buf = wave.export_8_bits();
    let mut file = File::create("./temp.wav").unwrap();
    file.write_all(&buf).unwrap();
}
