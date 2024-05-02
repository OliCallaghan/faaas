use rquickjs::module;

#[allow(non_snake_case)] // This is a work around whilst #[function(rename)] macro is broken
#[module]
pub mod crypto_mod {
    use pbkdf2::pbkdf2_hmac;
    use rquickjs::function;
    use sha2::Sha256;

    use crate::bindings;
    use crate::buffer::Buffer;

    // #[function]
    // pub fn create_hash(digest: String) -> Box<dyn Hash> {
    //     assert_eq!(digest, "sha256");
    // }

    // #[function]
    // pub fn create_hmac() {
    //     todo!()
    // }

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
