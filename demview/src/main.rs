use std::{fs::File, io::BufReader, path::PathBuf};

use clap::Parser;
use ghosting::{GhostHeader, RunLineV1};
use kiss3d::{camera::FirstPerson, window::Window, nalgebra as na};
use log::info;

#[derive(Debug, Parser)]
struct Opts {
    ghostfile: PathBuf,
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let opts = Opts::parse();

    let f = File::open(opts.ghostfile)?;
    let mut buf = BufReader::new(f);
    let hdr = GhostHeader::read(&mut buf)?;
    info!("reading v{} ghostfile for game {}", hdr.version, hdr.game);

    let mut lines = Vec::new();
    loop {
        let line = match RunLineV1::read(&mut buf) {
            Ok(line) => Ok(line),
            Err(e) => match e.kind() {
                std::io::ErrorKind::UnexpectedEof => break,
                _ => Err(e),
            },
        }?;
        lines.push(line);
    }

    let mut win = Window::new("ghosting-rs");

    let sc = 1. / 1000.;
    let scale = na::Scale3::new(sc, sc, sc);
    let shift = na::Isometry3::new(
        na::Vector3::new(-lines[0].x, -lines[0].y, -lines[0].z),
        na::zero(),
    );

    let mut cam_firstperson =
        FirstPerson::new(na::Point3::new(10., 10., 10.), na::Point3::origin());
    while win.render_with_camera(&mut cam_firstperson) {
        let mut it = lines.iter();
        let mut line = it.next().unwrap();
        loop {
            let pos = na::Point3::new(line.x, line.y, line.z);
            win.draw_point(&(scale * (shift * pos)), &na::Point3::new(0., 1., 0.));
            line = it.next().unwrap();
            if line.map != "" {
                break;
            }
        }
    }

    Ok(())
}
