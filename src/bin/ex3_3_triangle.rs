extern crate gnuplot;
extern crate sound_lib;
use gnuplot::{Caption, Color, Figure};
use sound_lib::MonoPcm;
use std::f64::consts::PI;

fn plot(data: &Vec<f64>, filename: &str, caption: &str) {
    let x = (0..data.len() as u32).collect::<Vec<u32>>();
    let mut fg = Figure::new();
    let range = 500;
    fg.axes2d().lines(
        &x[0..range],
        &data[0..range],
        &[Caption(caption), Color("blue")],
    );
    fg.set_terminal("png", filename);
    fg.show();
}

fn main() {
    let fs = 44100;
    let bits = 16;
    let length = fs * 1;
    let mut sound_data = vec![0.0; length as usize];
    let f0 = 500.0;

    for i in (1..44).step_by(2) {
        let i2 = i as f64;
        for n in 0..length {
            sound_data[n as usize] += 1.0 / (i2 * i2)
                * (PI * i2 / 2.0).sin()
                * (2.0 * PI * i2 * f0 * (n as f64) / (fs as f64)).sin();
        }
    }

    let gain = 0.1;
    sound_data = sound_data.iter().map(|d| d * gain).collect();
    plot(&sound_data, "ex3_3.png", "triangle wave");

    let pcm = MonoPcm {
        fs,
        bits,
        length,
        sound_data,
        ..Default::default()
    };

    pcm.write_to("ex3_3.wav").ok();
}
