use core::f64;
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = (f64::log10(num as f64) as u32) + 1u32;
    let armstrong: u32 = (1u32..=digits)
        .map(|x| (num / 10u32.pow(x - 1u32) % 10u32).pow(digits))
        .sum();
    armstrong == num
}
