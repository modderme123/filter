extern crate piston_window;
extern crate image;

use piston_window::*;
use std::path::Path;

fn posterize(img: &mut image::RgbaImage) {
    let mut pr = [0 as i16; 7];
    let mut pg = [0 as i16; 7];
    let mut pb = [0 as i16; 7];
    for i in 0..7 {
        pr[i] = (255 * i / 7) as i16;
        pg[i] = (255 * i / 7) as i16;
        pb[i] = (255 * i / 7) as i16;
    }
    for _ in 0..1000 {
        let mut count = [0usize; 7];
        let mut sr = [0usize; 7];
        let mut sg = [0usize; 7];
        let mut sb = [0usize; 7];
        for pixel in img.pixels_mut() {
            let mut ind = 0;
            let mut min = ((pr[ind] - pixel[0] as i16) as i32).pow(2) +
                          ((pg[ind] - pixel[1] as i16) as i32).pow(2) +
                          ((pb[ind] - pixel[2] as i16) as i32).pow(2);
            for i in 1..7 {
                let dist = ((pr[i] - pixel[0] as i16) as i32).pow(2) +
                           ((pg[i] - pixel[1] as i16) as i32).pow(2) +
                           ((pb[i] - pixel[2] as i16) as i32).pow(2);
                if dist < min {
                    min = dist;
                    ind = i;
                }
            }
            count[ind] += 1;
            sr[ind] += pixel[0] as usize;
            sg[ind] += pixel[1] as usize;
            sb[ind] += pixel[2] as usize;
        }
        for i in 0..7 {
            if count[i] > 0 {
                pr[i] = (sr[i] / count[i]) as i16;
                pg[i] = (sg[i] / count[i]) as i16;
                pb[i] = (sb[i] / count[i]) as i16;
            }
        }
    }
    for pixel in img.pixels_mut() {
        let mut ind = 0;
        let mut min = ((pr[ind] - pixel[0] as i16) as f64).powf(2.0) +
                      ((pg[ind] - pixel[1] as i16) as f64).powf(2.0) +
                      ((pb[ind] - pixel[2] as i16) as f64).powf(2.0);
        for i in 1..7 {
            let dist = ((pr[i] - pixel[0] as i16) as f64).powf(2.0) +
                       ((pg[i] - pixel[1] as i16) as f64).powf(2.0) +
                       ((pb[i] - pixel[2] as i16) as f64).powf(2.0);
            if dist < min {
                min = dist;
                ind = i;
            }
        }
        pixel[0] = pr[ind] as u8;
        pixel[1] = pg[ind] as u8;
        pixel[2] = pb[ind] as u8;
    }
}

fn main() {
    let mut img = image::open(&Path::new("assets/dp.jpg"))
        .unwrap()
        .to_rgba();

    let (width, height) = img.dimensions();

    let mut cursor = [0.0; 2];
    let mut press = false;

    let mut window: PistonWindow = WindowSettings::new("Image filter", (width + 300, height))
        .build()
        .unwrap();

    let mut glyphs = Glyphs::new("assets/NotoSans.ttf", window.factory.clone(), TextureSettings::new()).unwrap();

    let mut rust_logo =
        Texture::from_image(&mut window.factory, &img, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        rust_logo.update(&mut window.encoder, &img).unwrap();
        window.draw_2d(&e, |c, g| {
            clear([1.0; 4], g);
            image(&rust_logo, c.transform, g);
            rectangle(if press {
                          [0.5, 0.5, 0.5, 1.0]
                      } else {
                          [0.9, 0.9, 0.9, 1.0]
                      },
                      [width as f64, 0.0, 300.0, 40.0],
                      c.transform,
                      g);
            text([0.0, 0.0, 0.0, 1.0],
                 32,
                 "KMeans Posterize",
                 &mut glyphs,
                 c.transform.trans(width as f64, 30.0),
                 g)
                    .unwrap();
        });

        if e.press_args() == Some(Button::Mouse(MouseButton::Left)) &&
           cursor[0] < width as f64 + 300.0 && cursor[0] > width as f64 &&
           cursor[1] > 0.0 && cursor[1] < 40.0 {
            press = true;
        }

        if e.release_args() == Some(Button::Mouse(MouseButton::Left)) && press == true {
            press = false;
            posterize(&mut img);
        }
        e.mouse_cursor(|x, y| { cursor = [x, y]; });
    }
}
