
// -> Binary data to readable hexadecimel
// -> handle unspecified errors
// -> secure random number generation
// -> necessary cryptographic functions
// -> create non-zero u32 values for cryptographic operations
use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SystemRandom;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2};
use std::num::NonZeroU32;


// main returns result -> handle potential errors

fn main() -> Result<(), Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN; // 64 bytes
    let n_iter = NonZeroU32::new(100000).unwrap();
    let rng = SystemRandom::new();

    // Generation of salt 
    // salt array of 64 bytes created and filled with secure random bytes
    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;
    
    // Pasword derivation
    // 
    let password = "Who let the dogs out ?";
    let mut store_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut store_hash,
    );

    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("Stored Hash: {}", HEXUPPER.encode(&store_hash));

        // Verification 

        let verify_hash = pbkdf2::verify(
            pbkdf2::PBKDF2_HMAC_SHA512,
            n_iter,
            &salt,
            password.as_bytes(),
            &store_hash,
        );


}