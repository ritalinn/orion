// MIT License

// Copyright (c) 2018 brycx

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// Testing against custom test vectors.
// These test vectors have been generated with the cryptography.io Python package.
// More information here: https://github.com/brycx/PBKDF2-HMAC-SHA2-Test-Vectors

#[cfg(test)]
mod custom_test_vectors {

    extern crate hex;
    use self::hex::decode;
    use core::options::ShaVariantOption;
    use hazardous::pbkdf2::Pbkdf2;

    #[test]
    fn sha256_test_case_1() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 1,
            dklen: 20,
            hmac: ShaVariantOption::SHA256,
        };

        let expected_dk = decode("120fb6cffcf8b32c43e7225256c4f837a86548c9").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha256_test_case_2() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 2,
            dklen: 20,
            hmac: ShaVariantOption::SHA256,
        };

        let expected_dk = decode("ae4d0c95af6b46d32d0adff928f06dd02a303f8e").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha256_test_case_3() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 20,
            hmac: ShaVariantOption::SHA256,
        };

        let expected_dk = decode("c5e478d59288c841aa530db6845c4c8d962893a0").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha256_test_case_4() {
        let actual_dk = Pbkdf2 {
            password: "passwordPASSWORDpassword".as_bytes().to_vec(),
            salt: "saltSALTsaltSALTsaltSALTsaltSALTsalt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 25,
            hmac: ShaVariantOption::SHA256,
        };

        let expected_dk = decode("348c89dbcbd32b2f32d814b8116e84cf2b17347ebc1800181c").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha256_test_case_5() {
        let actual_dk = Pbkdf2 {
            password: "pass\0word".as_bytes().to_vec(),
            salt: "sa\0lt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 16,
            hmac: ShaVariantOption::SHA256,
        };

        let expected_dk = decode("89b69d0516f829893c696226650a8687").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha384_test_case_1() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 1,
            dklen: 20,
            hmac: ShaVariantOption::SHA384,
        };

        let expected_dk = decode("c0e14f06e49e32d73f9f52ddf1d0c5c719160923").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha384_test_case_2() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 2,
            dklen: 20,
            hmac: ShaVariantOption::SHA384,
        };

        let expected_dk = decode("54f775c6d790f21930459162fc535dbf04a93918").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha384_test_case_3() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 20,
            hmac: ShaVariantOption::SHA384,
        };

        let expected_dk = decode("559726be38db125bc85ed7895f6e3cf574c7a01c").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha384_test_case_4() {
        let actual_dk = Pbkdf2 {
            password: "passwordPASSWORDpassword".as_bytes().to_vec(),
            salt: "saltSALTsaltSALTsaltSALTsaltSALTsalt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 25,
            hmac: ShaVariantOption::SHA384,
        };

        let expected_dk = decode("819143ad66df9a552559b9e131c52ae6c5c1b0eed18f4d283b").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha384_test_case_5() {
        let actual_dk = Pbkdf2 {
            password: "pass\0word".as_bytes().to_vec(),
            salt: "sa\0lt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 16,
            hmac: ShaVariantOption::SHA384,
        };

        let expected_dk = decode("a3f00ac8657e095f8e0823d232fc60b3").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha512_test_case_1() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 1,
            dklen: 20,
            hmac: ShaVariantOption::SHA512,
        };

        let expected_dk = decode("867f70cf1ade02cff3752599a3a53dc4af34c7a6").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha512_test_case_2() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 2,
            dklen: 20,
            hmac: ShaVariantOption::SHA512,
        };

        let expected_dk = decode("e1d9c16aa681708a45f5c7c4e215ceb66e011a2e").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha512_test_case_3() {
        let actual_dk = Pbkdf2 {
            password: "password".as_bytes().to_vec(),
            salt: "salt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 20,
            hmac: ShaVariantOption::SHA512,
        };

        let expected_dk = decode("d197b1b33db0143e018b12f3d1d1479e6cdebdcc").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha512_test_case_4() {
        let actual_dk = Pbkdf2 {
            password: "passwordPASSWORDpassword".as_bytes().to_vec(),
            salt: "saltSALTsaltSALTsaltSALTsaltSALTsalt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 25,
            hmac: ShaVariantOption::SHA512,
        };

        let expected_dk = decode("8c0511f4c6e597c6ac6315d8f0362e225f3c501495ba23b868").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }

    #[test]
    fn sha512_test_case_5() {
        let actual_dk = Pbkdf2 {
            password: "pass\0word".as_bytes().to_vec(),
            salt: "sa\0lt".as_bytes().to_vec(),
            iterations: 4096,
            dklen: 16,
            hmac: ShaVariantOption::SHA512,
        };

        let expected_dk = decode("9d9e9c4cd21fe4be24d5b8244c759665").unwrap();

        assert_eq!(expected_dk, actual_dk.derive_key().unwrap());
    }
}
