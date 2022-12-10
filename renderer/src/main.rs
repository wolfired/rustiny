#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;
use std::path::Path;

use rustiny_computer_graphic::line_bresenham;
use rustiny_fixed_point::FixedPoint;
use rustiny_xfile::obj::Obj;
use rustiny_xfile::tga::TGAImage;

fn main() -> Result<(), Box<dyn Error>> {
    african_head("E:\\Desktop\\african_head.obj", "E:\\Desktop\\4444.tga")?;
    simple_line_fp("E:\\Desktop\\5555.tga")?;
    simple_line_f("E:\\Desktop\\6666.tga")?;

    Ok(())
}

fn simple_line_fp<P: AsRef<Path>>(tga_file: P) -> Result<(), Box<dyn Error>> {
    let mut img = TGAImage::new(1024, 1024);

    let w: FixedPoint<i32, 8> = (img.get_width() - 1).try_into()?;
    let h: FixedPoint<i32, 8> = (img.get_height() - 1).try_into()?;

    let p0: (FixedPoint<i32, 8>, FixedPoint<i32, 8>) = (0.0.try_into()?, 0.0.try_into()?);
    let p1: (FixedPoint<i32, 8>, FixedPoint<i32, 8>) = (1.0.try_into()?, 1.0.try_into()?);

    let one: FixedPoint<i32, 8> = 1.0f32.try_into()?;
    let two: FixedPoint<i32, 8> = 2.0f32.try_into()?;

    line_bresenham(
        (p0.0 + one) * w / two,
        (p0.1 + one) * h / two,
        (p1.0 + one) * w / two,
        (p1.1 + one) * h / two,
        |x, y| {
            img.set_color(
                x.try_into().unwrap(),
                y.try_into().unwrap(),
                255,
                255,
                255,
                255,
            );
        },
    );

    img.save(tga_file)?;

    Ok(())
}

fn simple_line_f<P: AsRef<Path>>(tga_file: P) -> Result<(), Box<dyn Error>> {
    let mut img = TGAImage::new(1024, 1024);

    let w = (img.get_width() - 1) as f32;
    let h = (img.get_height() - 1) as f32;

    let p0: (f32, f32) = (0.0, 0.0);
    let p1: (f32, f32) = (1.0, 1.0);

    let one: f32 = 1.0f32;
    let two: f32 = 2.0f32;

    line_bresenham(
        ((p0.0 + one) * w / two) as i64,
        ((p0.1 + one) * h / two) as i64,
        ((p1.0 + one) * w / two) as i64,
        ((p1.1 + one) * h / two) as i64,
        |x, y| {
            img.set_color(x as u16, y as u16, 255, 255, 255, 255);
        },
    );

    img.save(tga_file)?;

    Ok(())
}

fn african_head<P: AsRef<Path>>(obj_file: P, tga_file: P) -> Result<(), Box<dyn Error>> {
    let obj = Obj::load(obj_file)?;

    let mut img = TGAImage::new(1024, 1024);

    let w = img.get_width() - 1;
    let h = img.get_height() - 1;

    for f in obj.fs {
        let p0: (f32, f32, f32) = obj.vs[f.0 .0 - 1];
        let p1: (f32, f32, f32) = obj.vs[f.1 .0 - 1];

        line_bresenham(
            ((p0.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p0.1 + 1f32) * h as f32 / 2f32) as i64,
            ((p1.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p1.1 + 1f32) * h as f32 / 2f32) as i64,
            |x, y| img.set_color(x as u16, y as u16, 255, 255, 255, 255),
        );

        let p0: (f32, f32, f32) = obj.vs[f.1 .0 - 1];
        let p1: (f32, f32, f32) = obj.vs[f.2 .0 - 1];

        line_bresenham(
            ((p0.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p0.1 + 1f32) * h as f32 / 2f32) as i64,
            ((p1.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p1.1 + 1f32) * h as f32 / 2f32) as i64,
            |x, y| img.set_color(x as u16, y as u16, 255, 255, 255, 255),
        );

        let p0: (f32, f32, f32) = obj.vs[f.2 .0 - 1];
        let p1: (f32, f32, f32) = obj.vs[f.0 .0 - 1];

        line_bresenham(
            ((p0.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p0.1 + 1f32) * h as f32 / 2f32) as i64,
            ((p1.0 + 1f32) * w as f32 / 2f32) as i64,
            ((p1.1 + 1f32) * h as f32 / 2f32) as i64,
            |x, y| img.set_color(x as u16, y as u16, 255, 255, 255, 255),
        );
    }

    img.save(tga_file)?;

    Ok(())
}
