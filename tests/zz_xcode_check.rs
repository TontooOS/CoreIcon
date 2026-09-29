// Temporary: verify xcode full-icon tint variants in memory (PNG encoding
// is broken per Bugs/2.txt, so check process() output directly).
use coreicon::generator::AppIcon;
use coreicon::Color;

fn px(img: &coreimage::RgbaImage, x: u32, y: u32) -> [u8; 4] {
    let p = img.get_pixel(x, y);
    [p[0], p[1], p[2], p[3]]
}

#[test]
fn xcode_tint_variants() {
    let src = concat!(env!("CARGO_MANIFEST_DIR"), "/examples/xcode/xcode.png");
    let light = AppIcon::from_file(src).light().process().unwrap();
    let red = AppIcon::from_file(src)
        .light()
        .tint(Color::from_hex("#FF3B30").unwrap())
        .process()
        .unwrap();
    let green = AppIcon::from_file(src)
        .light()
        .tint(Color::from_hex("#34C759").unwrap())
        .process()
        .unwrap();

    // Light keeps the original blue background.
    let lb = px(&light, 512, 120);
    println!("light bg {lb:?}");
    assert!(lb[2] > 200 && lb[0] < 120, "light background changed: {lb:?}");

    // Full-icon filter: background takes the tint hue (bright), hammer a
    // darker shade of the same hue.
    let rb = px(&red, 512, 120);
    let rh = px(&red, 650, 350);
    println!("red bg {rb:?} hammer {rh:?}");
    assert!(rb[0] > 200 && rb[0] > rb[1] && rb[0] > rb[2], "red bg not tinted: {rb:?}");
    assert!(rh[0] > rh[1] && rh[0] > rh[2] && rh[0] < rb[0], "red hammer wrong: {rh:?}");

    let gb = px(&green, 512, 120);
    let gh = px(&green, 650, 350);
    println!("green bg {gb:?} hammer {gh:?}");
    assert!(gb[1] > 150 && gb[1] > gb[0] && gb[1] > gb[2], "green bg not tinted: {gb:?}");
    assert!(gh[1] > gh[0] && gh[1] > gh[2] && gh[1] < gb[1], "green hammer wrong: {gh:?}");
}
