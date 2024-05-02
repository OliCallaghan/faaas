use hmac::{Hmac, Mac};
use md5::Md5;
use rquickjs::{class, methods, module};
use rquickjs::{class::Trace, function::Opt};
use sha2::{Digest, Sha256};

use crate::buffer::Buffer;

#[derive(Clone, Trace)]
#[class]
pub enum Algorithm {
    Sha256,
    Md5,
}

#[methods]
impl Algorithm {
    #[qjs(constructor)]
    pub fn new(alg: String) -> Algorithm {
        match alg.as_str() {
            "md5" => Algorithm::Md5,
            "sha256" => Algorithm::Sha256,
            _ => unimplemented!("algorithm not implemented!"),
        }
    }
}

#[derive(Clone, Trace)]
#[class(rename = "Hash", rename_all = "camelCase")]
pub struct CryptoHash {
    alg: Algorithm,
    data: Vec<u8>,
}

#[methods]
impl CryptoHash {
    #[qjs(constructor)]
    pub fn new(alg: Algorithm) -> Self {
        CryptoHash { alg, data: vec![] }
    }

    #[qjs(rename = "update")]
    pub fn update(&mut self, data: String) -> CryptoHash {
        let mut data_bytes = Vec::from(data.as_bytes());
        self.data.append(&mut data_bytes);

        // TODO: Need a better way of returning `this` from methods.
        self.clone()
    }

    #[qjs(rename = "digest")]
    pub fn digest(&self) -> Buffer {
        let hashed_bytes = match self.alg {
            Algorithm::Md5 => Md5::digest(&self.data).to_vec(),
            Algorithm::Sha256 => Sha256::digest(&self.data).to_vec(),
        };

        Buffer::from_arr(hashed_bytes)
    }

    #[qjs(rename = "digestStr")]
    pub fn digest_str(&self, encoding: String) -> String {
        let digest_buf = self.digest();

        digest_buf.to_string(Opt(Some(encoding)))
    }
}

#[derive(Clone, Trace)]
#[class(rename = "Hmac", rename_all = "camelCase")]
pub struct CryptoHmac {
    alg: Algorithm,
    secret: Vec<u8>,
    data: Vec<u8>,
}

#[methods]
impl CryptoHmac {
    #[qjs(constructor)]
    pub fn new(alg: Algorithm, secret: Vec<u8>) -> Self {
        CryptoHmac {
            alg,
            secret,
            data: vec![],
        }
    }

    #[qjs(rename = "update")]
    pub fn update(&mut self, data: String) -> CryptoHmac {
        let mut data_bytes = Vec::from(data.as_bytes());
        self.data.append(&mut data_bytes);

        // TODO: Need a better way of returning `this` from methods.
        self.clone()
    }

    #[qjs(rename = "digest")]
    pub fn digest(&self) -> Buffer {
        match self.alg {
            Algorithm::Md5 => self.digest_with_md5(),
            Algorithm::Sha256 => self.digest_with_sha256(),
        }
    }

    #[qjs(skip)]
    fn digest_with_md5(&self) -> Buffer {
        type HmacMd5 = Hmac<Md5>;

        let mut hmac = HmacMd5::new_from_slice(&self.secret).unwrap();
        hmac.update(&self.data);

        let hmac_bytes = hmac.finalize().into_bytes();

        Buffer::from_arr(hmac_bytes.to_vec())
    }

    #[qjs(skip)]
    fn digest_with_sha256(&self) -> Buffer {
        type Hmac256 = Hmac<Sha256>;

        let mut hmac = Hmac256::new_from_slice(&self.secret).unwrap();
        hmac.update(&self.data);

        let hmac_bytes = hmac.finalize().into_bytes();

        Buffer::from_arr(hmac_bytes.to_vec())
    }

    #[qjs(rename = "digestStr")]
    pub fn digest_str(&self, encoding: String) -> String {
        let digest_buf = self.digest();

        digest_buf.to_string(Opt(Some(encoding)))
    }
}

#[allow(non_snake_case)] // This is a work around whilst #[function(rename)] macro is broken
#[module]
pub mod crypto_mod {
    use pbkdf2::pbkdf2_hmac;
    use rquickjs::function;
    use sha2::Sha256;

    use crate::bindings;
    use crate::buffer::Buffer;

    pub use super::{Algorithm, CryptoHash, CryptoHmac};

    #[function]
    pub fn createHash(digest: String) -> CryptoHash {
        CryptoHash::new(match digest.as_str() {
            "md5" => Algorithm::Md5,
            "sha256" => Algorithm::Sha256,
            _ => unimplemented!("hash function not implemented"),
        })
    }

    #[function]
    pub fn create_hmac() {
        todo!()
    }

    #[function]
    pub fn randomBytes(len: u64) -> Buffer {
        let rand_bytes = bindings::wasi::random::random::get_random_bytes(len);

        Buffer::from_arr(rand_bytes)
    }

    #[function]
    pub fn pbkdf2Sync(
        password: String,
        salt: String,
        iterations: u32,
        keylen: usize,
        digest: String,
    ) -> Buffer {
        assert_eq!(digest, "sha256");

        let pwd = password.as_bytes();
        let salt = salt.as_bytes();

        let mut key = vec![0u8; keylen];

        pbkdf2_hmac::<Sha256>(pwd, salt, iterations, &mut key);

        Buffer::from_arr(key)
    }
}
