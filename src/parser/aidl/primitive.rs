use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum Primitive {
    Void,
    Bool,
    BoolArray,
    Char,
    CharArray,
    Byte,
    ByteArray,
    Int,
    IntArray,
    Long,
    LongArray,
    Float,
    FloatArray,
    Double,
    DoubleArray,
    String,
    Vector,
    Map,
    Array,
    List,
}

impl Primitive {
    pub fn get_name(&self) -> &'static str {
        match self {
            Primitive::Void => "void",
            Primitive::Bool => "boolean",
            Primitive::BoolArray => "boolean[]",
            Primitive::Char => "char",
            Primitive::CharArray => "char[]",
            Primitive::Byte => "byte",
            Primitive::ByteArray => "byte[]",
            Primitive::Int => "int",
            Primitive::IntArray => "int[]",
            Primitive::Long => "long",
            Primitive::LongArray => "long[]",
            Primitive::Float => "float",
            Primitive::FloatArray => "float[]",
            Primitive::Double => "double",
            Primitive::DoubleArray => "double[]",
            Primitive::String => "String",
            Primitive::Vector => "Vector",
            Primitive::Map => "Map",
            Primitive::List => "List",
            Primitive::Array => "Array",
        }
    }
}
