#![allow(unused_variables)]
#![allow(unused_imports)]

use std::error::Error;

use rustiny_computer_graphic::line_bresenham;
use rustiny_xfile::obj::Obj;
use rustiny_xfile::tga::TGAImage;

fn main() -> Result<(), Box<dyn Error>> {
    let obj = Obj::load("E:\\Desktop\\african_head.obj")?;

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

    img.save("E:\\Desktop\\4444.tga")?;

    Ok(())
}
