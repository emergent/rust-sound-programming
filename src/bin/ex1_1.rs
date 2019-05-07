extern crate sound_lib;

use sound_lib::MonoPcm;

fn main() {
    let pcm0 = MonoPcm::read_from("a.wav").unwrap();

    println!("fs:\t{}", pcm0.fs);
    println!("bits:\t{}", pcm0.bits);
    println!("length:\t{}", pcm0.length);

    println!("riff_chunk_size: \t{}", pcm0.riff_chunk_size);
    println!("fmt_chunk_size:  \t{}", pcm0.fmt_chunk_size);
    println!("wave_format_type:\t{}", pcm0.wave_format_type);
    println!("channel:         \t{}", pcm0.channel);
    println!("samples_per_sec: \t{}", pcm0.samples_per_sec);
    println!("bytes_per_sec:   \t{}", pcm0.bytes_per_sec);
    println!("block_size:      \t{}", pcm0.block_size);
    println!("bits_per_sample: \t{}", pcm0.bits_per_sample);
    println!("data_chunk_size: \t{}", pcm0.data_chunk_size);
}
