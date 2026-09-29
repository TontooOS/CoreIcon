use coreicon::generator::*;
use coreicon::tico::Tico;
use coreicon::{Color, Gradient, GradientDirection};

fn circle_icon() -> IconCanvas {
    IconCanvas::new()
        .background(Background::color(Color::from_hex("#1d1d1d").unwrap()))
        .layer(
            Layer::new(LayerContent::circle(560.0))
                .position(232.0, 140.0)
                .tint(Color::WHITE),
        )
}

#[test]
fn export_writes_tico_container() {
    let bytes = Tico::export_bytes(&circle_icon(), "demo").unwrap();
    assert_eq!(&bytes[0..4], b"TICO", "must use the TICO container magic");
    // ArchiveKit validates the structure (manifest.fico + layer/*.tlyr).
    let info = archivekit::validate_tico(&bytes).unwrap();
    assert_eq!(info.layers, vec!["layer/00.tlyr".to_string()]);
    // The old ZIP layout is gone: no ZIP magic, no manifest.json.
    assert_ne!(&bytes[0..2], b"PK");
}

#[test]
fn export_feeds_app_builder_like_tbuild() {
    // Same loop as TBuild/FishRunner: CoreIcon export -> ArchiveKit .app icon.
    let bytes = Tico::export_bytes(&circle_icon(), "demo").unwrap();
    let mut manifest = archivekit::AppManifest::new("com.tontoo.demo", "1.0.0", "App/demo");
    manifest.names.push(("en_us".to_string(), "Demo".to_string()));
    let mut b = archivekit::AppBuilder::new("Demo").unwrap();
    b.set_manifest(manifest);
    b.add_executable("App/demo", b"binary".to_vec()).unwrap();
    let info = b.add_icon_tico("App/icon.tico", bytes).unwrap();
    assert_eq!(info.layers, vec!["layer/00.tlyr".to_string()]);
    let packed = b.finish().unwrap();
    assert_eq!(&packed[0..4], b"TAPP");
}

// NOTE: decode-dependent tests are ignored until Bugs/2.txt is fixed
// (CoreImage PNG encoder emits invalid streams for typical 1024px layers).
// Un-ignore them to verify the fix end to end.
#[test]
#[ignore = "blocked by Bugs/2.txt (CoreImage PNG encoder)"]
fn export_load_render_roundtrip() {
    let dir = std::env::temp_dir().join("coreicon_tico_check");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("roundtrip.tico");
    Tico::export(&circle_icon(), "demo", &path).unwrap();

    let icon = Tico::load(&path).unwrap();
    assert_eq!(icon.name(), "demo");
    assert_eq!(icon.layer_count(), 1);
    let img = icon.render_default().unwrap();
    assert_eq!((img.width(), img.height()), (1024, 1024));
    let small = icon.render(256, Some(Color::ACCENT)).unwrap();
    assert_eq!((small.width(), small.height()), (256, 256));
}

#[test]
#[ignore = "blocked by Bugs/2.txt (CoreImage PNG encoder)"]
fn load_bytes_roundtrip() {
    let bytes = Tico::export_bytes(&circle_icon(), "demo").unwrap();
    let icon = Tico::load_bytes(&bytes).unwrap();
    assert_eq!(icon.layer_count(), 1);
}

#[test]
#[ignore = "blocked by Bugs/2.txt (CoreImage PNG encoder)"]
fn gradient_background_roundtrip() {
    let icon = IconCanvas::new()
        .background(Background::gradient(Gradient::linear_two(
            Color::from_hex("#FF0000").unwrap(),
            Color::from_hex("#0000FF").unwrap(),
        )
        .with_direction(GradientDirection::LeftToRight)))
        .layer(
            Layer::new(LayerContent::rect(300.0, 300.0, 80.0))
                .position(362.0, 362.0)
                .tint(Color::WHITE),
        );
    let bytes = Tico::export_bytes(&icon, "grad").unwrap();
    let loaded = Tico::load_bytes(&bytes).unwrap();
    assert_eq!(loaded.layer_count(), 1);
    let img = loaded.render(64, None).unwrap();
    assert_eq!((img.width(), img.height()), (64, 64));
}

#[test]
fn corrupt_bytes_fail_to_load() {
    assert!(Tico::load_bytes(b"not a tico").is_err());
    let mut bytes = Tico::export_bytes(&circle_icon(), "demo").unwrap();
    let last = bytes.len() - 1;
    bytes[last] ^= 0xFF;
    assert!(Tico::load_bytes(&bytes).is_err());
}
