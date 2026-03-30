static mut DATA: i32 = 0;

fn main() {
    let r1 = unsafe { &mut DATA };
    let r2 = unsafe { &mut DATA }; // 🚨 UB

    *r1 += 1;
    *r2 += 1;
}
