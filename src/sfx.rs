use rodio::*;
use std::convert::AsRef;
use std::fs::File;
use std::io::*;
use std::sync::Arc;

#[repr(u8)]
pub enum Sound {
    PaddleHit = 0,
    WallHit = 1,
    Goal = 2,
}

// https://github.com/RustAudio/rodio/issues/141#issuecomment-336150490
struct SoundData(Arc<Vec<u8>>);

impl AsRef<[u8]> for SoundData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

pub struct Sfx {
    device: rodio::Device,
    raw_sounds: Vec<SoundData>,
}

impl Default for Sfx {
    fn default() -> Self {
        Self::new()
    }
}

impl Sfx {
    pub fn new() -> Self {
        let sounds = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("sounds")
            .unwrap();

        let mut raw_sounds = Vec::new();
        // add audio files in the same index order of Sound enum value
        ["paddle.wav", "wall.wav", "goal.wav"]
            .iter()
            .for_each(|file| {
                let mut buf = Vec::new();
                File::open(sounds.join(file))
                    .unwrap()
                    .read_to_end(&mut buf)
                    .unwrap();
                raw_sounds.push(SoundData(Arc::new(buf)));
            });

        Self {
            device: rodio::default_output_device().unwrap(),
            raw_sounds,
        }
    }

    pub fn play(&self, sound: Sound) {
        let data = SoundData(self.raw_sounds[sound as usize].0.clone());
        let source = rodio::Decoder::new(Cursor::new(data)).unwrap();
        rodio::play_raw(&self.device, source.convert_samples());
    }
}
