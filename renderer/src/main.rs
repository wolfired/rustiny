#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;
use std::path::Path;

use rustiny_computer_graphic::line_bresenham;
use rustiny_computer_graphic::Point2;
use rustiny_fixed_point::types::FixedPoint;
use rustiny_linear_algebra::X;
use rustiny_linear_algebra::Y;
use rustiny_number::ops::One;
use rustiny_xfile::obj::Obj;
use rustiny_xfile::tga::TGAImage;

fn main() -> Result<(), Box<dyn Error>> {
    simple_line_fp("E:\\Desktop\\4444.tga")?;
    simple_line_f("E:\\Desktop\\5555.tga")?;
    african_head("E:\\Desktop\\african_head.obj", "E:\\Desktop\\6666.tga")?;

    Ok(())
}

fn simple_line_fp<P: AsRef<Path>>(tga_file: P) -> Result<(), Box<dyn Error>> {
    let mut img = TGAImage::new(1024, 1024);

    let w: FixedPoint<i32, 8> = (img.get_width() - 1).try_into()?;
    let h: FixedPoint<i32, 8> = (img.get_height() - 1).try_into()?;

    let p0: Point2<FixedPoint<i32, 8>> = [0.try_into()?, 0.try_into()?].into();
    let p1: Point2<FixedPoint<i32, 8>> = [w.try_into()?, h.try_into()?].into();

    line_bresenham(p0, p1, |x, y| {
        img.set_color(
            x.try_into().unwrap(),
            y.try_into().unwrap(),
            255,
            255,
            255,
            255,
        );
    });

    img.save(tga_file)?;

    Ok(())
}

fn simple_line_f<P: AsRef<Path>>(tga_file: P) -> Result<(), Box<dyn Error>> {
    let mut img = TGAImage::new(1024, 1024);

    let w = (img.get_width() - 1) as i32;
    let h = (img.get_height() - 1) as i32;

    let p0: Point2<i32> = [0, 0].into();
    let p1: Point2<i32> = [w, h].into();

    line_bresenham(p0, p1, |x, y| {
        img.set_color(x as u16, y as u16, 255, 255, 255, 255);
    });

    img.save(tga_file)?;

    Ok(())
}

fn african_head<P: AsRef<Path>>(obj_file: P, tga_file: P) -> Result<(), Box<dyn Error>> {
    let obj = Obj::<f32>::load(obj_file)?;

    let mut img = TGAImage::new(1024, 1024);

    let w = img.get_width() - 1;
    let h = img.get_height() - 1;

    let mut vs: Vec<Point2<i16>> = Vec::with_capacity(obj.vs.capacity());

    for v in obj.vs {
        let x = (v.x() + 1.0) * w as f32 / 2.0;
        let y = (v.y() + 1.0) * h as f32 / 2.0;
        vs.push([x as i16, y as i16].into());
    }

    for f in obj.fs {
        let p0 = vs[f.0 .0 - 1];
        let p1 = vs[f.1 .0 - 1];
        let p2 = vs[f.2 .0 - 1];

        line_bresenham(p0, p1, |x, y| {
            img.set_color(x as u16, y as u16, 255, 255, 255, 255)
        });

        line_bresenham(p1, p2, |x, y| {
            img.set_color(x as u16, y as u16, 255, 255, 255, 255)
        });

        line_bresenham(p2, p0, |x, y| {
            img.set_color(x as u16, y as u16, 255, 255, 255, 255)
        });
    }

    img.save(tga_file)?;

    Ok(())
}
