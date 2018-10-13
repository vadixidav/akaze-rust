use ops::estimate_fundamental_matrix::remove_outliers;
use types::feature_match::Match;
use types::keypoint::Descriptor;
use types::keypoint::Keypoint;
use time::PreciseTime;
/// Match two sets of keypoints and descriptors. The
/// Hamming distance is used to determine the matches,
/// and a brute force algorithm is used to get the
/// best matches.
///
/// Matching is performed only in the forward direction,
/// and no geometric verification such as planar homographies or
/// RANSAC is used. We apply Lowe's ratio and remove successful
/// matches in the forward direction just to avoid having too
/// many matches to deal with and visualize, and also to speed
/// up matching time.
///
/// TODO: RANSAC and/or homographies. The current results are
/// not sufficient.
///
/// `descriptors_0` The first set of descriptors.
/// `descriptors_1` The second set of desctiptors.
/// `distance_threshold` The distance threshold below which
/// to accept a match.
/// # Return value
/// A vector of matches.
pub fn descriptor_match(
    descriptors_0: &Vec<Descriptor>,
    descriptors_1: &Vec<Descriptor>,
    distance_threshold: usize,
    lowes_ratio: f64,
) -> Vec<Match> {
    let start = PreciseTime::now();
    let mut output: Vec<Match> = vec![];
    let mut filtered_by_threshold = 0;
    let mut mean = 0.;
    let mut max = 0.;
    let mut min = std::f64::MAX;

    for (i, d0) in descriptors_0.iter().enumerate() {
        let mut min_distance = distance_threshold;
        let mut min_j = 0;
        let mut second_to_min_distance = min_distance;
        for (j, d1) in descriptors_1.iter().enumerate() {
            let distance = hamming_distance(d0, d1, second_to_min_distance);
            if distance < min_distance {
                second_to_min_distance = min_distance;
                min_distance = distance;
                min_j = j;
            } else {
                if distance < second_to_min_distance {
                    second_to_min_distance = distance;
                }
            }
        }
        // apply thresholding and Lowe's ratio
        if (min_distance as f64) < (second_to_min_distance as f64) * lowes_ratio {
            if min_distance < (distance_threshold as usize) {
                output.push(Match {
                    index_0: i,
                    index_1: min_j,
                    distance: min_distance as f64,
                });
                mean += min_distance as f64;
                if (min_distance as f64) < min {
                    min = min_distance as f64;
                }
                if (min_distance as f64) > max {
                    max = min_distance as f64;
                }
            } else {
                filtered_by_threshold += 1;
            }
        }
    }
    mean /= (filtered_by_threshold + output.len()) as f64;
    debug!(
        "{} matches, {} filtered, dist min={}, mean={}, max={}, took {}.",
        output.len(),
        filtered_by_threshold,
        min,
        mean,
        max,
        start.to(PreciseTime::now()),
    );
    output
}

pub fn ransac_match(
    keypoints_0: &Vec<Keypoint>,
    descriptors_0: &Vec<Descriptor>,
    keypoints_1: &Vec<Keypoint>,
    descriptors_1: &Vec<Descriptor>,
) -> Vec<Match> {
    // 50usize is a level such that no plausible matches will be filtered - effectively
    // turning this off
    let distance_threshold = 50usize;
    // Take all matches that pass Lowe's ratio.
    let mut output = descriptor_match(&descriptors_0, descriptors_1, distance_threshold, 0.7);
    let inliers = remove_outliers(
        &keypoints_0,
        &keypoints_1,
        &mut output,
        10000,
        0.05f32,
        0.25f32,
    );
    inliers
}

/// The Hamming distance between two descriptors.
/// Ex.
/// 0100100
/// 0100000
/// Hamming distance = 1: 1 bit position differs
/// `d0` the first descriptor.
/// `d1` the second descriptor.
/// # Return value
/// The Hamming distance
fn hamming_distance(
    d0: &Descriptor, d1: &Descriptor,
    bailout_distance: usize) -> usize {
    let mut distance = 0usize;
    for it in d0.vector.iter().zip(d1.vector.iter()) {
        let (x0, x1) = it;
        distance += (*x0 ^ *x1).count_ones() as usize;
        if distance > bailout_distance {
            break;
        }
    }
    distance as usize
}
