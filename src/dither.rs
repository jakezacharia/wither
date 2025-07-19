// threshold based dithering for grayscale - very basic implementation
pub fn dither_threshold(buffer: &mut [u8], height: usize, width: usize, threshold: u8) {
    // iterate through each value in the vector
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            // check current pixel against threshold value, set accordingly
            if buffer[i] > threshold {
                buffer[i] = 255;
            } else {
                buffer[i] = 0;
            }
        }
    }
}

// bayer matrix dithering implementation
pub fn dither_bayer(buffer: &mut [u8], height: usize, width: usize, matrix_dimension: usize)  {
    // larger matrices provide finer dithering patterns and better smoothing

    let bayer_matrix = [[0, 2],[3,1]];

    // bayer 4x4
    // let bayer_matrix = [[0, 8, 2, 10],
    //                     [12, 4, 14, 6],
    //                     [3, 11, 1, 9],
    //                     [15, 7, 13, 5]];

    // let bayer_matrix = threshhold_map / 2

    // loop over entire image and apply bayer matrix
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;

            let matrix_x = x % matrix_dimension;
            let matrix_y = y % matrix_dimension;

            let bayer_value: f64 = bayer_matrix[matrix_y][matrix_x] as f64;

            let scale: f64 = 255.0 / (matrix_dimension as f64);
            const ROUNDING_AMOUNT: f64 = 0.5;

            let normalized_threshold: f64 = (bayer_value + ROUNDING_AMOUNT) * (scale);

            if buffer[i] as f64 > normalized_threshold {
                buffer[i] = 255;
            } else {
                buffer[i] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dither_threshold() {
        let width = 4;
        let height = 1;
        let mut buffer = vec![64, 128, 192, 255];
        let expected = vec![0, 255, 255, 255];

        dither_threshold(&mut buffer, height, width, 127 );

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_dither_bayer_2x2() {
        let mut buffer = vec![255, 127, 129, 255];
        let expected = vec![255, 0, 0, 255];

        dither_bayer(&mut buffer, 2, 2, 2);

        assert_eq!(buffer, expected);
    }
    #[test]
    fn test_dither_bayer_4x4() {
        let mut buffer = vec![255, 127, 129, 255,
                                        255, 127, 129, 255,
                                        255, 127, 129, 255,
                                        255, 127, 129, 255];
        let expected = vec![255, 0, 0, 255,
                                    255, 0, 0, 255,
                                    255, 0, 0, 255,
                                    255, 0, 0, 255];

        dither_bayer(&mut buffer, 4, 4, 2);

        assert_eq!(buffer, expected);
    }

    // we could use some more tests for different dimensions as well
}
