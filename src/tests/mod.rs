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

/// Test HMAC against NIST test vectors.
pub mod nist_hmac;

/// Test HMAC aginast RFC test vectors.
pub mod rfc_hmac;

/// Test HKDF aginast RFC test vectors.
pub mod rfc_hkdf;

/// Test PBKDF2 aginast RFC test vectors.
pub mod rfc_pbkdf2;

/// Test HMAC aginast custom test vectors.
pub mod custom_pbkdf2;

/// Test cSHAKE against official test vectors from the KeccakCodePackage.
pub mod official_cshake;

/// Test HMAC against IETF Draft test vectors.
pub mod other_hmac;
