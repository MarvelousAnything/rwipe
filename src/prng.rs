pub struct RwipeEntropy {
    length: usize,
    buffer: *mut u8
}

pub trait PrngState {
    fn init(&mut self, seed: &mut RwipeEntropy);
    fn read(&mut self, buffer: &mut [u8]);
}

pub struct Prng<'a> {
    label: &'a str,
    state: Box<dyn PrngState + 'a>
}
