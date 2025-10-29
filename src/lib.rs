const MULTIPLIER: i64 = 0x5DEECE66D;
const ADDEND: i64 = 0xB;
const MASK: i64 = (1i64 << 48) - 1;

pub struct JavaRandom {
    seed: i64,
}

impl JavaRandom {
    pub fn new(seed: i64) -> Self {
        Self {
            seed: (seed ^ MULTIPLIER) & MASK,
        }
    }

    fn next(&mut self, bits: u32) -> i32 {
        self.seed = (self.seed.wrapping_mul(MULTIPLIER).wrapping_add(ADDEND)) & MASK;
        (self.seed >> (48 - bits)) as i32
    }

    pub fn next_int(&mut self, bound: i32) -> i32 {
        if bound <= 0 {
            panic!("bound must be positive");
        }

        if bound & (bound - 1) == 0 {
            return ((bound as i64 * self.next(31) as i64) >> 31) as i32;
        }

        loop {
            let bits = self.next(31);
            let val = bits % bound;
            if bits - val + (bound - 1) >= 0 {
                return val;
            }
        }
    }
}

// --------------------
//  Conversión de bloque a chunk
// --------------------
pub fn block_to_chunk(block: i32) -> i32 {
    if block >= 0 {
        block / 16
    } else {
        (block - 15) / 16
    }
}

// --------------------
//  Generación de semilla de chunk EXACTA a Java
// --------------------
fn chunk_seed(world_seed: i64, chunk_x: i32, chunk_z: i32) -> i64 {
    // Esto imita literalmente el código Java:
    // long seed = worldSeed
    //     + (long)(x * x * 4987142) 
    //     + (long)(x * 5947611)
    //     + (long)(z * z * 4392871)
    //     + (long)(z * 389711)
    //     ^ 987234911L;

    let mut s = world_seed;

    s = s
        .wrapping_add((chunk_x as i64).wrapping_mul(chunk_x as i64).wrapping_mul(4987142));
    s = s.wrapping_add((chunk_x as i64).wrapping_mul(5947611));
    s = s
        .wrapping_add((chunk_z as i64).wrapping_mul(chunk_z as i64).wrapping_mul(4392871));
    s = s.wrapping_add((chunk_z as i64).wrapping_mul(389711));

    s ^ 987234911
}

// --------------------
//  Comprobación de slime chunk
// --------------------
pub fn is_slime_chunk(seed: i64, chunk_x: i32, chunk_z: i32) -> bool {
    let s = chunk_seed(seed, chunk_x, chunk_z);
    let mut rng = JavaRandom::new(s);
    rng.next_int(10) == 0
}