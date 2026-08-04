use mkpage::{assets::copy, error::AppError};
use std::fs;
use tempfile::tempdir;

#[test]
fn nested_assets_copy_byte_for_byte_in_sorted_order() {
    let temp = tempdir().unwrap();
    let static_root = temp.path().join("static");
    let output = temp.path().join("output");
    fs::create_dir_all(static_root.join("css")).unwrap();
    fs::write(static_root.join("z.bin"), [0_u8, 255]).unwrap();
    fs::write(static_root.join("css/site.css"), "body{}\n").unwrap();
    let copied = copy(&static_root, &output, &[]).unwrap();
    assert_eq!(
        copied,
        vec![
            std::path::PathBuf::from("css/site.css"),
            std::path::PathBuf::from("z.bin")
        ]
    );
    assert_eq!(fs::read(output.join("z.bin")).unwrap(), vec![0, 255]);
}
#[test]
fn asset_page_collisions_fail_before_copy() {
    let temp = tempdir().unwrap();
    let static_root = temp.path().join("static");
    let output = temp.path().join("output");
    fs::create_dir_all(&static_root).unwrap();
    fs::write(static_root.join("index.html"), "asset").unwrap();
    assert!(matches!(
        copy(
            &static_root,
            &output,
            &[std::path::PathBuf::from("index.html")]
        ),
        Err(AppError::StaticAssetCollision { .. })
    ));
}
