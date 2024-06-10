
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

