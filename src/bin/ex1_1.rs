extern crate sound_lib;

use sound_lib::MonoPcm;

fn main() {
    let pcm0 = MonoPcm::read_from("a.wav").unwrap();

    println!("{}", pcm0.fs);
    println!("{}", pcm0.bits);
    println!("{}", pcm0.length);
}
