// use std::marker::PhantomData;

use fundsp::hacker::*;

// #[derive(Default, Clone)]
// pub struct Mapper<T, F, E, I, R>
// where
//     T: Float,
//     F: Float,
//     E: Fn(&Frame<T, I>) -> R + Clone + Send + Sync,
//     I: Size<T>,
//     R: ConstantFrame<Sample = F>,
//     R::Size: Size<F>,
//     R::Size: Size<T>,
// {
//     envelope: E,
//     _marker: PhantomData<I>,
//     _marker2: PhantomData<F>,
//     _marker3: PhantomData<T>,
//     _marker4: PhantomData<R>,
// }

// impl<T, F, E, I, R> Mapper<T, F, E, I, R>
// where
//     T: Float,
//     F: Float,
//     E: Fn(&Frame<T, I>) -> R + Clone + Send + Sync,
//     I: Size<T>,
//     R: ConstantFrame<Sample = F>,
//     R::Size: Size<F>,
//     R::Size: Size<T>,
// {
//     pub fn new(sample_rate: f64, envelope: E) -> Self {
//         let mut node = Self {
//             envelope,
//             _marker: PhantomData,
//             _marker2: PhantomData,
//             _marker3: PhantomData,
//             _marker4: PhantomData,
//         };
//         node.set_sample_rate(sample_rate);
//         node.reset();
//         node
//     }
// }

// impl<T, F, E, I, R> AudioNode for Mapper<T, F, E, I, R>
// where
//     T: Float,
//     F: Float,
//     E: Fn(&Frame<T, I>) -> R + Clone + Send + Sync,
//     I: Size<T>,
//     R: ConstantFrame<Sample = F>,
//     R::Size: Size<F>,
//     R::Size: Size<T>,
// {
//     const ID: u64 = 1000;
//     type Sample = T;
//     type Inputs = I;
//     type Outputs = R::Size;
//     type Setting = ();

//     #[inline]
//     fn tick(
//         &mut self,
//         input: &Frame<Self::Sample, Self::Inputs>,
//     ) -> Frame<Self::Sample, Self::Outputs> {
//         let value: Frame<_, _> = (self.envelope)(input).convert();
//         Frame::generate(|i| convert(value[i]))
//     }
// }

// pub fn mapper<E, R>(
//     f: E,
// ) -> An<Mapper<f64, f64, impl Fn(&Frame<f64, U1>) -> R + Sized + Clone, U1, R>>
// where
//     E: Fn(f64) -> R + Clone + Send + Sync,
//     R: ConstantFrame<Sample = f64>,
//     R::Size: Size<f64>,
// {
//     An(Mapper::new(
//         DEFAULT_SR,
//         move |i: &Frame<f64, U1>| f(i[0]),
//     ))
// }

// pub fn mapper2<E, R>(
//     f: E,
// ) -> An<Mapper<f64, f64, impl Fn(&Frame<f64, U2>) -> R + Sized + Clone, U2, R>>
// where
//     E: Fn(f64, f64) -> R + Clone + Send + Sync,
//     R: ConstantFrame<Sample = f64>,
//     R::Size: Size<f64>,
// {
//     An(Mapper::new(
//         DEFAULT_SR,
//         move |i: &Frame<f64, U2>| f(i[0], i[1]),
//     ))
// }

#[inline]
pub fn hz_midi<T: Real>(x: T) -> T {
    T::new(69) + T::new(12) * log2(x / T::new(440))
}

#[inline]
pub fn remap<T: Real>(val: T, in_min: T, in_max: T, out_min: T, out_max: T) -> T {
    lerp(out_min, out_max, delerp(in_min, in_max, val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hz_midi() {
        for i in 10..100 {
            let midi = i as f64;
            let f = midi_hz(midi);
            let actual = hz_midi(f);
            let epsilon = 1e-6;
            assert!(
                (midi - actual) < epsilon,
                "Expected: {}, Actual: {}",
                midi,
                actual
            )
        }
    }
}
