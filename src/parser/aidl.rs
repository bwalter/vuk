pub mod ast;
mod parse;
mod primitive;

use crate::model::{self, Model};
pub use parse::parse;
use primitive::Primitive;
use std::{collections::HashMap, rc::Rc};
use strum::IntoEnumIterator;

pub fn create_model(files: Vec<ast::File>) -> Model {
    let mut items = HashMap::new();

    // Root package
    let root_pkg = Rc::new(model::Package::new(""));

    // Go through files
    files.into_iter().for_each(|file| {
        let pkg = Rc::new(create_model_package(&file.package));
        let imports = file.imports;

        // Add items
        file.items.into_iter().for_each(|item| match item {
            ast::Item::Interface {
                name,
                docu,
                consts,
                methods,
                annotations,
            } => {
                let interface =
                    create_model_interface(&pkg, &imports, &name, &consts, &methods, docu);

                items.insert(
                    interface.get_key().clone(),
                    Rc::new(model::Item::Interface(interface)),
                );
            }
            ast::Item::Parcelable {
                name,
                docu,
                members,
                annotations,
            } => {
                let strukt = create_model_struct(&pkg, &imports, &name, &members, docu);
                items.insert(
                    strukt.get_key().clone(),
                    Rc::new(model::Item::Struct(strukt)),
                );
            }
            ast::Item::Enum {
                name,
                docu,
                elements,
                annotations,
            } => {
                let enumeration = create_model_enum(&pkg, &imports, &name, &elements, docu);
                items.insert(
                    enumeration.get_key().clone(),
                    Rc::new(model::Item::Enum(enumeration)),
                );
            }
        });
    });

    // Create standard types
    let standard_types = primitive::Primitive::iter()
        .map(|p| Primitive::get_name(&p))
        .map(|n| model::StandardType::new(n, root_pkg.clone()))
        .map(|st| (st.get_key().clone(), Rc::new(st)))
        .collect();

    let mut model = Model {
        items,
        standard_types,
    };

    model.resolve_types();
    model
}

fn create_model_package(pkg: &str) -> model::Package {
    model::Package::new(pkg)
}

fn create_model_interface(
    pkg: &Rc<model::Package>,
    imports: &[String],
    name: &str,
    consts: &Vec<ast::Const>,
    methods: &Vec<ast::Method>,
    docu: String,
) -> model::Interface {
    model::Interface::new(
        pkg.clone(),
        imports,
        name,
        docu,
        consts
            .into_iter()
            .map(|c| create_model_const(pkg, c))
            .collect(),
        methods
            .into_iter()
            .map(|m| create_model_method(pkg, m))
            .collect(),
    )
}

fn create_model_struct(
    pkg: &Rc<model::Package>,
    imports: &[String],
    name: &str,
    members: &Vec<ast::Member>,
    docu: String,
) -> model::Struct {
    model::Struct::new(
        pkg.clone(),
        imports,
        name,
        docu,
        members
            .into_iter()
            .map(|m| create_model_member(pkg, m))
            .collect(),
    )
}

fn create_model_enum(
    pkg: &Rc<model::Package>,
    imports: &[String],
    name: &str,
    elements: &Vec<ast::EnumElement>,
    docu: String,
) -> model::Enum {
    model::Enum::new(
        pkg.clone(),
        name,
        docu,
        elements
            .into_iter()
            .map(|e| create_model_enum_element(pkg, e))
            .collect(),
    )
}

fn create_model_const(package: &Rc<model::Package>, the_const: &ast::Const) -> model::Const {
    let const_type = create_model_unresolved_type(package, &the_const.const_type);

    model::Const::new(
        &the_const.name,
        Rc::new(model::Type::Unresolved(const_type)),
        the_const.value.clone(),
        0, // TODO
        the_const.docu.clone(),
    )
}

fn create_model_member(package: &Rc<model::Package>, member: &ast::Member) -> model::Member {
    model::Member::new(
        create_model_arg(package, &member.member_type, member.name.clone()),
        0, // TODO
        member.docu.clone(),
    )
}

fn create_model_enum_element(
    package: &Rc<model::Package>,
    element: &ast::EnumElement,
) -> model::EnumElement {
    model::EnumElement {
        name: element.name.clone(),
        value: element.value.clone(),
        index: 0, // TODO
        docu: element.docu.clone(),
    }
}

fn create_model_method(package: &Rc<model::Package>, method: &ast::Method) -> model::Method {
    let return_arg = create_model_arg(package, &method.return_type, String::new());
    let args = method
        .args
        .iter()
        .map(|a| create_model_arg(package, &a.arg_type, a.name.clone()))
        .collect();

    model::Method::new(
        &method.name,
        return_arg,
        args,
        0, /* TODO */
        method.docu.clone(),
    )
}

fn create_model_arg(
    package: &Rc<model::Package>,
    arg_type: &ast::Type,
    name: String,
) -> model::Arg {
    let generic_args = arg_type
        .generic_types
        .iter()
        .map(|t| create_model_arg(package, t, String::new()))
        .collect();

    model::Arg {
        name,
        arg_type: Rc::new(model::Type::Unresolved(create_model_unresolved_type(
            package, arg_type,
        ))),
        generic_args,
    }
}

fn create_model_unresolved_type(
    package: &Rc<model::Package>,
    arg_type: &ast::Type,
) -> model::UnresolvedType {
    // TODO: deal with array (e.g.: Krumpli[])
    model::UnresolvedType::new(package.clone(), arg_type.name.clone())
}
