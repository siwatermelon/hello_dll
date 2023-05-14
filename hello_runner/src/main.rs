#[link(name = "hello.dll",kind="dylib")]
extern {
    fn add(left: usize, right: usize) -> usize;
}

use std::{thread, time};


fn main() {
    let ten_millis = time::Duration::from_millis(100);
    let now = time::Instant::now();
    for i in 0..100 {
        thread::sleep(ten_millis);
    }
    unsafe {
        println!("2+2={}", add(2,2));
    }
    //只需要在这里不断请求web上的dll,读取到内存

}