use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v_clone = v.clone();

    let handle1 = std::thread::spawn(move || {
        let mut v = v_clone.lock().unwrap();
        *v.get_mut(0).unwrap() = 4;
    });

    let handle2 = std::thread::spawn(move || {
        let v = v.lock().unwrap();
        println!("The first element is: {}", v[0]);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}