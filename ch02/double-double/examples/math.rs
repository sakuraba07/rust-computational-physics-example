use simba::scalar::{ComplexField, RealField};
use xprec::Df64;

fn main() {
    let x = Df64::from(2.0);

    // 平方根
    let sqrt_x = x.sqrt();
    println!("sqrt(2) = {}", sqrt_x);

    // べき乗
    let x_cubed = x.powi(3);
    println!("2^3 = {}", x_cubed);

    // 高精度のπ
    let pi: Df64 = Df64::pi();
    println!("π (Df64) = {}", pi);
    println!("π (f64)  = {}", std::f64::consts::PI);

    // 三角関数
    let sin_pi = pi.sin();
    println!("sin(π) = {}", sin_pi);
}
