use float_dbg::*;

fn main() {
    println!("f32 example");
    0.032_f32.explain();

    println!();

    println!("f64 example");
    0.032_f64.explain();
}
