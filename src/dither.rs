
// threshold based dithering for grayscale - very basic implementation
pub fn dither_threshold(buffer: &mut [u8], height: usize, width:usize, threshold: u8) {
  // iterate through each value in the vector
    for y in 0..height {
        for x in 0..width  {
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
  fn test_dither_threshold_2() {
    let width = 4;
    let height = 4;
    let mut buffer = vec![
        0, 64, 128, 192,
        64, 128, 192, 255,
        128, 192, 255, 128,
        192, 255, 128, 64,
    ];
    let expected = vec![
        0, 0, 255, 255,
        0, 255, 255, 255,
        255, 255, 255, 255,
        255, 255, 255, 0,
    ];
    dither_threshold(&mut buffer, height, width, 127);

    assert_eq!(buffer, expected);
  }
  // we could use some more tests for different dimensions as well

}