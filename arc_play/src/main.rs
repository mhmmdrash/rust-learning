use std::sync::Arc;

#[derive(Debug)]
struct Data {
    value: u32,
    name: String
}

impl Data {
    fn new(val: u32, name: String) -> Data {
        Data {
            value: val,
            name: name,
        }
    }
}

fn main() {
    let data = Arc::new(Data::new(32, "Derby".to_string()));
    let thread_1 = Arc::clone(&data);
    let thread_2 = Arc::clone(&data);

    let th1 = std::thread::spawn(move || {
        println!("{:?}", thread_1);
    });

    let th2 = std::thread::spawn(move || {
        println!("{:?}", thread_2);
    });

}