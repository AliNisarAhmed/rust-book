use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

fn main() {
    let a: i64 = 42;
    let a_ptr = &a as *const i64;
    let a_addr: usize = unsafe { std::mem::transmute(a_ptr) };

    println!("a: {} ({:p}..0x{:x})", a, a_ptr, a_addr + 7);

    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
    }
}

// fn main() {
//     let a = 42;
//     let b: String;
//     let c: Cow<str>;

//     unsafe {
//         let b_ptr = &B as *const u8 as *mut u8;
//         b = String::from_raw_parts(b_ptr, 10, 10);

//         let c_ptr = &C as *const u8 as *const c_char;

//         c = CStr::from_ptr(c_ptr).to_string_lossy();
//     }

//     println!("a: {}, b: {}, c: {}", a, b, c);
// }
