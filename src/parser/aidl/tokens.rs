pub enum AIDLToken {
    Package {
        name: String,
    },
    Interface {
        methods: Vec<Method>,
    },
    Parcelable {
        items: Vec<ParcelableItem>,
    },
    Enum {
        backing_type: String,
        values: Vec<EnumValue>,
    },
}

pub struct Method {
    is_one_way: bool,
    name: String,
    args: Vec<MethodArg>,
    return_type: Type,
}

pub struct MethodArg {
    name: String,
    arg_type: Type,
}

pub enum Type {
    Native(NativeType),
    Container {
        container_type: ContainerType,
        contained: Box<Type>,
    },
    Custom(CustomType), // e.g.: other element (parcelable, enum, interface)
}

pub enum ContainerType {
    Array,
    Vector,
}

pub enum NativeType {
    Boolean,
    Byte,
    Char,
    Int,
    String,
    ByteArray,
    IntArray,
}

pub struct CustomType {
    package: String,
    name: String,
}

pub struct ParcelableItem {
    name: String,
    item_type: Type,
}

pub struct EnumValue {
    name: String,
    id: i32,
}
