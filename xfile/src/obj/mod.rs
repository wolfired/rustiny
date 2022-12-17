//! <http://paulbourke.net/dataformats/obj/>
//!
//!

#![allow(dead_code)]

use std::error::Error;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

use rustiny_computer_graphic::point::Point2;
use rustiny_computer_graphic::point::Point3;
use rustiny_number::types::Number;

#[derive(Debug)]
pub struct Obj<T: Number> {
    pub vs: Vec<Point3<T>>,
    vts: Vec<Point2<T>>,
    vns: Vec<Point3<T>>,
    pub fs: Vec<(
        (usize, usize, usize),
        (usize, usize, usize),
        (usize, usize, usize),
    )>,
}

impl<T: Number> Obj<T>
where
    T: TryFrom<f32>,
{
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(path)?;

        let mut content = String::new();

        file.read_to_string(&mut content)?;

        let mut vs = Vec::new();
        let mut vts = Vec::new();
        let mut vns = Vec::new();
        let mut fs = Vec::new();

        for line in content.lines() {
            if line.starts_with("v ") {
                let ds: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .take(3)
                    .map(|w| w.parse().unwrap())
                    .collect();
                let Ok(v0) = ds[0].try_into() else {
                    return Err("err".into());
                };
                let Ok(v1) = ds[1].try_into() else {
                    return Err("err".into());
                };
                let Ok(v2) = ds[2].try_into() else {
                    return Err("err".into());
                };
                vs.push([v0, v1, v2].into());
            } else if line.starts_with("vt ") {
                let ds: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .take(2)
                    .map(|w| w.parse().unwrap())
                    .collect();
                let Ok(v0) = ds[0].try_into() else {
                        return Err("err".into());
                    };
                let Ok(v1) = ds[1].try_into() else {
                        return Err("err".into());
                    };
                vts.push([v0, v1].into());
            } else if line.starts_with("vn ") {
                let ds: Vec<f32> = line
                    .split_whitespace()
                    .skip(1)
                    .take(3)
                    .map(|w| w.parse().unwrap())
                    .collect();
                let Ok(v0) = ds[0].try_into() else {
                        return Err("err".into());
                    };
                let Ok(v1) = ds[1].try_into() else {
                        return Err("err".into());
                    };
                let Ok(v2) = ds[2].try_into() else {
                        return Err("err".into());
                    };
                vns.push([v0, v1, v2].into());
            } else if line.starts_with("f ") {
                let ds: Vec<(usize, usize, usize)> = line
                    .split_whitespace()
                    .skip(1)
                    .take(3)
                    .map(|w| {
                        let ds: Vec<usize> = w.split("/").map(|w| w.parse().unwrap()).collect();
                        (ds[0], ds[1], ds[2])
                    })
                    .collect();
                fs.push((ds[0], ds[1], ds[2]));
            }
        }

        Ok(Self { vs, vts, vns, fs })
    }
}
