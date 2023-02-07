use byteorder::{LittleEndian as LE, ReadBytesExt};
use clap::Parser;
use std::{io::{Read, BufReader}, path::PathBuf, fs::File};

#[derive(Debug)]
struct GhostHeader {
    version: u8,
    game: u8,
    trail_length: u8,

    trail_color: (u8, u8, u8),
    ghost_color: (u8, u8, u8),
}

impl GhostHeader {
    fn read<R>(rdr: &mut R) -> Result<Self, std::io::Error>
    where
        R: Read + ReadBytesExt,
    {
        let _ = rdr.read_u8()?;
        let version = rdr.read_u8()?;
        let game = rdr.read_u8()?;
        let trail_color = (rdr.read_u8()?, rdr.read_u8()?, rdr.read_u8()?);
        let ghost_color = (rdr.read_u8()?, rdr.read_u8()?, rdr.read_u8()?);
        let trail_length = rdr.read_u8()?;

        Ok(Self {
            version, game,
            trail_length,
            trail_color, ghost_color,
        })
    }
}

#[derive(Debug)]
struct RunLineV1 {
    map: String,
    name: String,
    timestamp: f32,
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
struct RunLineV2 {
    map: String,
    name: String,
    timestamp: f32,
    x: f32,
    y: f32,
    z: f32,
    yaw: f32,
}

fn read_length_prefixed_string<R>(rdr: &mut R) -> Result<String, std::io::Error> where R: Read {
    let len = rdr.read_u8()? as usize;
    let mut buf = vec![0; len];
    rdr.read_exact(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).into_owned())
}

impl RunLineV1 {
    fn read<R>(rdr: &mut R) -> Result<Self, std::io::Error>
    where
        R: Read + ReadBytesExt,
    {
        let map = read_length_prefixed_string(rdr)?;
        let name = read_length_prefixed_string(rdr)?;
        let timestamp = rdr.read_f32::<LE>()?;
        let x = rdr.read_f32::<LE>()?;
        let y = rdr.read_f32::<LE>()?;
        let z = rdr.read_f32::<LE>()?;

        Ok(Self {
            map, name,

            timestamp,
            x, y, z,
        })
    }
}

#[derive(Debug, Parser)]
struct Opts {
    ghostfile: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts= Opts::parse();

    let f = File::open(opts.ghostfile)?;
    let mut buf = BufReader::new(f);
    let hdr = GhostHeader::read(&mut buf)?;
    println!("hdr={:#?}", hdr);

    let mut lines = Vec::new();
    loop {
        let line = match RunLineV1::read(&mut buf) {
            Ok(line) => Ok(line),
            Err(e) => match e.kind() {
                std::io::ErrorKind::UnexpectedEof => break,
                _ => Err(e),
            },
        };
        lines.push(line);
    }
    println!("line={:#?}", lines[1]);
    // while buf.end

    Ok(())
}
