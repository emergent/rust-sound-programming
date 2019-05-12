extern crate sound_lib;
use sound_lib::MonoPcm;
use std::f64::consts::PI;

fn sine_wave(pcm: &mut MonoPcm, f0: f64, a: f64, offset: u32, duration: u32) {
    for n in 0..duration {
        pcm.sound_data
            .push(a * (2.0 * PI * f0 * (n as f64) / pcm.fs as f64).sin());
    }

    let fade_dur = pcm.fs / 100;
    for n in 0..fade_dur {
        pcm.sound_data[(offset + n) as usize] *= n as f64 / fade_dur as f64;
        pcm.sound_data[(offset + duration - n - 1) as usize] *= n as f64 / fade_dur as f64;
    }
}

fn main() {
    let fs = 44100;
    let bits = 16;
    let length = fs * 2;
    let sound_data = Vec::with_capacity(length as usize * 1);

    let mut pcm: MonoPcm = MonoPcm {
        fs,
        bits,
        length,
        sound_data,
        ..Default::default()
    };

    // sine wave
    let a = 0.1; // amplitude
    let f = vec![
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
    ]; // frequency

    let offset_base = pcm.fs / 4;
    let duration = pcm.fs / 4;
    for (i, &f0) in f.iter().enumerate() {
        sine_wave(&mut pcm, f0, a, offset_base * (i as u32), duration);
    }

    pcm.write_to("ex2_2.wav").ok();
}
