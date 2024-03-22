use crate::hasher::HashResult;

pub trait Hasher {
    fn update(&mut self, data: impl AsRef<[u8]>);

    fn finish(&mut self) -> HashResult;
}

pub(crate) trait ShaInternal: Hasher {
    fn expand(&self) -> [u32; 64];
    
    fn compress(&mut self);
}

pub(crate) trait ShaParams: Hasher {
    const K: [u32; 64];

    const H: [u32; 8];
}