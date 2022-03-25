use crate::Error;

macro_rules! syscall {
    ($($name:ident($a:ident, $($b:ident, $($c:ident, $($d:ident, $($e:ident, $($f:ident, )?)?)?)?)?);)+) => {
        $(
            pub unsafe fn $name($a: usize, $($b: usize, $($c: usize, $($d: usize, $($e: usize, $($f: usize)?)?)?)?)?) -> crate::Result<usize> {
                let ret: usize;

                core::arch::asm!(
                    "svc 0",
                    in("x8") $a,
                    $(
                        in("x0") $b,
                        $(
                            in("x1") $c,
                            $(
                                in("x2") $d,
                                $(
                                    in("x3") $e,
                                    $(
                                        in("x4") $f,
                                    )?
                                )?
                            )?
                        )?
                    )?
                    lateout("x0") ret,
                    options(nostack),
                );

                Error::demux(ret)
            }
        )+
    };
}

syscall! {
    syscall0(a,);
    syscall1(a, b,);
    syscall2(a, b, c,);
    syscall3(a, b, c, d,);
    syscall4(a, b, c, d, e,);
    syscall5(a, b, c, d, e, f,);
}