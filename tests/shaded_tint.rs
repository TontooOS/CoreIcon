use coreicon::generator::*;
use coreicon::Color;
use image::{Rgba, RgbaImage};

fn make_gradient_png(path: &std::path::Path) {
    // 64x64: top half white, bottom half mid-gray (128).
    let mut img = RgbaImage::new(64, 64);
    for y in 0..64 {
        for x in 0..64 {
            let v = if y < 32 { 255u8 } else { 128u8 };
            img.put_pixel(x, y, Rgba([v, v, v, 255]));
        }
    }
    img.save(path).unwrap();
}

#[test]
fn shaded_flag_preserves_gradient_flat_does_not() {
    let dir = std::env::temp_dir().join("coreicon_shaded_check");
    std::fs::create_dir_all(&dir).unwrap();
    let src = dir.join("pill.png");
    make_gradient_png(&src);
    let tint = Color::new(0.2, 0.4, 1.0, 1.0);

    let render = |shaded: bool| {
        IconCanvas::new()
            .layer(
                Layer::new(LayerContent::image(src.to_str().unwrap()))
                    .position(0.0, 0.0)
                    .size(64.0, 64.0)
                    .tint(tint)
                    .shaded(shaded),
            )
            .render()
    };

    // Layer canvas is 1024; our 64px image sits at top-left.
    let flat = render(false);
    let shaded = render(true);

    let flat_top = *flat.get_pixel(32, 8);
    let flat_bottom = *flat.get_pixel(32, 40);
    let shaded_top = *shaded.get_pixel(32, 8);
    let shaded_bottom = *shaded.get_pixel(32, 40);

    // Flat: top and bottom identical full tint.
    assert_eq!([flat_top[0], flat_top[1], flat_top[2]], [51, 102, 255], "flat top {:?}", flat_top);
    assert_eq!([flat_bottom[0], flat_bottom[1], flat_bottom[2]], [51, 102, 255], "flat bottom {:?}", flat_bottom);

    // Shaded: white -> full tint, mid-gray -> ~half tint.
    assert_eq!([shaded_top[0], shaded_top[1], shaded_top[2]], [51, 102, 255], "shaded top {:?}", shaded_top);
    assert_eq!([shaded_bottom[0], shaded_bottom[1], shaded_bottom[2]], [26, 51, 128], "shaded bottom {:?}", shaded_bottom);

    // Alpha preserved everywhere.
    for p in [flat_top, flat_bottom, shaded_top, shaded_bottom] {
        assert_eq!(p[3], 255);
    }
}

#[test]
fn shaded_defaults_to_false() {
    let l = Layer::new(LayerContent::rect(10.0, 10.0, 2.0));
    assert!(!l.shaded, "default must stay false for back-compat");
}
