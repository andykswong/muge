#![cfg(all(feature = "serde", feature = "gltf-name"))]

use assert_json_diff::{assert_json_matches, CompareMode, Config, NumericMode};
use mugltf::GltfAsset;
use serde_json::Value;

#[test]
fn serde_animated_cube() -> Result<(), Error> {
    assert_serde_matches(include_str!("./model/AnimatedCube/glTF/AnimatedCube.gltf"))
}

#[test]
fn serde_simple_skin() -> Result<(), Error> {
    assert_serde_matches(include_str!(
        "./model/SimpleSkin/glTF-Embedded/SimpleSkin.gltf"
    ))
}

#[test]
fn serde_simple_sparse_accessor() -> Result<(), Error> {
    assert_serde_matches(include_str!(
        "./model/SimpleSparseAccessor/glTF-Embedded/SimpleSparseAccessor.gltf"
    ))
}

#[test]
fn serde_simple_morph() -> Result<(), Error> {
    assert_serde_matches(include_str!(
        "./model/SimpleMorph/glTF-Embedded/SimpleMorph.gltf"
    ))
}

#[test]
fn serde_cameras() -> Result<(), Error> {
    assert_serde_matches(include_str!("./model/Cameras/glTF-Embedded/Cameras.gltf"))
}

#[test]
fn serde_interpolation_test() -> Result<(), Error> {
    assert_serde_matches(include_str!(
        "./model/InterpolationTest/glTF/InterpolationTest.gltf"
    ))
}

#[test]
fn serde_interpolation_test_glb() -> Result<(), Error> {
    assert_deser_glb_matches(
        include_bytes!("./model/InterpolationTest/glTF-Binary/InterpolationTest.glb"),
        include_str!("./model/InterpolationTest/glTF-Binary/InterpolationTestGlb.gltf"),
        include_bytes!("./model/InterpolationTest/glTF/interpolation.bin"),
    )
}

fn assert_serde_matches(json: &str) -> Result<(), Error> {
    let gltf = <GltfAsset>::parse_gltf(json)?;
    let gltf_value = serde_json::to_value(&gltf.gltf)?;
    let expected_value: Value = serde_json::from_str(json)?;

    let config = Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat);
    assert_json_matches!(gltf_value, expected_value, config);
    Ok(())
}

fn assert_deser_glb_matches(glb: &[u8], json: &str, bin: &[u8]) -> Result<(), Error> {
    let gltf = <GltfAsset>::parse_glb(glb)?;
    let gltf_value = serde_json::to_value(&gltf.gltf)?;
    let expected_value: Value = serde_json::from_str(json)?;

    let config = Config::new(CompareMode::Inclusive).numeric_mode(NumericMode::AssumeFloat);
    assert_json_matches!(gltf_value, expected_value, config);

    assert_eq!(gltf.gltf.buffers[0].byte_length, gltf.bin.len());
    assert_eq!(&gltf.bin[0..bin.len()], bin);

    Ok(())
}

#[derive(Debug)]
enum Error {
    ParseError(mugltf::ParseGltfError),
    SerdeError(serde_json::Error),
}

impl From<mugltf::ParseGltfError> for Error {
    fn from(err: mugltf::ParseGltfError) -> Self {
        Error::ParseError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeError(err)
    }
}
