/*
 * file: main.rs
 * minifb wrapper library.
 * provides function for 2-d graphics.
 *
 * Author: Metflekx
 *
 * Manuals:
 *  TODO
 * */

// Represents coordinates of a point
struct Point {
    x: i32,
    y: i32
}

/* Draws a pixel on point(x, y) */
fn draw_pixel(x: i32, y: i32, color: u32, buf: &mut Vec<u32>) {
    let index: usize = y as usize * WIDTH + x as usize;
    if index < buf.len() {
        buf[index as usize] = color;
    }
}

/* calculate where the next initial 
   coordinate of a wrt a should be */
fn next_coordinate(ai: i32, af: i32) -> i32 {
    return if ai < af {ai + 1} else {ai - 1};
}

/* Draws a line from point a to b */
fn line(a: Point, b: Point, slopeRemainder: f64, color: u32, buf: &mut Vec<u32>) -> bool {
    // Declare
    let mut xi: i32 = a.x;
    let xf: i32 = b.x;
    let mut yi: i32 = a.y;
    let yf: i32 = b.y;
    println!("x_i: {} -> x_f{} \ty_i: {} y_f: {}", xi, xf, yi, yf);
    
    // get distance and slope
    let xdistance: i32 = (xf - xi).abs();
    let ydistance: i32 = (yf - yi).abs();

    // Base Case 
    if xdistance < 1 && ydistance < 1 {
        return true; // Line is drawn
    }

    // Slope
    let slope: f64 = (ydistance as f64/ xdistance as f64) + slopeRemainder;
    if xdistance > 0 { // if: x is not done
        draw_pixel(xi, yi, color, buf);
        xi = next_coordinate(xi, xf);
    }

    let mut count: f64 = slope;
    while count >= 1.0 { // if: y should catch up with x
        draw_pixel(xi, yi, color, buf);
        yi = next_coordinate(yi, yf);
        count -= 1.0;
    }

    // recursively call line with new ponit a (src)
    // and old constant b (dst).
    return line(
        Point {x: xi, y: yi}, b,  
        slopeRemainder, color, buf);
}
