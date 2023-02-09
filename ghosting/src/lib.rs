use std::io::Read;

use byteorder::{LittleEndian as LE, ReadBytesExt};

#[derive(Debug)]
pub struct GhostHeader {
    pub version: u8,
    pub game: u8,

    pub trail_length: u8,
    pub trail_color: (u8, u8, u8),
    pub ghost_color: (u8, u8, u8),
}

impl GhostHeader {
    pub fn read<R>(rdr: &mut R) -> Result<Self, std::io::Error>
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
            version,
            game,
            trail_length,
            trail_color,
            ghost_color,
        })
    }
}

#[derive(Debug)]
pub struct RunLineV1 {
    pub map: String,
    pub name: String,
    pub timestamp: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
pub struct RunLineV2 {
    pub map: String,
    pub name: String,
    pub timestamp: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub yaw: f32,
}

fn read_length_prefixed_string<R>(rdr: &mut R) -> Result<String, std::io::Error>
where
    R: Read,
{
    let len = rdr.read_u8()? as usize;
    let mut buf = vec![0; len];
    rdr.read_exact(&mut buf)?;

    Ok(String::from_utf8_lossy(&buf).into_owned())
}

impl RunLineV1 {
    pub fn read<R>(rdr: &mut R) -> Result<Self, std::io::Error>
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
            map,
            name,

            timestamp,
            x,
            y,
            z,
        })
    }
}
