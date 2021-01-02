#[derive(Debug)]
pub struct File {
    pub package: String,
    pub imports: Vec<String>,
    pub items: Vec<Item>,
}

pub enum InterfaceElement {
    Const(Const),
    Method(Method),
}

#[derive(Debug, PartialEq)]
pub enum Item {
    Interface {
        name: String,
        docu: String,
        consts: Vec<Const>,
        methods: Vec<Method>,
        annotations: Vec<Annotation>,
    },
    Parcelable {
        name: String,
        docu: String,
        members: Vec<Member>,
        annotations: Vec<Annotation>,
    },
    Enum {
        name: String,
        docu: String,
        elements: Vec<EnumElement>,
        annotations: Vec<Annotation>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Const {
    pub name: String,
    pub const_type: Type,
    pub value: String,
    pub docu: String,
    pub annotations: Vec<Annotation>,
}

impl Const {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        name: S1,
        const_type: Type,
        value: S2,
        docu: String,
        annotations: Vec<Annotation>,
    ) -> Self {
        Const {
            name: name.into(),
            const_type,
            value: value.into(),
            docu,
            annotations,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Method {
    pub is_one_way: bool,
    pub name: String,
    pub return_type: Type,
    pub args: Vec<Arg>,
    pub docu: String,
    pub annotations: Vec<Annotation>,
}

impl Method {
    pub fn new<S: Into<String>>(
        is_one_way: bool,
        name: S,
        return_type: Type,
        args: Vec<Arg>,
        docu: String,
        annotations: Vec<Annotation>,
    ) -> Self {
        Method {
            is_one_way,
            name: name.into(),
            return_type,
            args,
            docu,
            annotations,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Arg {
    pub direction: Direction,
    pub name: String,
    pub arg_type: Type,
    pub annotations: Vec<Annotation>,
}

impl Arg {
    pub fn new<S: Into<String>>(
        direction: Direction,
        name: S,
        arg_type: Type,
        annotations: Vec<Annotation>,
    ) -> Self {
        Arg {
            direction,
            name: name.into(),
            arg_type,
            annotations,
        }
    }

    #[cfg(test)]
    pub fn with_name<S: Into<String>>(name: S, arg_type: Type) -> Self {
        Self::with_direction(Direction::Unspecified, name, arg_type)
    }

    pub fn with_direction<S: Into<String>>(direction: Direction, name: S, arg_type: Type) -> Self {
        Arg {
            direction,
            name: name.into(),
            arg_type,
            annotations: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn unnamed(arg_type: Type) -> Self {
        Self::with_direction(Direction::Unspecified, "", arg_type)
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    In,
    Out,
    InOut,
    Unspecified,
}

#[derive(Debug, PartialEq)]
pub struct Annotation(pub String);

#[derive(Debug, PartialEq)]
pub struct Type {
    pub name: String,
    pub generic_types: Vec<Type>,
}

impl Type {
    pub fn new<S: Into<String>>(name: S, generic_types: Vec<Type>) -> Self {
        Type {
            name: name.into(),
            generic_types,
        }
    }

    pub fn with_name<S: Into<String>>(name: S) -> Self {
        Type {
            name: name.into(),
            generic_types: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Member {
    pub name: String,
    pub member_type: Type,
    pub docu: String,
    pub annotations: Vec<Annotation>,
}

impl Member {
    pub fn new<S: Into<String>>(
        name: S,
        member_type: Type,
        docu: String,
        annotations: Vec<Annotation>,
    ) -> Self {
        Member {
            name: name.into(),
            member_type,
            docu,
            annotations,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct EnumElement {
    pub name: String,
    pub value: String,
    pub docu: String,
}
