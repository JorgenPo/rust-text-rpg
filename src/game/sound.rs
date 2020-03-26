//! Sound subsystem code

use std::error::Error;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use rodio::{Source, Device, Decoder};
use rodio::source::Buffered;
use std::ops::Deref;


pub struct Manager {
    device: Option<rodio::Device>,
    sound_cache: HashMap<String, Buffered<Decoder<BufReader<File>>>>,
}

impl Manager {
    pub fn new() -> Self {
        let device = rodio::default_output_device();
        Manager {
            device,
            sound_cache: HashMap::new()
        }
    }

    fn load(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(name)?;
        let reader = BufReader::new(file);
        let source = rodio::Decoder::new(reader)?.buffered();

        self.sound_cache.insert(String::from(name), source);

        Ok(())
    }

    pub fn play(&mut self, name: &str) -> Result<(), Box<dyn Error>> {
        // Just silently return OK
        if self.device.is_none() {
            return Ok(());
        }

        let sound = match self.sound_cache.get(name) {
            None => {
                self.load(name)?;
                self.sound_cache.get(name).unwrap()
            },
            Some(sound) => sound,
        };

        rodio::play_raw(&self.device.as_ref().unwrap(), sound.clone().convert_samples());

        Ok(())
    }
}