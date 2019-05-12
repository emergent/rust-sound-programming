use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

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

        Self::print_bytes(&buf_header, false);

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
        let normalize_base = 2_u16.pow((bits_per_sample - 1).into()) as f64;
        loop {
            let size = reader.read(&mut buf)?;
            if size == 0 {
                break;
            }

            for i in 0..size / 2 {
                let chunk = ((buf[i * 2 + 1] as u16) << 8 | (buf[i * 2] as u16)) as i16;
                data.push((chunk as f64) / normalize_base);
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

    pub fn write_to(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut writer = BufWriter::new(File::create(filename)?);

        //Self::str_to_bytes(&mut buf_header[0..4], "RIFF");
        writer.write(b"RIFF")?;
        writer.write(&Self::u32_to_bytes(36 + self.length * 2))?;
        writer.write(b"WAVEfmt ")?;
        writer.write(&Self::u32_to_bytes(16))?; // fmt_chunk_size
        writer.write(&Self::u16_to_bytes(1))?; // wave_format_type
        writer.write(&Self::u16_to_bytes(1))?; // channel
        writer.write(&Self::u32_to_bytes(self.fs))?; // sample_per_sec
        writer.write(&Self::u32_to_bytes(self.fs * self.bits as u32 / 8))?; // bytes_per_sec
        writer.write(&Self::u16_to_bytes(self.bits / 8))?; // block_size
        writer.write(&Self::u16_to_bytes(self.bits))?; // bits_per_sample
        writer.write(b"data")?;
        writer.write(&Self::u32_to_bytes(self.length * 2))?; // bits_per_sample

        for data in self
            .sound_data
            .iter()
            .map(|f| {
                let s = match (f + 1.0) / 2.0 * 65536.0 {
                    s if s > 65535.0 => 65535.0,
                    s if s < 0.0 => 0.0,
                    s => s,
                };
                ((s + 0.5) as i32 - 32768) as i16
            })
            .map(|s| Self::i16_to_bytes(s))
        {
            writer.write(&data)?;
        }

        Ok(())
    }

    fn u32_to_bytes(u: u32) -> [u8; 4] {
        let mut bytes = [0u8; 4];
        bytes[0] = u as u8;
        bytes[1] = (u >> 8) as u8;
        bytes[2] = (u >> 16) as u8;
        bytes[3] = (u >> 24) as u8;
        bytes
    }

    fn u16_to_bytes(u: u16) -> [u8; 2] {
        let mut bytes = [0u8; 2];
        bytes[0] = u as u8;
        bytes[1] = (u >> 8) as u8;
        bytes
    }

    fn i16_to_bytes(i: i16) -> [u8; 2] {
        Self::u16_to_bytes(i as u16)
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
