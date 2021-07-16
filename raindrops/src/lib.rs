pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    if (n % 3 != 0) && (n % 5 != 0) && (n % 7 != 0) {
        s += n.to_string().as_str();
    }
    if n % 3 == 0 {
        s += "Pling";
    }
    if n % 5 == 0 {
        s += "Plang";
    }
    if n % 7 == 0 {
        s += "Plong";
    }
    s
}
