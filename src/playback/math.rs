use fundsp::hacker::*;

#[inline]
pub fn hz_midi<T: Real>(x: T) -> T {
    T::new(69) + T::new(12) * log2(x / T::new(440))
}

#[inline]
pub fn remap<T: Real>(val: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T {
    lerp(out_min, out_max, delerp(in_min, in_max, val))
}

#[inline]
pub fn distort<T: Real>(val: T, factor: T, rescale: T) -> T {
    let x = val * factor;
    let y = if val > T::zero() {
        T::one() - exp(-x)
    } else {
        exp(x) - T::one()
    };
    y / rescale
}
