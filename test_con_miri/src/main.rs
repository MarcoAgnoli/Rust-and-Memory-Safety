fn main() {
    let v: Vec<i32> = vec![1, 2, 3];
    let ptr = v.as_ptr();
    drop(v); // dealloca la memoria
    let val = unsafe { *ptr }; // ⚠️ use-after-free!
    println!("valore: {}", val);
}