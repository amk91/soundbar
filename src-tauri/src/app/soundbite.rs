use std::{
    fs::File,
    path::PathBuf,
    io::BufReader,
};

use rodio::{
    Sink,
    Decoder,
    Source,
    OutputStreamHandle,
    buffer::SamplesBuffer
};
use anyhow::Result;

pub struct Soundbite {
    sink: Sink,
    buffer: Vec<i16>,
    channels: u16,
    sample_rate: u32,
    volume: f32,
    speed: f32,
}

impl Soundbite {
    pub fn new(
        stream_handle: &OutputStreamHandle,
        path: &PathBuf
    ) -> Result<Soundbite> {
        let file = File::open(path)?;
        let source = Decoder::new(BufReader::new(file))?;

        let channels = source.channels();
        let sample_rate = source.sample_rate();

        let sink = Sink::try_new(stream_handle)?;

        Ok(Soundbite {
            sink,
            buffer: source.collect(),
            channels,
            sample_rate,
            volume: 1.0,
            speed: 1.0,
        })
    }

    pub fn set_volume(&mut self, volume: f32) -> &mut Self {
        self.volume = volume;
        self
    }

    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.speed = speed;
        self
    }

    pub fn play(&self) {
        self.sink.stop();
        self.sink.set_volume(self.volume);
        self.sink.set_speed(self.speed);
        self.sink.append(
            SamplesBuffer::new(
                self.channels,
                self.sample_rate,
                self.buffer.clone()
            )
        );
    }
}
