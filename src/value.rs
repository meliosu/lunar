#[derive(Clone, Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Slice(Box<[u8]>),
}

impl Value {
    pub fn ptr(&self) -> *const () {
        match self {
            Self::Integer(int) => int as *const i64 as *const (),
            Self::Float(float) => float as *const f64 as *const (),
            Self::Slice(slice) => slice as *const Box<[u8]> as *const (),
            _ => unimplemented!(),
        }
    }
}
