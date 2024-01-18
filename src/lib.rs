//#![no_std]
#![deny(clippy::pedantic)]
#![deny(warnings)]
#![deny(missing_docs)]
#![allow(dead_code)]
// TODO
//  1. Get one instance working (or at least not erroring)
//  2. check 12-byte adrs fields -- how big is the integer really?
//  3. revisit/clean hash functions
//  4. adrs - store in be or le; how to account for sha2/shake?? (different size)

//! TKTK crate doc

extern crate alloc; // TODO: remove (with vecs)

mod algs;
mod traits;
mod types;

// Per eqns 5.1-4 on page 16, LGW=4, W=16 and LEN2=3 are constant across all parameter sets.
const LGW: u32 = 4;
const W: u32 = 16;


macro_rules! functionality {
    () => {
        #[cfg(test)]
        mod tests {
            use super::*;
            use crate::algs::{slh_keygen_with_rng, slh_sign_with_rng, slh_verify};
            use generic_array::typenum::{Prod, Sum, U2, U3};
            use rand_core::OsRng;

            #[test]
            fn it_works1111() {
                let m = [0u8, 1, 2, 3];
                // TODO: can we push LEN SUM<PROD> calculation downwards? (and remove a generic arg)
                let (sk, pk) =
                    slh_keygen_with_rng::<D, H, HP, Sum<Prod<U2, N>, U3>, N>(&mut OsRng).unwrap();
                let sig = slh_sign_with_rng::<A, D, H, HP, K, Sum<Prod<U2, N>, U3>, M, N>(
                    &mut OsRng, &m, &sk, false,
                )
                .unwrap();
                let result =
                    slh_verify::<A, D, H, HP, K, Sum<Prod<U2, N>, U3>, M, N>(&m, &sig, &pk);
                assert_eq!(result, false);
            }
        }
    };
}

/// TKTK
#[cfg(feature = "slh_dsa_sha2_128s")]
pub mod slh_dsa_sha2_128s {
    use generic_array::typenum::{U12, U14, U16, U30, U63, U7, U9};

    type N = U16;
    type H = U63;
    type D = U7;
    type HP = U9;
    type A = U12;
    type K = U14;
    type M = U30;
    const PK_LEN: usize = 32;
    const SIG_LEN: usize = 7856;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_shake_128s")]
pub mod slh_dsa_shake_128s {
    const N: usize = 16;
    const H: u32 = 63;
    const D: u32 = 7;
    const H_PRIME: u32 = 9;
    const A: u32 = 12;
    const K: u32 = 14;
    const LGW: u32 = 4;
    const M: u32 = 30;
    const PK_LEN: usize = 32;
    const SIG_LEN: usize = 7856;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_sha2_128f")]
pub mod slh_dsa_sha2_128f {
    const N: usize = 16;
    const H: u32 = 66;
    const D: u32 = 22;
    const H_PRIME: u32 = 3;
    const A: u32 = 6;
    const K: u32 = 33;
    const LGW: u32 = 4;
    const M: u32 = 34;
    const PK_LEN: usize = 32;
    const SIG_LEN: usize = 17088;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_shake_128f")]
pub mod slh_dsa_shake_128f {
    const N: usize = 16;
    const H: u32 = 66;
    const D: u32 = 22;
    const H_PRIME: u32 = 3;
    const A: u32 = 6;
    const K: u32 = 33;
    const LGW: u32 = 4;
    const M: u32 = 34;
    const PK_LEN: usize = 32;
    const SIG_LEN: usize = 17088;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_sha2_192s")]
pub mod slh_dsa_sha2_192s {
    const N: usize = 24;
    const H: u32 = 63;
    const D: u32 = 7;
    const H_PRIME: u32 = 9;
    const A: u32 = 14;
    const K: u32 = 17;
    const LGW: u32 = 4;
    const M: u32 = 39;
    const PK_LEN: usize = 48;
    const SIG_LEN: usize = 16224;
    const SK_LEN: usize = 00000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_shake_192s")]
pub mod slh_dsa_shake_192s {
    const N: usize = 24;
    const H: u32 = 63;
    const D: u32 = 7;
    const H_PRIME: u32 = 9;
    const A: u32 = 14;
    const K: u32 = 17;
    const LGW: u32 = 4;
    const M: u32 = 39;
    const PK_LEN: usize = 48;
    const SIG_LEN: usize = 16224;
    const SK_LEN: usize = 00000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_sha2_192f")]
pub mod slh_dsa_sha2_192f {
    const N: usize = 24;
    const H: u32 = 66;
    const D: u32 = 22;
    const H_PRIME: u32 = 3;
    const A: u32 = 8;
    const K: u32 = 33;
    const LGW: u32 = 4;
    const M: u32 = 42;
    const PK_LEN: usize = 48;
    const SIG_LEN: usize = 35664;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_shake_192f")]
pub mod slh_dsa_shake_192f {
    const N: usize = 24;
    const H: u32 = 66;
    const D: u32 = 22;
    const H_PRIME: u32 = 3;
    const A: u32 = 8;
    const K: u32 = 33;
    const LGW: u32 = 4;
    const M: u32 = 42;
    const PK_LEN: usize = 48;
    const SIG_LEN: usize = 35664;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_sha2_256s")]
pub mod slh_dsa_sha2_256s {
    const N: usize = 32;
    const H: u32 = 64;
    const D: u32 = 8;
    const H_PRIME: u32 = 8;
    const A: u32 = 14;
    const K: u32 = 22;
    const LGW: u32 = 4;
    const M: u32 = 47;
    const PK_LEN: usize = 64;
    const SIG_LEN: usize = 29792;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_shake_256s")]
pub mod slh_dsa_shake_256s {
    const N: usize = 32;
    const H: u32 = 64;
    const D: u32 = 8;
    const H_PRIME: u32 = 8;
    const A: u32 = 14;
    const K: u32 = 22;
    const LGW: u32 = 4;
    const M: u32 = 47;
    const PK_LEN: usize = 64;
    const SIG_LEN: usize = 29792;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_sha2_256f")]
pub mod slh_dsa_sha2_256f {
    const N: usize = 32;
    const H: u32 = 68;
    const D: u32 = 17;
    const H_PRIME: u32 = 4;
    const A: u32 = 9;
    const K: u32 = 35;
    const LGW: u32 = 4;
    const M: u32 = 49;
    const PK_LEN: usize = 64;
    const SIG_LEN: usize = 49856;
    const SK_LEN: usize = 0000;

    functionality!();
}

/// TKTK
#[cfg(feature = "slh_dsa_shake_256f")]
pub mod slh_dsa_shake_256f {
    const N: usize = 32;
    const H: u32 = 68;
    const D: u32 = 17;
    const H_PRIME: u32 = 4;
    const A: u32 = 9;
    const K: u32 = 35;
    const LGW: u32 = 4;
    const M: u32 = 49;
    const PK_LEN: usize = 64;
    const SIG_LEN: usize = 49856;
    const SK_LEN: usize = 0000;

    functionality!();
}
