
// threshold based dithering, very basic implementation
// goal here is to add other forms of dithering and allow user to select algos, but this works for now
pub fn dither(buffer: &mut [u8], height: usize, width:usize, threshold: u8) {
  // iterate through each value in the vector
    for y in 0..height {
        for x in 0..width  {
            let i = y * width + x;
            if buffer[i] > threshold {
              buffer[i] = 255;
            } else {
              buffer[i] = 0;
            }
        }
    }
}


// unit tests, dood. very basic for now. testing thresholding
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dither_threshold() {
    // predefining a test buffer to run our dither algo
    let width = 4;
    let height = 1;
    let mut buffer = vec![64, 128, 192, 255];
    let expected = vec![0, 255, 255, 255];

    dither(&mut buffer, height, width, 127);

    assert_eq!(buffer, expected);
  }
}