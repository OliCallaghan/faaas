use rquickjs::{class, class::Trace, methods, Value};

#[derive(Trace)]
#[class]
pub struct Console {}

#[methods]
impl Console {
    #[qjs(skip)]
    pub fn new() -> Self {
        Console {}
    }

    #[qjs(rename = "log")]
    pub fn log(&self, val: Value<'_>) {
        println!("{:?}", val);
    }
}
