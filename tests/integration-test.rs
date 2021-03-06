extern crate akaze;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate image;
use std::path::PathBuf;
use std::time::SystemTime;

use akaze::types::evolution::{write_evolutions, Config};
use akaze::types::feature_match::draw_matches;
use akaze::types::keypoint::draw_keypoints_to_image;

/// Test data is included with this repository. This
/// function helps find it for testing.
fn locate_test_data() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-data")
}

#[test]
fn test_locate_data() {
    warn!(
        "Note: test data can be obtained from the akaze-test-data
        repository See README.md"
    );
    let test_data_path = locate_test_data();
    let mut image_file_path = test_data_path;
    image_file_path.push("1.jpg");
    let metadata = ::std::fs::metadata(image_file_path).unwrap();
    debug_assert!(metadata.is_file());
    let test_data_path = locate_test_data();
    let mut image_file_path = test_data_path;
    image_file_path.push("2.jpg");
    let metadata = ::std::fs::metadata(image_file_path).unwrap();
    debug_assert!(metadata.is_file());
}

#[test]
fn extract_features() {
    let start = SystemTime::now();
    let env = env_logger::Env::default().filter_or("AKAZE_LOG", "debug");
    env_logger::Builder::from_env(env).init();
    let mut test_image_path = locate_test_data();
    test_image_path.push("1.jpg");
    let options = Config::default();
    let (evolutions, keypoints, _) = akaze::extract_features(test_image_path.clone(), options);
    match std::env::var("AKAZE_SCALE_SPACE_DIR") {
        Ok(val) => {
            info!("Writing scale space; if you want to skip this step, undefine the env var AKAZE_SCALE_SPACE_DIR");
            let string_to_pass = val;
            let path_to_scale_space_dir = std::path::Path::new(&string_to_pass.clone()).to_owned();
            std::fs::create_dir_all(&string_to_pass).unwrap();
            write_evolutions(&evolutions, path_to_scale_space_dir.clone());
            let mut input_image = image::open(test_image_path).unwrap().to_rgb();
            draw_keypoints_to_image(&mut input_image, &keypoints);
            let mut path_to_keypoint_image = path_to_scale_space_dir;
            path_to_keypoint_image.push("keypoints.png");
            match input_image.save(path_to_keypoint_image) {
                Ok(_val) => debug!("Wrote keypoint image successfully."),
                Err(_e) => debug!("Could not write keypoint image for some reason, skipping."),
            }
        }
        Err(_e) => {
            info!("Not writing scale space; do write scale space, define the env var AKAZE_SCALE_SPACE_DIR");
        }
    }
    info!("Total duration: {:?}", start.elapsed().unwrap());
}

#[test]
fn match_features() {
    debug!("Setting up matching test by extracting features from 2 images.");
    let mut test_image_path_0 = locate_test_data();
    test_image_path_0.push("1.jpg");
    let mut test_image_path_1 = locate_test_data();
    test_image_path_1.push("2.jpg");
    let options = Config::default();
    let (_evolutions_0, keypoints_0, descriptors_0) =
        akaze::extract_features(test_image_path_0.clone(), options);
    let (_evolutions_1, keypoints_1, descriptors_1) =
        akaze::extract_features(test_image_path_1.clone(), options);
    debug!("Beginning matching process.");
    let matches = akaze::match_features(
        &keypoints_0,
        &descriptors_0,
        &keypoints_1,
        &descriptors_1,
        0.86,
        1000,
        3.0,
    );
    info!("Got {} matches.", matches.len());
    let start = SystemTime::now();
    match std::env::var("AKAZE_SCALE_SPACE_DIR") {
        Ok(val) => {
            info!("Writing scale space; if you want to skip this step, undefine the env var AKAZE_SCALE_SPACE_DIR");
            let string_to_pass = val;
            let path_to_scale_space_dir = std::path::Path::new(&string_to_pass.clone()).to_owned();
            std::fs::create_dir_all(&string_to_pass).unwrap();
            let input_image_0 = image::open(test_image_path_0).unwrap().to_rgb();
            let input_image_1 = image::open(test_image_path_1).unwrap().to_rgb();
            let match_image = draw_matches(
                &input_image_0,
                &input_image_1,
                &keypoints_0,
                &keypoints_1,
                &matches,
            );
            let mut path_to_matches_image = path_to_scale_space_dir;
            path_to_matches_image.push("matches.png");
            match match_image.save(path_to_matches_image) {
                Ok(_val) => debug!("Wrote matches image successfully."),
                Err(_e) => debug!("Could not write matches image for some reason, skipping."),
            }
        }
        Err(_e) => {
            info!("Not writing scale space; do write scale space, define the env var AKAZE_SCALE_SPACE_DIR");
        }
    }
    info!("Total duration: {:?}", start.elapsed().unwrap());
}
