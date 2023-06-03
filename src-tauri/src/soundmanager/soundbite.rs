use std::{
    fs::File,
    path::PathBuf,
    io::{Cursor, BufReader},
};

use rodio::{
    Sink,
    Decoder,
    Source,
    OutputStreamHandle,
    buffer::SamplesBuffer
};
use anyhow::Result;

//TODO: define struct SoundbiteData for serialization purposes
// to use inside the existing Soundbite struct

pub struct SoundbiteData {
    buffer: Vec<i16>,
    channels: u16,
    sample_rate: u32,
    volume: f32,
    speed: f32,
}

impl SoundbiteData {
    pub fn new(
        buffer: Vec<u8>,
        volume: f32,
        speed: f32,
    ) -> Result<SoundbiteData> {
        let source = Decoder::new(Cursor::new(buffer))?;
        let channels = source.channels();
        let sample_rate = source.sample_rate();
        Ok(SoundbiteData {
            buffer: source.collect(),
            channels,
            sample_rate,
            volume,
            speed,
        })
    }
}

pub struct Soundbite {
    sink: Sink,

    pub name: String,
    data: SoundbiteData,
}

impl Soundbite {
    pub fn new(
        stream_handle: &OutputStreamHandle,
        name: String,
        data: SoundbiteData,
    ) -> Result<Soundbite> {
        let sink = Sink::try_new(stream_handle)?;

        Ok(Soundbite {
            sink,
            name,
            data,
        })
    }

    pub fn set_volume(&mut self, volume: f32) -> &mut Self {
        self.data.volume = volume;
        self
    }

    pub fn set_speed(&mut self, speed: f32) -> &mut Self {
        self.data.speed = speed;
        self
    }

    pub fn play(&self) {
        self.sink.stop();
        self.sink.set_volume(self.data.volume);
        self.sink.set_speed(self.data.speed);
        self.sink.append(
            SamplesBuffer::new(
                self.data.channels,
                self.data.sample_rate,
                self.data.buffer.clone()
            )
        );
    }
}
