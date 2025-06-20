mod dither;

fn main() {

    // creating a fake vector gradient for dither testing purposes, will remove later once we can process the vector with dither algo
    let width = 4;
    let height = 4;
    let buffer = vec![
        0, 64, 128, 192,
        64, 128, 192, 255,
        128, 192, 255, 128,
        192, 255, 128, 64,
    ];

    // lets print the current buffer to ensure we can draw it
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;
            print!("{:>4}", buffer[i])
        }
        println!();
    }

}

