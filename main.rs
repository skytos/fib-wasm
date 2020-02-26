#![no_std]
#![no_main]
#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[export_name = "fib"]
pub extern "C" fn fib(n: i32) -> i32{
    let mut a = 0;
    let mut b = 1;
    let mut i = 1;

    while i < n {
        let t = a;
        a = b;
        b += t;
        i+=1;
    }
    b
}