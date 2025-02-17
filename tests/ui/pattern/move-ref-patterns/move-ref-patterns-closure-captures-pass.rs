// check-pass

#![allow(drop_ref)]

fn main() {
    struct U;
    fn accept_fn_once(_: impl FnOnce()) {}
    fn accept_fn_mut(_: impl FnMut()) {}
    fn accept_fn(_: impl Fn()) {}

    let mut tup = (U, U, U);
    let (ref _x0, _x1, ref mut _x2) = tup;
    let c1 = || {
        drop::<&U>(_x0);
        drop::<U>(_x1);
        drop::<&mut U>(_x2);
    };
    accept_fn_once(c1);

    let c2 = || {
        drop::<&U>(_x0);
        drop::<&mut U>(_x2);
    };
    accept_fn_mut(c2);

    let c3 = || {
        drop::<&U>(_x0);
    };
    accept_fn(c3);
}
