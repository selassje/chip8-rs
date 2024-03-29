use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::time::Duration;

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            if self.phase >= 0.0 && self.phase < 0.5 {
                 *x = self.volume;
            } else {
                 *x = -self.volume;
            }
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

pub struct Audio {
    device  : sdl2::audio::AudioDevice<SquareWave>
}

impl Audio {

    pub fn new(audio_subsystem: &sdl2::AudioSubsystem) -> Self
    {
        let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  
        samples: None
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }}).unwrap();

        Audio {
          device: device,
        }
    }

    pub fn beep(&self) {
    self.device.resume();
    std::thread::sleep(Duration::from_millis(100));
    self.device.pause();
    }
}