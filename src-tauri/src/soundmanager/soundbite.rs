use std::io::Cursor;

use serde::{
    Serialize,
    Deserialize
};

use rodio::{
    Sink,
    Decoder,
    Source,
    OutputStreamHandle,
    buffer::SamplesBuffer
};

use anyhow::Result;

use super::utils::{
    SoundManagerError,
    NewSoundbiteError,
};

//TODO: define struct SoundbiteData for serialization purposes
// to use inside the existing Soundbite struct

#[derive(Serialize, Deserialize, Debug)]
pub struct SoundbiteData {
    pub name: String,
    buffer: Vec<i16>,
    channels: u16,
    sample_rate: u32,
    pub volume: f32,
    pub speed: f32,
}

impl SoundbiteData {
    pub fn new(
        name: String,
        buffer: Vec<u8>,
        volume: f32,
        speed: f32,
    ) -> Result<SoundbiteData> {
        let source = Decoder::new(Cursor::new(buffer))?;
        let channels = source.channels();
        let sample_rate = source.sample_rate();
        Ok(SoundbiteData {
            name,
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
    pub data: SoundbiteData,
}

impl Soundbite {
    pub fn new(
        stream_handle: &OutputStreamHandle,
        data: SoundbiteData,
    ) -> Result<Soundbite> {
        let sink = Sink::try_new(stream_handle)?;

        Ok(Soundbite {
            sink,
            data,
        })
    }

    pub fn from_data(data: SoundbiteData) -> Soundbite {
        Soundbite {
            sink: Sink::new_idle().0,
            data
        }
    }

    pub fn init_sink(
        &mut self,
        stream_handle: &OutputStreamHandle
    ) -> Result<(), SoundManagerError> {
        if let Ok(sink) = Sink::try_new(stream_handle) {
            self.sink = sink;
            Ok(())
        } else {
            Err(SoundManagerError::NewSoundbiteError(
                NewSoundbiteError::UnableToCreateFromData(
                    self.data.name.clone()
                )
            ))
        }
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
