extern crate sound_lib;
use sound_lib::MonoPcm;
use std::f64::consts::PI;

fn main() {
    let fs = 44100;
    let bits = 16;
    let length = fs * 1;
    let mut sdata = Vec::with_capacity(length as usize * 1);

    // sine wave
    let a = 0.1; // amplitude
    let f = vec![
        261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
    ]; // frequency

    for &f0 in &f {
        for n in 0..length {
            sdata.push(a * (2.0 * PI * f0 * n as f64 / fs as f64).sin());
        }
    }

    let pcm = MonoPcm {
        fs,
        bits,
        length: length * f.len() as u32,
        sound_data: sdata,
        ..Default::default()
    };

    pcm.write_to("ex2_1.wav").ok();
}
