fn main() {
    let b = Box::new(42);              // allocazione sullo heap
    let p = Box::into_raw(b);          // otteniamo un raw pointer

    unsafe {
        drop(Box::from_raw(p));        // liberiamo la memoria
        println!("{}", *p);            // uso del puntatore dopo il free
    }
}