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

pub fn dither_bayer(buffer: &mut [u8], height: usize, width: usize) {
    let raw_matrix = [[0, 8, 2, 10], [12, 4, 14, 6], [3, 11, 1, 9], [15, 7, 13, 5]];

    // for loops used to iterate over all positions of the buffer
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            // tile the bayer matrix across the image
            let matrix_x = x % 4;
            let matrix_y = y % 4;

            // look up raw threshold vals
            let raw_value: f64 = raw_matrix[matrix_y][matrix_x] as f64;

            // normalize val within range 0-255 to get a threshold in the same scale as pixel vals
            // had to manually cast scale as an f64, there is probably a better way to do this
            let scale: f64 = 255.0 / 16.0;
            let normalized_threshold: f64 = (raw_value + 0.5) * (scale);
            //compare pixel against threshold
            if buffer[i] as f64 > normalized_threshold {
                buffer[i] = 255;
            } else {
                buffer[i] = 0;
            }
        }
    }
}

// goal here is to add other forms of dithering and allow user to select algos, but this works for now
// pub fn ordered_dither(buffer: &mut [u8], height: usize, width:usize)

// unit tests, dood. very basic for now. testing thresholding
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dither_threshold() {
        let width = 4;
        let height = 1;
        let mut buffer = vec![64, 128, 192, 255];
        let expected = vec![0, 255, 255, 255];

        dither_threshold(&mut buffer, height, width, 127);

        assert_eq!(buffer, expected);
    }

    #[test]
    fn test_dither_bayer() {
        let width = 4;
        let height = 4;
        let mut buffer = vec![
            128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128, 128,
        ];
        let expected = vec![
            0, 255, 0, 255, 255, 0, 255, 0, 0, 255, 0, 255, 255, 0, 255, 0,
        ];
        dither_bayer(&mut buffer, height, width);

        assert_eq!(buffer, expected);
    }
    // we could use some more tests for different dimensions as well
}
