# A library to generate inputs for zero knowledge circuits requiring ecdsa (secp256k1) signatures

## Motivation behind this library
I created this library as a contribution to the [Noir ecosystem](https://github.com/noir-lang/noir).

When working through Noir's [documentation on ECDSA Signature Verification](https://noir-lang.org/standard_library/cryptographic_primitives/ecdsa_sig_verification), I realized that input generation for circuits that deal with cryptography is not at all a trivial task. This motivated me to create a rust library that can reliably generate inputs for any message signature by any signing key.

This library enables the use of ecdsa signatures over the secp256k1 curve within noir circuits. An example circuit can be found in `circuits/secp256k1/main.nr`.

## Example circuit

To run the example circuit, you must have Noir's nargo client installed.

Run `nargo test` to verify the default inputs, or create your own using the library.

Default inputs:

```
    main(
        # message
        [198, 88, 243, 122, 71, 77, 168, 42, 143, 171, 247, 244, 65, 43, 39, 144, 191, 174, 225, 24, 216, 74, 135, 40, 228, 251, 255, 57, 93, 20, 78, 166],
        # x-coordinate
        [37, 0, 163, 241, 87, 78, 49, 199, 229, 105, 41, 14, 72, 76, 141, 136, 233, 90, 222, 14, 58, 82, 150, 61, 82, 90, 139, 189, 82, 226, 157, 193],
        # y-coordinate
        [205, 63, 220, 81, 232, 75, 187, 88, 56, 193, 146, 31, 146, 124, 241, 8, 139, 253, 27, 154, 169, 4, 229, 152, 1, 160, 118, 112, 16, 67, 173, 183],
        # signature
        [149, 185, 187, 32, 142, 0, 74, 15, 89, 141, 78, 29, 54, 176, 39, 209, 212, 197, 80, 119, 77, 161, 32, 71, 237, 182, 150, 148, 138, 20, 243, 254, 9, 145, 20, 177, 253, 187, 73, 185, 70, 248, 125, 32, 238, 11, 136, 48, 46, 53, 202, 101, 187, 91, 65, 79, 1, 23, 8, 246, 84, 130, 37, 254]
    )
```

## Table of contents

|        Key manager             |              Input Generator                  |       StoreManager             |
|--------------------------------|-----------------------------------------------|--------------------------------|
| Creates new keys               | Derives x and y coordinate from public key    | Saves a key in a mysqlite db   |
| Deserializes ecdsa SigningKeys | Signs any message with any key                | Retrieves a key by uid from db |

## Examples

An example can be found in `lib.rs`.
