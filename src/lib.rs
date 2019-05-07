use std::fs::File;
use std::io::{BufReader, Read};

pub struct MonoPcm {
    pub fs: u32,
    pub bits: u16,
    pub length: u32,
    pub sound_data: Vec<f64>,

    pub riff_chunk_size: u32,
    pub fmt_chunk_size: u32,
    pub wave_format_type: u16,
    pub channel: u16,
    pub samples_per_sec: u32,
    pub bytes_per_sec: u32,
    pub block_size: u16,
    pub bits_per_sample: u16,
    pub data_chunk_size: u32,
}

impl MonoPcm {
    pub fn read_from(filename: &str) -> Result<MonoPcm, std::io::Error> {
        let mut reader = BufReader::new(File::open(filename)?);
        let mut buf_header = [0; 44];
        let mut data = Vec::new();
        reader.read_exact(&mut buf_header)?;

        Self::print_bytes(&buf_header, true);

        let riff_chunk_size = Self::bytes_to_u32(&buf_header[4..8]);
        let fmt_chunk_size = Self::bytes_to_u32(&buf_header[16..20]);
        let wave_format_type = Self::bytes_to_u16(&buf_header[20..22]);
        let channel = Self::bytes_to_u16(&buf_header[22..24]);
        let samples_per_sec = Self::bytes_to_u32(&buf_header[24..28]);
        let bytes_per_sec = Self::bytes_to_u32(&buf_header[28..32]);
        let block_size = Self::bytes_to_u16(&buf_header[32..34]);
        let bits_per_sample = Self::bytes_to_u16(&buf_header[34..36]);
        let data_chunk_size = Self::bytes_to_u32(&buf_header[40..44]);

        let mut buf = [0; 65536];
        loop {
            let size = reader.read(&mut buf)?;
            if size == 0 {
                break;
            }
            //Self::print_bytes(&buf[..size], false);

            for i in 0..size / 2 {
                let chunk = (buf[i * 2 + 1] as u16) << 8 | (buf[i * 2] as u16);
                data.push((chunk as f64) / 32768.0);
            }
        }

        Ok(MonoPcm {
            fs: samples_per_sec,
            bits: bits_per_sample,
            length: data_chunk_size / 2,
            sound_data: data,
            riff_chunk_size,
            fmt_chunk_size,
            wave_format_type,
            channel,
            samples_per_sec,
            bytes_per_sec,
            block_size,
            bits_per_sample,
            data_chunk_size,
        })
    }

    fn bytes_to_u32(bytes: &[u8]) -> u32 {
        // little endian
        (bytes[3] as u32) << 24
            | (bytes[2] as u32) << 16
            | (bytes[1] as u32) << 8
            | (bytes[0] as u32)
    }

    fn bytes_to_u16(bytes: &[u8]) -> u16 {
        // little endian
        (bytes[1] as u16) << 8 | (bytes[0] as u16)
    }

    fn print_bytes(bytes: &[u8], disp_char: bool) {
        println!("    \t 0\t 1\t 2\t 3\t 4\t 5\t 6\t 7");
        for (i, &b) in bytes.iter().enumerate() {
            if i % 8 == 0 {
                print!("{:04x}\t", i);
            }
            let s = if disp_char && b >= 32 && b <= 126 {
                format!(" {}", b as char)
            } else {
                format!("{:02x}", b)
            };
            if i % 8 == 7 {
                println!("{:2}", s);
            } else {
                print!("{:2}\t", s);
            }
        }
        println!("");
    }
}
