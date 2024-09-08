fn main() {
    // Initialize variables for the animation
    let mut a = 0.; // Angle for the first axis
    let mut b = a;  // Angle for the second axis
    print!("\x1b[2J"); // Clear the screen

    loop {
        // Create buffers for rendering the frame
        let (mut r, mut z, mut j, sc) = ([' '; 1760], [0.; 1760], 0., f32::sin_cos);
        
        // Loop through the angles for the animation
        while j < 6.28 { // 6.28 is approximately 2 * PI
            let mut i = 0.;
            while i < 6.28 {
                // Calculate sine and cosine for the angles
                let ((c, l), (f, d), (e, g), (n, m)) = (sc(i), sc(j), sc(a), sc(b));
                let h = d + 2.; // Distance from the viewer
                let (p, t) = (1. / (c * h * e + f * g + 5.), c * h * g - f * e);
                
                // Calculate screen coordinates
                let (x, y) = (
                    (40. + 30. * p * (l * h * m - t * n)) as usize,
                    (12. + 15. * p * (l * h * n + t * m)) as usize
                );
                
                // Calculate the index for the buffer
                let o = x + 80 * y;
                let q = (8. * ((f * e - c * d * g) * m - c * d * e - f * g - l * d * n)) as usize;

                // Update the buffer if the calculated point is within bounds and has a higher value
                if 22 > y && y > 0 && x > 0 && 80 > x && p > z[o] {
                    z[o] = p; // Update depth buffer
                    r[o] = b".,-~:;=!*#$@"[q.max(0)] as char; // Set character based on brightness
                }
                i += 0.02; // Increment angle for the inner loop
            }
            j += 0.07; // Increment angle for the outer loop
        }

        // Print the rendered frame
        print!("\x1b[H"); // Move cursor to the top left
        for k in 0..1761 {
            print!("{}", if k % 80 != 0 { r[k] } else { '\n' }); // Print each character or newline
            a += 4e-5; // Increment angle for the first axis
            b += 2e-5; // Increment angle for the second axis
        }
        // Control the speed of the animation
        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
