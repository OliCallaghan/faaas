use rquickjs::class::Trace;
use rquickjs::{class, function::Opt, methods, module};

#[derive(Trace)]
#[class(rename_all = "camelCase")]
pub struct Buffer {
    data: Vec<u8>,
}

#[methods]
impl Buffer {
    // Deprecated
    #[qjs(constructor)]
    pub fn new() -> Self {
        Buffer { data: vec![] }
    }

    #[qjs(static, rename = "fromBuffer")]
    pub fn from_buf(buf: &Buffer) -> Self {
        Buffer {
            data: buf.data.clone(),
        }
    }

    #[qjs(static, rename = "fromArray")]
    pub fn from_arr(arr: Vec<u8>) -> Self {
        Buffer { data: arr }
    }

    #[qjs(static, rename = "fromString")]
    pub fn from_str(str: String) -> Self {
        Buffer {
            data: str.into_bytes(),
        }
    }

    #[qjs(skip)]
    fn get_offset(&self, offset: usize) -> &[u8] {
        &self.data[offset..]
    }

    #[qjs(skip)]
    fn get_bytes<const N: usize>(&self, offset: usize) -> [u8; N] {
        self.get_offset(offset)[..N].try_into().unwrap()
    }

    #[qjs(rename = "readUInt32BE")]
    pub fn read_uint32be(&self, offset: Opt<usize>) -> u32 {
        u32::from_be_bytes(self.get_bytes(offset.unwrap_or(0)))
    }

    #[qjs(rename = "readUInt32LE")]
    pub fn read_uint32le(&self, offset: Opt<usize>) -> u32 {
        u32::from_le_bytes(self.get_bytes(offset.unwrap_or(0)))
    }

    #[qjs(rename = "readUInt16BE")]
    pub fn get_uint16be(&self, offset: Opt<usize>) -> u16 {
        u16::from_be_bytes(self.get_bytes(offset.unwrap_or(0)))
    }

    #[qjs(rename = "readUInt16LE")]
    pub fn get_uint16le(&self, offset: Opt<usize>) -> u16 {
        u16::from_le_bytes(self.get_bytes(offset.unwrap_or(0)))
    }

    #[qjs(rename = "toString")]
    pub fn to_string(&self, encoding: Opt<String>) -> String {
        let enc = encoding.0.unwrap_or_else(|| "utf8".to_string());

        match enc.as_str() {
            "utf8" => String::from_utf8(self.data.clone()).unwrap_or_default(),
            "hex" => hex::encode(&self.data),
            &_ => panic!("unsupported operation"),
        }
    }
}

#[module]
pub mod buffer_mod {
    pub use super::Buffer;
}
