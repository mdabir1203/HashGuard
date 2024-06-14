# Ultimate Guide to Secure password hashing in Rust
## Introduction
Ensuring the security of user passwords is paramount importance in today's digital world. One of the most effective methods for securing passwords is through hashing, specifically using the PBKDF2 (Password-Based Key Derivation Function 2) algorithm. In this comprehensive guide, we'll dive into secure password hashing and verification using Rust and PBKDF2, making security easier for developers of all levels to understand and implement.

## What is Password Hashing and Why is it Important?
Password hashing is the process of transforming a plain text password into a secure, fixed-size string of characters. This transformation makes it nearly impossible to revert the hash back to the original password, ensuring the safety of user credentials even if the database is compromised.

## Key reasons for password hashing:

**Security:** Protects passwords from being easily deciphered if the data is breached.

**Salting:** Adds an additional layer of security by appending a random string to the password before hashing.

**Iteration Count:** Increases the complexity and time required to brute-force the hash.


## Demo 

https://github.com/mdabir1203/Hash_Rust/assets/66947064/4b96e439-be18-4bca-82a6-e9625e734aba

### Question for Thought 

1. How to securely hash a password?
2. How to verify a password against a stored hash?
3. How to handle random number generation securely?
4. How to ensure non-zero iteration count?
5. What is the purpose of the HEXUPPER encoding in this code?
6. How does the SystemRandom function ensure secure random number generation?
7. How does the pbkdf2::derive function work to create a hashed password?
8. What is the purpose of the validate_password function?
9. Why is it checking for a password length of less than 2?
10. How does the pbkdf2::verify function verify the hashed password?
11. Why is the Unspecified error from the ring crate used in this code?
12. What would happen if the unwrap() function encounters an error during execution?
13. Why is the salt generated with a length equal to SHA512_OUTPUT_LEN?
14. What is the purpose of the commented out code related to wrong_password and its verification?
