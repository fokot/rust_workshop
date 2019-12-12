fn noop<T>(val: T) -> T {
    val
}

fn show_size<T>(_value: T) {
    // std::mem::size_of - function which returns size in Bytes
    println!("The size is {}", std::mem::size_of::<T>());
}

fn main() {
    // by default 0 = 0i32
    let val = noop(0u32);
    show_size(val);
}
