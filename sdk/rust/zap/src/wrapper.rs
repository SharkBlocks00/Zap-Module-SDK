use std::{panic, slice};

use zap_sdk::ZapValue;

use crate::{FromZapValue, IntoZapValue, Result, ZapError};

#[inline]
unsafe fn arguments<'a>(args: *const ZapValue, argc: u32) -> &'a [ZapValue] {
    slice::from_raw_parts(args, argc as usize)
}

#[inline]
fn check_argc(expected: usize, found: usize) -> Result<()> {
    if expected == found {
        Ok(())
    } else {
        Err(ZapError::InvalidArgumentCount { expected, found })
    }
}

#[inline]
fn catch<R>(f: impl FnOnce() -> Result<R>) -> ZapValue
where
    R: IntoZapValue,
{
    match panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(Ok(value)) => value.into_zap(),

        Ok(Err(err)) => {
            eprintln!("{err}");
            ZapValue::null()
        }

        Err(_) => {
            eprintln!("native module panicked");
            ZapValue::null()
        }
    }
}

pub unsafe fn invoke0<R>(args: *const ZapValue, argc: u32, function: fn() -> R) -> ZapValue
where
    R: IntoZapValue,
{
    catch(|| {
        check_argc(0, argc as usize)?;

        let value = function();

        Ok(value)
    })
}

pub unsafe fn invoke1<A, R>(args: *const ZapValue, argc: u32, function: fn(A) -> R) -> ZapValue
where
    A: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(1, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;

        Ok(function(a))
    })
}

pub unsafe fn invoke2<A, B, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(2, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;

        Ok(function(a, b))
    })
}

pub unsafe fn invoke3<A, B, C, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(3, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;

        Ok(function(a, b, c))
    })
}

pub unsafe fn invoke4<A, B, C, D, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(4, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;

        Ok(function(a, b, c, d))
    })
}

pub unsafe fn invoke5<A, B, C, D, E, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(5, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;

        Ok(function(a, b, c, d, e))
    })
}

pub unsafe fn invoke6<A, B, C, D, E, F, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(6, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;

        Ok(function(a, b, c, d, e, f))
    })
}

pub unsafe fn invoke7<A, B, C, D, E, F, G, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(7, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;

        Ok(function(a, b, c, d, e, f, g))
    })
}

pub unsafe fn invoke8<A, B, C, D, E, F, G, H, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(8, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;

        Ok(function(a, b, c, d, e, f, g, h))
    })
}

pub unsafe fn invoke9<A, B, C, D, E, F, G, H, I, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H, I) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    I: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(9, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;
        let i = I::from_zap(&args[8])?;

        Ok(function(a, b, c, d, e, f, g, h, i))
    })
}

pub unsafe fn invoke10<A, B, C, D, E, F, G, H, I, J, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H, I, J) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    I: FromZapValue,
    J: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(10, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;
        let i = I::from_zap(&args[8])?;
        let j = J::from_zap(&args[9])?;

        Ok(function(a, b, c, d, e, f, g, h, i, j))
    })
}

pub unsafe fn invoke11<A, B, C, D, E, F, G, H, I, J, K, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H, I, J, K) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    I: FromZapValue,
    J: FromZapValue,
    K: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(11, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;
        let i = I::from_zap(&args[8])?;
        let j = J::from_zap(&args[9])?;
        let k = K::from_zap(&args[10])?;

        Ok(function(a, b, c, d, e, f, g, h, i, j, k))
    })
}

pub unsafe fn invoke12<A, B, C, D, E, F, G, H, I, J, K, L, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H, I, J, K, L) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    I: FromZapValue,
    J: FromZapValue,
    K: FromZapValue,
    L: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(12, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;
        let i = I::from_zap(&args[8])?;
        let j = J::from_zap(&args[9])?;
        let k = K::from_zap(&args[10])?;
        let l = L::from_zap(&args[11])?;

        Ok(function(a, b, c, d, e, f, g, h, i, j, k, l))
    })
}

pub unsafe fn invoke13<A, B, C, D, E, F, G, H, I, J, K, L, M, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H, I, J, K, L, M) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    I: FromZapValue,
    J: FromZapValue,
    K: FromZapValue,
    L: FromZapValue,
    M: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(13, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;
        let i = I::from_zap(&args[8])?;
        let j = J::from_zap(&args[9])?;
        let k = K::from_zap(&args[10])?;
        let l = L::from_zap(&args[11])?;
        let m = M::from_zap(&args[12])?;

        Ok(function(a, b, c, d, e, f, g, h, i, j, k, l, m))
    })
}

pub unsafe fn invoke14<A, B, C, D, E, F, G, H, I, J, K, L, M, N, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    I: FromZapValue,
    J: FromZapValue,
    K: FromZapValue,
    L: FromZapValue,
    M: FromZapValue,
    N: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(14, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;
        let i = I::from_zap(&args[8])?;
        let j = J::from_zap(&args[9])?;
        let k = K::from_zap(&args[10])?;
        let l = L::from_zap(&args[11])?;
        let m = M::from_zap(&args[12])?;
        let n = N::from_zap(&args[13])?;

        Ok(function(a, b, c, d, e, f, g, h, i, j, k, l, m, n))
    })
}

pub unsafe fn invoke15<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    I: FromZapValue,
    J: FromZapValue,
    K: FromZapValue,
    L: FromZapValue,
    M: FromZapValue,
    N: FromZapValue,
    O: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(15, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;
        let i = I::from_zap(&args[8])?;
        let j = J::from_zap(&args[9])?;
        let k = K::from_zap(&args[10])?;
        let l = L::from_zap(&args[11])?;
        let m = M::from_zap(&args[12])?;
        let n = N::from_zap(&args[13])?;
        let o = O::from_zap(&args[14])?;

        Ok(function(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o))
    })
}

pub unsafe fn invoke16<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, R>(
    args: *const ZapValue,
    argc: u32,
    function: fn(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P) -> R,
) -> ZapValue
where
    A: FromZapValue,
    B: FromZapValue,
    C: FromZapValue,
    D: FromZapValue,
    E: FromZapValue,
    F: FromZapValue,
    G: FromZapValue,
    H: FromZapValue,
    I: FromZapValue,
    J: FromZapValue,
    K: FromZapValue,
    L: FromZapValue,
    M: FromZapValue,
    N: FromZapValue,
    O: FromZapValue,
    P: FromZapValue,
    R: IntoZapValue,
{
    catch(|| {
        check_argc(16, argc as usize)?;

        let args = arguments(args, argc);

        let a = A::from_zap(&args[0])?;
        let b = B::from_zap(&args[1])?;
        let c = C::from_zap(&args[2])?;
        let d = D::from_zap(&args[3])?;
        let e = E::from_zap(&args[4])?;
        let f = F::from_zap(&args[5])?;
        let g = G::from_zap(&args[6])?;
        let h = H::from_zap(&args[7])?;
        let i = I::from_zap(&args[8])?;
        let j = J::from_zap(&args[9])?;
        let k = K::from_zap(&args[10])?;
        let l = L::from_zap(&args[11])?;
        let m = M::from_zap(&args[12])?;
        let n = N::from_zap(&args[13])?;
        let o = O::from_zap(&args[14])?;
        let p = P::from_zap(&args[15])?;

        Ok(function(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p))
    })
}
