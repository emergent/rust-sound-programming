use std::fs::File;
use std::io::{Read, BufReader};


pub struct MonoPcm {
    pub fs: u32,
    pub bits: u16,
    pub length: u32,
    pub sound_data: Vec<f64>,
}

impl MonoPcm {
    pub fn read_from(filename: &str) -> Result<MonoPcm, std::io::Error> {
        let mut reader = BufReader::new(File::open(filename)?);
        let mut buf_header = [0; 44];
        let mut data = Vec::new();
        reader.read_exact(&mut buf_header)?;
        for (i, &b) in buf_header.iter().enumerate() {
            let c = if b >= 32 && b <= 126 { b as char } else { '.' };
            println!("{}\t: {:02x}\t=> {}", i, b, c);
        }
        println!("");

        let samples_per_sec = (buf_header[27] as u32) << 24 |
            (buf_header[26] as u32) << 16 |
            (buf_header[25] as u32) << 8 |
            (buf_header[24] as u32);
        let bits_per_sample = (buf_header[35] as u16) << 8 |
            (buf_header[34] as u16);
        let data_chunk_size = (buf_header[43] as u32) << 24 |
            (buf_header[42] as u32) << 16 |
            (buf_header[41] as u32) << 8 |
            (buf_header[40] as u32);

        let mut buf = [0; 65536];
        loop {
            let size = reader.read(&mut buf)?;
            for i in 0..size / 2 {
                let chunk = (buf[i * 2] as u16) << 8 | (buf[i * 2 + 1] as u16);
                data.push((chunk as f64) / 32768.0);
            }

            if size == 0 {
                break
            }
        }

        Ok(MonoPcm {
            fs: samples_per_sec,
            bits: bits_per_sample,
            length: data_chunk_size,
            sound_data: data,
        })
    }
}
