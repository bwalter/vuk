use once_cell::unsync::OnceCell;
use std::{
    collections::{hash_map, HashMap, HashSet},
    fmt::Display,
    rc::Rc,
};

pub type Key = String;

#[derive(Debug, Default)]
pub struct Model {
    pub items: HashMap<Key, Rc<Item>>,
    pub standard_types: HashMap<Key, Rc<StandardType>>,
}

impl Model {
    pub fn resolve_types(&mut self) {
        // Go through all interfaces/structs/enums and add type
        let resolved_items = self
            .items
            .iter()
            .map(|(_, i)| match **i {
                Item::Interface(ref i) => Item::Interface(self.resolved_interface(&i)),
                Item::Struct(ref s) => Item::Struct(self.resolved_struct(&s)),
                Item::Enum(ref e) => Item::Enum(self.resolved_enum(&e)),
            })
            .map(|item| (item.get_key().clone(), Rc::new(item)))
            .collect();

        // Set the model items again
        self.items = resolved_items;
    }

    fn resolved_interface(&self, interface: &Interface) -> Interface {
        let imports = interface.imports.clone(); // TODO?

        let mut index_offset = 0;

        // Go through all interface const
        let resolved_consts: Vec<Const> = interface
            .consts
            .iter()
            .enumerate()
            .map(|(index, c)| {
                let resolved_const_type = if let Type::Unresolved(ref ut) = *c.const_type {
                    if let Some(t) = self.get_resolved_type(ut, &imports) {
                        t
                    } else {
                        c.const_type.clone()
                    }
                } else {
                    c.const_type.clone()
                };
                Const::new(
                    &c.name,
                    resolved_const_type,
                    c.value.clone(),
                    index + index_offset,
                )
            })
            .collect();

        index_offset += resolved_consts.len();

        // Go through all interface methods
        let resolved_methods = interface
            .methods
            .iter()
            .enumerate()
            .map(|(index, m)| {
                // Try to resolve return type
                let resolved_return_arg = self.resolved_arg(&m.return_arg, &imports);

                // Go through all method args
                let resolved_args = m
                    .args
                    .iter()
                    .map(|a| self.resolved_arg(a, &imports))
                    .collect::<Vec<Arg>>();

                Method::new(
                    m.name.clone(),
                    resolved_return_arg,
                    resolved_args,
                    index + index_offset,
                )
            })
            .collect::<Vec<Method>>();

        Interface::new(
            interface.pkg.clone(),
            interface.imports.clone(),
            interface.name.clone(),
            interface.docu.clone(),
            resolved_consts,
            resolved_methods,
        )
    }

    fn resolved_struct(&self, structure: &Struct) -> Struct {
        let mut imports = structure.imports.clone(); // TODO?
        imports.push(structure.pkg.path.clone());

        // Go through all struct members
        let resolved_members = structure
            .members
            .iter()
            .enumerate()
            .map(|(index, m)| {
                let resolved_arg = self.resolved_arg(&m.arg, &imports);
                Member::new(resolved_arg, index)
            })
            .collect();

        Struct::new(
            structure.pkg.clone(),
            structure.imports.clone(),
            &structure.name,
            &structure.docu,
            resolved_members,
        )
    }

    fn resolved_enum(&self, enumeration: &Enum) -> Enum {
        Enum::new(
            enumeration.pkg.clone(),
            enumeration.name.clone(),
            enumeration.docu.clone(),
            enumeration.elements.clone(),
        )
    }

    fn resolved_arg(&self, arg: &Arg, imports: &[String]) -> Arg {
        let resolved_generic_args = arg
            .generic_args
            .iter()
            .map(|ga| self.resolved_arg(ga, imports))
            .collect();

        let resolved_type = if let Some(resolved_type) = self.get_resolved_arg_type(&arg, &imports)
        {
            resolved_type
        } else {
            println!("WARNING: unresolved type: {}", arg.arg_type.get_key());
            arg.arg_type.clone() // still unresolved
        };

        Arg::new(arg.name.clone(), resolved_type, resolved_generic_args)
    }

    fn get_resolved_arg_type(&self, arg: &Arg, imports: &[String]) -> Option<Rc<Type>> {
        if let Type::Unresolved(ref unresolved_type) = *arg.arg_type {
            if let Some(resolved_type) = self.get_resolved_type(unresolved_type, &imports) {
                return Some(resolved_type);
            }
        }

        return None;
    }

    fn get_resolved_type(
        &self,
        unresolved_type: &UnresolvedType,
        imports: &[String],
    ) -> Option<Rc<Type>> {
        // Try to resolve primitive (key = name)
        if let Some(standard_type) = self.standard_types.get(&unresolved_type.name) {
            return Some(Rc::new(Type::Standard(standard_type.clone())));
        }

        // Try with items (interfaces/structs/enums)
        // TODO: support for wildchar imports (e.g: a.b.c.*)
        for (_, item) in &self.items {
            if item.get_pkg().path == unresolved_type.owner.path
                && item.get_name() == unresolved_type.name
            {
                return Some(Rc::new(Type::Item(item.clone())));
            }

            for import in imports {
                let item_path = format!("{}.{}", item.get_pkg().path, item.get_name());
                if unresolved_type.name == item_path
                    || format!("{}.{}", import, unresolved_type.name) == item_path
                {
                    return Some(Rc::new(Type::Item(item.clone())));
                }
            }
        }

        // Not found
        return None;
    }

    pub fn find_dependencies(&self, from: &Item) -> Vec<(HashSet<usize>, Rc<Item>)> {
        let items = match from {
            Item::Interface(i) => self.find_dependencies_of_interface(i),
            Item::Struct(s) => self.find_dependencies_of_struct(s),
            Item::Enum(e) => self.find_dependencies_of_enum(e),
        };

        // Map items with reference members
        let mut map: HashMap<String, HashSet<usize>> =
            items
                .iter()
                .fold(HashMap::new(), |mut acc, (item_index, item)| {
                    acc.entry(item.get_key().clone())
                        .or_default()
                        .insert(*item_index);
                    acc
                });

        // Re-build items without duplicate and with reference members
        items
            .into_iter()
            .filter_map(|(_, item)| match map.entry(item.get_key().clone()) {
                hash_map::Entry::Occupied(oe) => {
                    let (_, indices) = oe.remove_entry();
                    Some((indices, item))
                }
                hash_map::Entry::Vacant(_) => None,
            })
            .collect()
    }

    pub fn find_dependencies_of_interface(&self, from: &Interface) -> Vec<(usize, Rc<Item>)> {
        from.methods
            .iter()
            .flat_map(|m| {
                let mut items: Vec<(usize, Rc<Item>)> = m
                    .args
                    .iter()
                    .flat_map(|a| self.find_dependencies_of_arg(a))
                    .map(|item| (m.index, item))
                    .collect();

                if let Type::Item(ref i) = *m.return_arg.arg_type {
                    items.push((m.index, i.clone()));
                }

                items
            })
            .collect()
    }

    pub fn find_dependencies_of_struct(&self, from: &Struct) -> Vec<(usize, Rc<Item>)> {
        from.members
            .iter()
            .flat_map(|member| {
                self.find_dependencies_of_arg(&member.arg)
                    .into_iter()
                    .map(|item| (member.index, item))
                    .collect::<Vec<(usize, Rc<Item>)>>()
            })
            .collect()
    }

    fn find_dependencies_of_arg(&self, from: &Arg) -> Vec<Rc<Item>> {
        let mut items: Vec<Rc<Item>> = from
            .generic_args
            .iter()
            .flat_map(|ga| self.find_dependencies_of_arg(&ga))
            .collect();

        if let Type::Item(ref i) = *from.arg_type {
            items.push(i.clone());
        }

        items
    }

    pub fn find_dependencies_of_enum(&self, _from: &Enum) -> Vec<(usize, Rc<Item>)> {
        Vec::new()
    }

    pub fn find_references(&self, to: &Item) -> Vec<(HashSet<usize>, Rc<Item>)> {
        self.items
            .iter()
            .filter_map(|(_, candidate_item)| {
                let matching_dependencies = self
                    .find_dependencies(candidate_item)
                    .into_iter()
                    .filter(|(_, i)| i.get_key() == to.get_key());
                let indices =
                    matching_dependencies.fold(HashSet::new(), |mut acc, (indices, _)| {
                        for i in indices {
                            acc.insert(i);
                        }
                        acc
                    });
                if indices.is_empty() {
                    None
                } else {
                    Some((indices, candidate_item.clone()))
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub enum Error {
    NotFound(Key),
    InvalidType {
        key: Key,
        expected: &'static str,
        found: &'static str,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Model error: {:#?}", *self)
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub struct Package {
    pub path: String,
}

impl Package {
    pub fn new<S: Into<String>>(path: S) -> Self {
        Package { path: path.into() }
    }

    pub fn get_key<'a>(&'a self) -> &'a String {
        &self.path
    }
}

#[derive(Debug)]
pub enum Item {
    Interface(Interface),
    Struct(Struct),
    Enum(Enum),
}

impl Item {
    pub fn get_name(&self) -> &str {
        match self {
            Item::Interface(i) => &i.name,
            Item::Struct(s) => &s.name,
            Item::Enum(e) => &e.name,
        }
    }

    pub fn get_pkg(&self) -> &Package {
        match self {
            Item::Interface(i) => &i.pkg,
            Item::Struct(s) => &s.pkg,
            Item::Enum(e) => &e.pkg,
        }
    }

    pub fn get_key(&self) -> &Key {
        match self {
            Item::Interface(i) => i.get_key(),
            Item::Struct(s) => s.get_key(),
            Item::Enum(e) => &e.get_key(),
        }
    }

    pub fn get_docu(&self) -> &str {
        match self {
            Item::Interface(i) => &i.docu,
            Item::Struct(s) => &s.docu,
            Item::Enum(e) => &e.docu,
        }
    }
}

#[derive(Debug)]
pub struct Interface {
    pub pkg: Rc<Package>,
    pub imports: Vec<String>,
    pub name: String,
    pub docu: String,
    pub consts: Vec<Const>,
    pub methods: Vec<Method>,
    lazy_key: OnceCell<String>,
}

impl Interface {
    pub fn new<S1: Into<String>, S2: Into<String>, V: Into<Vec<String>>>(
        pkg: Rc<Package>,
        imports: V,
        name: S1,
        docu: S2,
        consts: Vec<Const>,
        methods: Vec<Method>,
    ) -> Self {
        Interface {
            pkg,
            imports: imports.into(),
            name: name.into(),
            docu: docu.into(),
            consts,
            methods,
            lazy_key: OnceCell::new(),
        }
    }

    pub fn get_key(&self) -> &Key {
        &self
            .lazy_key
            .get_or_init(|| format!("{}.{}", self.pkg.path, self.name))
    }
}

#[derive(Debug)]
pub struct Struct {
    pub pkg: Rc<Package>,
    pub imports: Vec<String>,
    pub name: String,
    pub docu: String,
    pub members: Vec<Member>,
    lazy_key: OnceCell<String>,
}

impl Struct {
    pub fn new<S1: Into<String>, S2: Into<String>, V: Into<Vec<String>>>(
        pkg: Rc<Package>,
        imports: V,
        name: S1,
        docu: S2,
        members: Vec<Member>,
    ) -> Self {
        Struct {
            pkg,
            imports: imports.into(),
            name: name.into(),
            docu: docu.into(),
            members,
            lazy_key: OnceCell::new(),
        }
    }

    pub fn get_key(&self) -> &Key {
        &self
            .lazy_key
            .get_or_init(|| format!("{}.{}", self.pkg.path, self.name))
    }
}

#[derive(Debug)]
pub struct Enum {
    pub pkg: Rc<Package>,
    pub name: String,
    pub docu: String,
    pub elements: Vec<EnumElement>,
    lazy_key: OnceCell<String>,
}

impl Enum {
    pub fn new<S: Into<String>>(
        pkg: Rc<Package>,
        name: S,
        docu: String,
        elements: Vec<EnumElement>,
    ) -> Self {
        Enum {
            pkg,
            name: name.into(),
            docu,
            elements,
            lazy_key: OnceCell::new(),
        }
    }

    pub fn get_key(&self) -> &Key {
        &self
            .lazy_key
            .get_or_init(|| format!("{}.{}", self.pkg.path, self.name))
    }
}

#[derive(Debug)]
pub struct Const {
    pub name: String,
    pub const_type: Rc<Type>,
    pub value: String,
    pub index: usize,
}

impl Const {
    pub fn new<S: Into<String>>(
        name: S,
        const_type: Rc<Type>,
        value: String,
        index: usize,
    ) -> Self {
        Const {
            name: name.into(),
            const_type,
            value,
            index,
        }
    }
}

#[derive(Debug)]
pub struct Member {
    pub arg: Arg,
    pub index: usize,
}

impl Member {
    pub fn new(arg: Arg, index: usize) -> Self {
        Member { arg, index }
    }
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub return_arg: Arg,
    pub args: Vec<Arg>,
    pub index: usize,
}

impl Method {
    pub fn new<S: Into<String>>(name: S, return_arg: Arg, args: Vec<Arg>, index: usize) -> Self {
        Method {
            name: name.into(),
            return_arg,
            args,
            index,
        }
    }

    //pub fn get_sig(&self) -> &Key {
    //    self.lazy_sig.get_or_init(|| {
    //        let args_string = self
    //            .args
    //            .iter()
    //            .map(|a| a.arg_type.get_key().clone())
    //            .collect::<Vec<String>>()
    //            .join(",");
    //        format!(
    //            "{}({})->{}",
    //            self.name,
    //            args_string,
    //            self.return_arg.arg_type.get_key()
    //        )
    //    })
    //}
}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
    pub arg_type: Rc<Type>,
    pub generic_args: Vec<Arg>,
}

impl Arg {
    pub fn new<S: Into<String>>(name: S, arg_type: Rc<Type>, generic_args: Vec<Arg>) -> Self {
        Arg {
            name: name.into(),
            arg_type,
            generic_args,
        }
    }
}

#[derive(Debug)]
pub enum Type {
    Standard(Rc<StandardType>),
    Unresolved(UnresolvedType),
    Item(Rc<Item>),
}

impl Type {
    pub fn get_key(&self) -> &String {
        match *self {
            Type::Standard(ref g) => g.get_key(),
            Type::Unresolved(ref u) => u.get_key(),
            Type::Item(ref i) => i.get_key(),
        }
    }

    pub fn get_name(&self) -> &str {
        match *self {
            Type::Standard(ref g) => &g.name,
            Type::Unresolved(ref u) => &u.name,
            Type::Item(ref i) => i.get_name(),
        }
    }
}

impl Type {
    pub fn is_void(&self) -> bool {
        match self {
            Type::Standard(st) => st.is_void(),
            Type::Unresolved(_) => false,
            Type::Item(_) => false,
        }
    }
}

#[derive(Debug)]
pub struct StandardType {
    pub name: String,
    pub package: Rc<Package>,
    lazy_key: OnceCell<String>,
}

impl StandardType {
    // TODO: cover more languages (e.g.: Unit, (), Nothing, ...)
    pub fn is_void(&self) -> bool {
        self.name == "void"
    }
}

impl StandardType {
    pub fn new<S: Into<String>>(name: S, package: Rc<Package>) -> Self {
        StandardType {
            name: name.into(),
            package,
            lazy_key: OnceCell::new(),
        }
    }

    pub fn get_key(&self) -> &Key {
        self.lazy_key.get_or_init(|| {
            if self.package.path.is_empty() {
                format!("{}", self.name)
            } else {
                format!("{}.{}", self.package.path, self.name)
            }
        })
    }
}
#[derive(Debug)]
pub struct UnresolvedType {
    pub owner: Rc<Package>,
    pub name: String,
    lazy_key: OnceCell<String>,
}

impl UnresolvedType {
    pub fn new<S: Into<String>>(owner: Rc<Package>, name: S) -> Self {
        UnresolvedType {
            owner,
            name: name.into(),
            lazy_key: OnceCell::new(),
        }
    }

    pub fn get_key(&self) -> &Key {
        self.lazy_key
            .get_or_init(|| format!("{}:{}", self.owner.path, self.name,))
    }
}
#[derive(Clone, Debug)]
pub struct EnumElement {
    pub name: String,
    pub value: String,
    pub index: usize,
}
