pub(super) struct Crc32 {
    table: [u32; 256],
}

impl Crc32 {
    pub(super) fn new() -> Self {
        let mut table = [0u32; 256];

        for i in 0..256 {
            let mut c = (i << 24) as u32;
            for _ in 0..8 {
                c = if c & 0x80000000 != 0 {
                    0x04c11db7 ^ (c << 1)
                } else {
                    c << 1
                };
            }
            table[i] = c;
        }

        Self { table }
    }

    pub(super) fn checksum(&self, buf: &[u8]) -> u32 {
        let mut c = 0xffffffff;
        for &byte in buf {
            c = (c << 8) ^ self.table[((c >> 24) ^ byte as u32) as usize];
        }
        c
    }
}
