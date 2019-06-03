extern crate gnuplot;
extern crate rand;
extern crate sound_lib;
use gnuplot::{Caption, Color, Figure};
use rand::prelude::*;
use sound_lib::MonoPcm;
use std::f64::consts::PI;

fn plot(data: &Vec<f64>, filename: &str, caption: &str) {
    let x = (0..data.len() as u32).collect::<Vec<u32>>();
    let mut fg = Figure::new();
    let range = 250;
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
    let f0 = 1.0;

    let mut rng = rand::thread_rng();
    for i in 1..22050 {
        let i2 = i as f64;
        let theta = rng.gen::<f64>() * 2.0 * PI;

        for n in 0..length {
            sound_data[n as usize] += (2.0 * PI * i2 * f0 * (n as f64) / (fs as f64) + theta).sin();
        }
    }

    let gain = 0.001;
    sound_data = sound_data.iter().map(|d| d * gain).collect();
    plot(&sound_data, "ex3_5.png", "white noise");

    let pcm = MonoPcm {
        fs,
        bits,
        length,
        sound_data,
        ..Default::default()
    };

    pcm.write_to("ex3_5.wav").ok();
}
