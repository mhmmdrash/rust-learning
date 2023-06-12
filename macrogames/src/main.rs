macro_rules! goob {
    ($s:expr) => {
        println!("goob goob {}", $s)
    }
}

fn main() {
    let a = "cool";
    goob!(a);
}
