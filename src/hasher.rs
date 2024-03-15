use crate::traits::*;
use crate::utils::*;

pub struct HashResult(pub [u8; 32]);

impl core::fmt::Display for HashResult {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

#[derive(Debug, Clone)]
pub struct Sha256Hasher {
    state: [u32; 8],
    buffer: [u8; 64],
    state_len: usize,
    buffer_len: usize
}

impl ShaParams for Sha256Hasher {
    const H: [u32; 8] = [ 0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19 ];

    const K: [u32; 64] = [ 0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2 ];
}

impl Sha256Hasher {
    #[allow(unused)]
    pub fn new() -> Self {
        Self {
            state: Self::H, // init state
            buffer: [0u8; 64],
            state_len: 0,
            buffer_len: 0,
        }
    }
}

impl Hasher for Sha256Hasher {
    fn finish(&mut self) -> HashResult {
        // Append the '1' padding bit
        self.buffer[self.buffer_len] = 0x80; //0b10000000
        self.buffer_len += 1;

        if self.buffer_len > 56 {
            self.buffer[56..64].copy_from_slice(&[0u8; 8]);
            self.compress();
            self.buffer_len = 0;
        }

        // padding with zeroes
        self.buffer[self.buffer_len..].fill(0);

        // append with message length
        self.buffer[56..].copy_from_slice(&(self.state_len * 8).to_be_bytes());

        // Final compress 
        self.compress();

        let mut result = [0u8; 32];

        // decomposing u32 to 4 bytes
        for (chunk, value) in result.chunks_exact_mut(4).zip(self.state.iter()) {
            chunk.copy_from_slice(&value.to_be_bytes());
        }

        HashResult(result)
    }

    fn update(&mut self, data: impl AsRef<[u8]>) {
        let data = data.as_ref();
        let mut offset = 0;

        // caching buffer length
        let buffer_len = self.buffer_len;

        // adding data to the remaining buffer space
        if buffer_len > 0 {
            let remaining = 64 - buffer_len;

            if data.len() >= remaining {
                self.buffer[buffer_len..].copy_from_slice(&data[..remaining]);

                self.compress();

                offset += remaining;

                self.buffer_len = 0;

            } else {
                self.buffer[buffer_len..buffer_len + data.len()].copy_from_slice(data);
                self.buffer_len += data.len();
            }
        }

        let chunks = data[offset..].chunks_exact(64);

        // process blocks
        for chunk in chunks.clone() {
            self.buffer.copy_from_slice(chunk);
            self.compress();
        }

        let remaining = chunks.remainder();

        if !remaining.is_empty() {
            self.buffer[..remaining.len()].copy_from_slice(remaining);
            self.buffer_len = remaining.len();
        }

        self.state_len += data.len();
    }
}

impl ShaInternal for Sha256Hasher {
    fn compress(&mut self) {
        let mut w = [0u32; 64];

        // load the message schedule
        for i in 0..16 {
            let offset = i * 4;
            w[i] = u32::from_be_bytes([self.buffer[offset], self.buffer[offset+1], self.buffer[offset+2], self.buffer[offset+3]]);
        }

        // expand the message schedule
        for i in 16..64 {
            w[i] = gamma1(w[i-2]).wrapping_add(w[i - 7]).wrapping_add(gamma0(w[i - 15])).wrapping_add(w[i - 16]);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        // actual compression happens here
        for i in 0..64 {
            let t0 = h.wrapping_add(sigma1(e)).wrapping_add(ch([e, f, g])).wrapping_add(Self::K[i]).wrapping_add(w[i]);
            let t1 = sigma0(a).wrapping_add(maj([a, b, c]));
            d = d.wrapping_add(t0);
            h = t0.wrapping_add(t1);

            let temp = h;
            h = g;
            g = f;
            f = e;
            e = d;
            d = c;
            c = b;
            b = a;
            a = temp;
        }

        // update state
        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
    }
}