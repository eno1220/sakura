use core::arch::asm;

#[allow(dead_code)]
struct SbiRet {
    error: i64,
    value: i64,
}

fn sbi_call(
    arg0: i64,
    arg1: i64,
    arg2: i64,
    arg3: i64,
    arg4: i64,
    arg5: i64,
    fid: i64,
    eid: i64,
) -> SbiRet {
    let error: i64;
    let value: i64;
    /* https://doc.rust-jp.rs/rust-by-example-ja/unsafe/asm.html */
    unsafe {
        asm!(
            "ecall",
            inout("a0") arg0 => error,
            inout("a1") arg1 => value,
            inout("a2") arg2 => _,
            inout("a3") arg3 => _,
            inout("a4") arg4 => _,
            inout("a5") arg5 => _,
            inout("a6") fid => _,
            inout("a7") eid => _,
        );
    }
    SbiRet { error, value }
}

pub fn putchar(c: u8) {
    sbi_call(c as i64, 0, 0, 0, 0, 0, 0, 1);
}
