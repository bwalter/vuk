use std::cell::RefCell;

use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_until},
    character::complete::{
        alpha1, alphanumeric1, char, digit0, digit1, line_ending, multispace0, multispace1,
        not_line_ending, space0,
    },
    combinator::{all_consuming, cut, map, opt, recognize},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
use nom_locate::LocatedSpan;

use crate::parser::aidl::ast::{
    Annotation, Arg, Const, Direction, EnumElement, File, InterfaceElement, Item, Member, Method,
    Type,
};
use crate::parser::error::ParseContentError;

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse(input: &str) -> Result<File, ParseContentError> {
    let input = Span::new(input);

    match all_consuming(parse_aidl)(input) {
        Ok((_, file)) => Ok(file),
        Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => Err(e.into()),
        Err(nom::Err::Incomplete(_)) => unreachable!(),
    }
}

// AIDL file:
// - 0+ <comment>
// - <package>
// - 0+ <import>
// - 0+ <item>
fn parse_aidl(input: Span) -> IResult<Span, File> {
    let (input, _) = many0(ws(parse_comment))(input)?;
    let (input, package) = ws(parse_package)(input)?;

    fn find_imports(
        input: Span,
        imports: RefCell<Vec<String>>,
    ) -> IResult<Span, RefCell<Vec<String>>> {
        let (try_input, t) = tuple((
            opt(ws(parse_comment)),
            opt(ws(parse_import)),
            opt(ws(parse_forward_declaration)),
        ))(input)?;

        if let (_, None, None) = t {
            // Done => return the old input }
            return Ok((input, imports));
        }

        if let (_, Some(import), _) = t {
            imports.borrow_mut().push(import.to_string());
        }

        // Continue with consumed input
        find_imports(try_input, imports)
    };

    let (input, imports) = find_imports(input, RefCell::new(Vec::new()))?;
    let imports = imports.into_inner();

    let (input, items) = many0(ws(parse_item))(input)?;

    Ok((
        input,
        File {
            package,
            imports,
            items,
        },
    ))
}

// Example:
// - package <package_name>;
fn parse_package(input: Span) -> IResult<Span, String> {
    let (input, (_, pkg_name, _)) =
        tuple((ws_plus(tag("package")), ws(parse_package_name), char(';')))(input)?;

    Ok((input, pkg_name.to_string()))
}

// Examples:
// - x
// - x.y.z
fn parse_package_name(input: Span) -> IResult<Span, Span> {
    recognize(separated_list1(char('.'), identifier))(input)
}

// Example:
// - import <package_name>;
fn parse_import(input: Span) -> IResult<Span, Span> {
    let (input, (_, pkg_name, _)) =
        tuple((ws_plus(tag("import")), ws(parse_package_name), char(';')))(input)?;

    Ok((input, pkg_name))
}

// Forward declaration (ignored)
// Examples:
// - interface InterfaceName;
// - parcelable ParcelableName;
// - enum ParcelableName;
fn parse_forward_declaration(input: Span) -> IResult<Span, ()> {
    let (input, _) = recognize(tuple((
        ws_plus(alt((tag("interface"), tag("parcelable"), tag("enum")))),
        ws(identifier),
        char(';'),
    )))(input)?;

    Ok((input, ()))
}

// Examples:
// - <interface>
// - <parcelable>
// - <enum>
// TODO:  enum
fn parse_item(input: Span) -> IResult<Span, Item> {
    alt((parse_interface, parse_parcelable, parse_enum))(input)
}

// Examples:
// - interface interfaceName { <const>; <method>; <method>; <const>; }
fn parse_interface(input: Span) -> IResult<Span, Item> {
    let (input, opt_docu) = opt(extract_javadoc)(input)?;

    let (input, annotations) = many0(ws_plus(parse_annotation))(input)?;
    let (input, _) = ws_plus(tag("interface"))(input)?;
    let (input, interface_name) = cut(ws(identifier))(input)?;

    let (input, interface_element) = delimited(
        ws(char('{')),
        many0(ws(parse_interface_element)),
        ws(char('}')),
    )(input)?;

    let mut consts = Vec::new();
    let mut methods = Vec::new();

    interface_element.into_iter().for_each(|e| match e {
        InterfaceElement::Const(c) => {
            consts.push(c);
        }
        InterfaceElement::Method(m) => {
            methods.push(m);
        }
    });

    Ok((
        input,
        Item::Interface {
            name: interface_name.to_string(),
            docu: opt_docu.unwrap_or_default(),
            consts,
            methods,
            annotations,
        },
    ))
}

// Examples:
// - parcelable parcelableName { <member>; <member>; }
fn parse_parcelable(input: Span) -> IResult<Span, Item> {
    let (input, opt_docu) = opt(extract_javadoc)(input)?;

    let (input, annotations) = many0(ws_plus(parse_annotation))(input)?;
    let (input, _) = ws_plus(tag("parcelable"))(input)?;
    let (input, parcelable_name) = cut(ws(identifier))(input)?;

    let (input, members) = delimited(
        ws(char('{')),
        many0(ws(parse_member)),
        pair(opt(parse_comment), ws(char('}'))),
    )(input)?;

    Ok((
        input,
        Item::Parcelable {
            name: parcelable_name.to_string(),
            docu: opt_docu.unwrap_or_default(),
            members,
            annotations,
        },
    ))
}

// Examples:
// - enum enumName { <enum_element>, <enum_element> }
fn parse_enum(input: Span) -> IResult<Span, Item> {
    let (input, opt_docu) = opt(extract_javadoc)(input)?;

    let (input, annotations) = many0(ws_plus(parse_annotation))(input)?;
    let (input, _) = ws_plus(tag("enum"))(input)?;
    let (input, enum_name) = cut(ws(identifier))(input)?;

    let (input, elements) = delimited(
        ws(char('{')),
        separated_list1(char(','), ws(parse_enum_element)),
        ws(pair(opt(ws(char(','))), char('}'))),
    )(input)?;

    Ok((
        input,
        Item::Enum {
            name: enum_name.to_string(),
            docu: opt_docu.unwrap_or_default(),
            elements,
            annotations,
        },
    ))
}

// Examples:
// - ELEMENT
// - ELEMENT = 3
// - ELEMENT = "elementValue"
fn parse_enum_element(input: Span) -> IResult<Span, EnumElement> {
    let (input, opt_docu) = opt(extract_javadoc)(input)?;

    let (input, (name, opt_value)) = tuple((
        identifier,
        opt(preceded(ws(char('=')), parse_enum_element_value)),
    ))(input)?;

    Ok((
        input,
        EnumElement {
            name: name.to_string(),
            value: opt_value
                .and_then(|v| Some(v.to_string()))
                .unwrap_or_default(),
            docu: opt_docu.unwrap_or_default(),
        },
    ))
}

fn parse_enum_element_value(input: Span) -> IResult<Span, Span> {
    alt((
        recognize(pair(opt(char('-')), digit1)),
        delimited(char('"'), take_until("\""), char('"')),
    ))(input)
}

// Examples:
// - <const>
// - <method>
fn parse_interface_element(input: Span) -> IResult<Span, InterfaceElement> {
    // Const
    match parse_const(input) {
        Err(nom::Err::Failure(e)) => return Err(nom::Err::Failure(e)),
        Err(nom::Err::Error(_)) => (),
        Err(nom::Err::Incomplete(_)) => unreachable!(),
        Ok((input, c)) => {
            return Ok((input, InterfaceElement::Const(c)));
        }
    }

    // Interface
    match parse_method(input) {
        Err(nom::Err::Failure(e)) => return Err(nom::Err::Failure(e)),
        Err(nom::Err::Error(_)) => (),
        Err(nom::Err::Incomplete(_)) => unreachable!(),
        Ok((input, m)) => {
            return Ok((input, InterfaceElement::Method(m)));
        }
    }

    // (ignored) comments
    let (input, _) = many0(ws(parse_comment))(input)?;

    // Invalid input (up to the next ';' or '}') => failure
    let (_, input) = is_not("};")(input)?;
    Err(nom::Err::Failure(nom::error::make_error(
        input,
        nom::error::ErrorKind::Many1,
    )))
}

// Examples:
// - const Type constName = value
fn parse_const(input: Span) -> IResult<Span, Const> {
    let (input, opt_docu) = opt(extract_javadoc)(input)?;
    let (input, (annotations, _, const_type, const_name, _, const_value, _)) = tuple((
        many0(ws_plus(parse_annotation)),
        ws_plus(tag("const")),
        ws_plus(parse_type),
        ws(identifier),
        ws(char('=')),
        ws(parse_const_value),
        char(';'),
    ))(input)?;

    Ok((
        input,
        Const::new(
            *const_name,
            const_type,
            *const_value,
            opt_docu.unwrap_or_default(),
            annotations,
        ),
    ))
}

// Examples:
// - 123
// - "a string value"
fn parse_const_value(input: Span) -> IResult<Span, Span> {
    alt((
        recognize(tuple((
            digit1,
            opt(preceded(char('.'), digit0)),
            opt(char('f')),
        ))),
        delimited(char('"'), take_until("\""), char('"')),
        tag("{}"),
    ))(input)
}

// Examples:
// - Type memberName;
fn parse_member(input: Span) -> IResult<Span, Member> {
    let (input, opt_docu) = opt(extract_javadoc)(input)?;
    let (input, (annotations, member_type, member_name, _opt_value, _)) = tuple((
        many0(ws_plus(parse_annotation)),
        ws_plus(parse_type),
        ws(identifier),
        opt(ws(preceded(char('='), ws(parse_const_value)))),
        char(';'),
    ))(input)?;

    // TODO: value!

    Ok((
        input,
        Member::new(
            *member_name,
            member_type,
            opt_docu.unwrap_or_default(),
            annotations,
        ),
    ))
}

// Examples:
// - oneway ReturnType methodName(<args>)
// - ReturnType methodName(<args>) = 123
fn parse_method(input: Span) -> IResult<Span, Method> {
    let (input, opt_docu) = opt(extract_javadoc)(input)?;

    let (input, (annotations, oneway_opt, method_type, method_name, args, _, _)) = tuple((
        many0(ws_plus(parse_annotation)),
        opt(ws_plus(tag("oneway"))),
        ws_plus(parse_type),
        ws(identifier),
        delimited(ws(char('(')), parse_args, ws(char(')'))),
        opt(tuple((ws(char('=')), ws(digit1)))),
        char(';'),
    ))(input)?;

    Ok((
        input,
        Method::new(
            oneway_opt.is_some(),
            *method_name,
            method_type,
            args,
            opt_docu.unwrap_or_default(),
            annotations,
        ),
    ))
}

// Examples:
// -
// - <arg>
// - <arg>, <arg>, <arg>
fn parse_args(input: Span) -> IResult<Span, Vec<Arg>> {
    separated_list0(ws(char(',')), ws(parse_arg))(input)
}

// Examples:
// - <type>
// - <direction> <type>
// - <direction> <type> argName
fn parse_arg(input: Span) -> IResult<Span, Arg> {
    let (input, (annotations, arg_direction, arg_type, opt_arg_name)) = tuple((
        many0(ws_plus(parse_annotation)),
        opt(ws_plus(parse_direction)),
        parse_type,
        opt(plus_ws(identifier)),
    ))(input)?;

    Ok((
        input,
        Arg::new(
            arg_direction.unwrap_or(Direction::Unspecified),
            opt_arg_name.as_deref().unwrap_or(&"").to_string(),
            arg_type,
            annotations,
        ),
    ))
}

// in, out or inout
fn parse_direction(input: Span) -> IResult<Span, Direction> {
    let (input, direction_str) = alt((tag("inout"), tag("in"), tag("out")))(input)?;

    Ok((
        input,
        match *direction_str {
            "in" => Direction::In,
            "out" => Direction::Out,
            "inout" => Direction::InOut,
            _ => unreachable!(),
        },
    ))
}

// Examples:
// - typeName
// - <generic>
fn parse_type(input: Span) -> IResult<Span, Type> {
    alt((
        parse_generic,
        parse_custom_array,
        map(parse_type_name, |s: String| Type::new(s, Vec::new())),
    ))(input)
}

fn parse_custom_array(input: Span) -> IResult<Span, Type> {
    let (input, name) = terminated(parse_package_name, preceded(space0, tag("[]")))(input)?;
    let new_type = Type::new("Array", vec![Type::with_name(*name)]);

    Ok((input, new_type))
}
//let unresolved_typeme = if unresolved_type.name.ends_with("[]") {
//    let name = unresolved_type.name[0..unresolved_type.name.len() - 2].to_string();

fn parse_type_name(input: Span) -> IResult<Span, String> {
    let (input, (name, opt_array)) =
        pair(parse_package_name, opt(preceded(space0, tag("[]"))))(input)?;
    let s = match opt_array {
        Some(_) => format!("{}[]", name),
        None => name.to_string(),
    };

    Ok((input, s))
}

// - typeName<subType1, subType2<subSubType>>
fn parse_generic(input: Span) -> IResult<Span, Type> {
    let (input, (name, generic_types)) = tuple((
        ws(parse_type_name),
        delimited(char('<'), ws(parse_generic_args), char('>')),
    ))(input)?;

    Ok((
        input,
        Type {
            name: name.to_string(),
            generic_types,
        },
    ))
}

fn parse_generic_args(input: Span) -> IResult<Span, Vec<Type>> {
    separated_list1(char(','), ws(parse_type))(input)
}

// Examples:
// - @AnnotationName
// - @AnnotationName(Hello="World")
fn parse_annotation(input: Span) -> IResult<Span, Annotation> {
    let (input, annotation) = preceded(
        char('@'),
        recognize(pair(
            identifier,
            opt(delimited(char('('), is_not(")"), char(')'))),
        )),
    )(input)?;

    Ok((input, Annotation(annotation.to_string())))
}

// Examples:
// /* ... */
// // ...
fn parse_comment(input: Span) -> IResult<Span, Span> {
    alt((
        delimited(tag("//"), preceded(space0, not_line_ending), line_ending),
        delimited(tag("/*"), ws(take_until("*/")), tag("*/")),
    ))(input)
}

fn extract_javadoc(input: Span) -> IResult<Span, String> {
    let (input, comments) = many0(ws(recognize(parse_comment)))(input)?;

    let mut docu = String::new();
    if let Some(lc) = comments.last() {
        if let Ok((_, Some(jd))) = opt(parse_javadoc)(*lc) {
            docu = jd;
        }
    }

    Ok((input, docu))
}

// Examples:
// /** ... */
// /** ... \n * ... \n * ... */
fn parse_javadoc(input: Span) -> IResult<Span, String> {
    let (input, comment_str) = delimited(tag("/**"), take_until("*/"), tag("*/"))(input)?;
    let (_, lines) = separated_list0(
        tuple((line_ending, space0, opt(char('*')), space0)),
        not_line_ending,
    )(comment_str)?;

    let doc = lines
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    Ok((input, doc))
}

// Valid identifier
pub fn identifier(input: Span) -> IResult<Span, Span> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0(alt((alphanumeric1, tag("_")))),
    ))(input)
}

// Ignore leading/trailing spaces
fn ws<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}

// Ignore leading/trailing spaces (at least 1 leading space required)
fn plus_ws<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace1, f, multispace0)
}

// Ignore leading/trailing spaces (at least 1 trailing space required)
fn ws_plus<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("package x;rest");
        let (input, pkg) = parse_package(input)?;
        assert_eq!(pkg, "x");
        assert_eq!(*input, "rest");

        let input = Span::new("package x.y.z;rest");
        let (input, pkg) = parse_package(input)?;
        assert_eq!(pkg, "x.y.z");
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_import() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("import x.y.z;rest");
        let (input, pkg) = parse_import(input)?;
        assert_eq!(*pkg, "x.y.z");
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_simple_type() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("TypeName;rest");
        let (input, output_type) = parse_type(input)?;

        assert_eq!(output_type, Type::with_name("TypeName"),);
        assert_eq!(*input, ";rest");

        Ok(())
    }

    #[test]
    fn test_array_type() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("ArrayTypeName [];rest");
        let (input, output_type) = parse_type(input)?;

        assert_eq!(
            output_type,
            Type::new("Array", vec![Type::with_name("ArrayTypeName")])
        );
        assert_eq!(*input, ";rest");

        Ok(())
    }

    #[test]
    fn test_generic_type() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("TestMap<Key,TestVec<V1, V2>>;rest");
        let (input, output_type) = parse_type(input)?;

        assert_eq!(
            output_type,
            Type {
                name: "TestMap".to_string(),
                generic_types: vec![
                    Type::with_name("Key"),
                    Type::new(
                        "TestVec",
                        vec![Type::with_name("V1"), Type::with_name("V2")]
                    )
                ],
            }
        );
        assert_eq!(*input, ";rest");

        Ok(())
    }
    #[test]
    fn test_arg_with_name() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("TypeName Albert;rest");
        let (input, arg) = parse_arg(input)?;

        assert_eq!(arg, Arg::with_name("Albert", Type::with_name("TypeName")));
        assert_eq!(*input, ";rest");

        Ok(())
    }

    #[test]
    fn test_arg_with_direction() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("in TypeName;rest");
        let (input, arg) = parse_arg(input)?;

        assert_eq!(
            arg,
            Arg::with_direction(Direction::In, "", Type::with_name("TypeName"))
        );
        assert_eq!(*input, ";rest");

        Ok(())
    }

    #[test]
    fn test_arg_with_direction_and_name() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("inout TypeName<G> Roger;rest");
        let (input, arg) = parse_arg(input)?;

        assert_eq!(
            arg,
            Arg::with_direction(
                Direction::InOut,
                "Roger",
                Type::new("TypeName", vec![Type::with_name("G")])
            )
        );
        assert_eq!(*input, ";rest");

        Ok(())
    }

    #[test]
    fn test_arg_with_annotations() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"@Annotation1
            @Annotation2(AnnotationParam)
            TypeName Albert;rest"#,
        );
        let (input, arg) = parse_arg(input)?;

        assert_eq!(
            arg,
            Arg::new(
                Direction::Unspecified,
                "Albert",
                Type::with_name("TypeName"),
                vec![
                    Annotation("Annotation1".to_string()),
                    Annotation("Annotation2(AnnotationParam)".to_string())
                ],
            )
        );
        assert_eq!(*input, ";rest");

        Ok(())
    }

    #[test]
    fn test_num_const() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("const TypeName CONST_NAME = 123;rest");
        let (input, the_const) = parse_const(input)?;

        assert_eq!(
            the_const,
            Const::new(
                "CONST_NAME",
                Type::with_name("TypeName"),
                "123",
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_string_const() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("const TypeName CONST_NAME = \"const_value\";rest");
        let (input, the_const) = parse_const(input)?;

        assert_eq!(
            the_const,
            Const::new(
                "CONST_NAME",
                Type::with_name("TypeName"),
                "const_value",
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_const_with_javadoc() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"
/**
 * Const docu
 */
const TypeName CONST_NAME = 123;rest"#,
        );
        let (input, the_const) = parse_const(input)?;

        assert_eq!(
            the_const,
            Const::new(
                "CONST_NAME",
                Type::with_name("TypeName"),
                "123",
                " Const docu ".to_string(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_const_with_annotation() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("@AnnotationName const TypeName CONST_NAME = 123;rest");
        let (input, the_const) = parse_const(input)?;

        assert_eq!(
            the_const,
            Const::new(
                "CONST_NAME",
                Type::with_name("TypeName"),
                "123",
                String::new(),
                vec![Annotation("AnnotationName".to_string())]
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_member() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("TypeName memberName;rest");
        let (input, member) = parse_member(input)?;

        assert_eq!(
            member,
            Member::new(
                "memberName",
                Type::with_name("TypeName"),
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_member_with_value() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("TypeName memberName = \"memberValue\";rest");
        let (input, member) = parse_member(input)?;

        assert_eq!(
            member,
            Member::new(
                "memberName",
                Type::with_name("TypeName"),
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_member_with_javadoc() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"
/**
 * Member docu
 */
TypeName memberName;rest"#,
        );
        let (input, member) = parse_member(input)?;

        assert_eq!(
            member,
            Member::new(
                "memberName",
                Type::with_name("TypeName"),
                " Member docu ".to_string(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_member_with_annotation() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("@AnnotationName TypeName memberName;rest");
        let (input, member) = parse_member(input)?;

        assert_eq!(
            member,
            Member::new(
                "memberName",
                Type::with_name("TypeName"),
                String::new(),
                vec![Annotation("AnnotationName".to_string())]
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_method_without_arg() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("TypeName myMethod();rest");
        let (input, method) = parse_method(input)?;

        assert_eq!(
            method,
            Method::new(
                false,
                "myMethod",
                Type::with_name("TypeName"),
                Vec::new(),
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_method_with_1_arg() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("TypeName myMethod(ArgType arg);rest");
        let (input, method) = parse_method(input)?;

        assert_eq!(
            method,
            Method::new(
                false,
                "myMethod",
                Type::with_name("TypeName"),
                vec![Arg::with_name("arg", Type::with_name("ArgType"))],
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_method_with_3_args() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("TypeName myMethod(ArgType1, ArgType2 arg2, ArgType3);rest");
        let (input, method) = parse_method(input)?;

        assert_eq!(
            method,
            Method::new(
                false,
                "myMethod",
                Type::with_name("TypeName"),
                vec![
                    Arg::unnamed(Type::with_name("ArgType1")),
                    Arg::with_name("arg2", Type::with_name("ArgType2")),
                    Arg::unnamed(Type::with_name("ArgType3"))
                ],
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_oneway_method() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("oneway void myMethod();rest");
        let (input, method) = parse_method(input)?;

        assert_eq!(
            method,
            Method::new(
                true,
                "myMethod",
                Type::with_name("void"),
                Vec::new(),
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_method_with_value() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("void myMethod() = 123;rest");
        let (input, method) = parse_method(input)?;

        assert_eq!(
            method,
            Method::new(
                false,
                "myMethod",
                Type::with_name("void"),
                Vec::new(),
                String::new(),
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_method_with_javadoc() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"
/**
 * Method docu
 */
void myMethod() = 123;rest"#,
        );
        let (input, method) = parse_method(input)?;

        assert_eq!(
            method,
            Method::new(
                false,
                "myMethod",
                Type::with_name("void"),
                Vec::new(),
                " Method docu ".to_string(), // TODO
                Vec::new(),
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_method_withannotation() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("@AnnotationName TypeName myMethod();rest");
        let (input, method) = parse_method(input)?;

        assert_eq!(
            method,
            Method::new(
                false,
                "myMethod",
                Type::with_name("TypeName"),
                Vec::new(),
                String::new(),
                vec![Annotation("AnnotationName".to_string())]
            )
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_eol_comment() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("// first comment\n// second comment\nrest");
        let (input, comments) = many0(map(parse_comment, |c| *c))(input)?;

        assert_eq!(comments, vec!["first comment", "second comment"]);
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_multiline_comment() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("/* this is a multi\nline comment*/rest");
        let (input, comment) = parse_comment(input)?;

        assert_eq!(*comment, "this is a multi\nline comment");
        assert_eq!(*input, "rest");

        Ok(())
    }
    #[test]
    #[ignore]
    fn test_javadoc() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("/** This is a javadoc\n * comment*/rest");
        let (input, comment) = parse_javadoc(input)?;

        assert_eq!(comment, " This is a javadoc comment".to_string());
        assert_eq!(*input, "rest");

        let input =
            Span::new("/**\n * JavaDoc title\n *\n * JavaDoc line1\n * JavaDoc line2\n */rest");
        let (input, comment) = parse_javadoc(input)?;

        assert_eq!(
            comment,
            "JavaDoc title\n\nJavaDoc line1\nJAvaDoc line2".to_string()
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_annotation() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new("@AnnotationName;rest");
        let (input, annotation) = parse_annotation(input)?;

        assert_eq!(annotation, Annotation("AnnotationName".to_string()));

        assert_eq!(*input, ";rest");
        let input = Span::new("@AnnotationName(Hello=\"World\");rest");
        let (input, annotation) = parse_annotation(input)?;

        assert_eq!(
            annotation,
            Annotation("AnnotationName(Hello=\"World\")".to_string())
        );
        assert_eq!(*input, ";rest");

        Ok(())
    }

    #[test]
    fn test_interface() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"
            /**
             * JavaDoc of Potato
             */
            interface Potato {
                /**
                 * const1 docu
                 */
                const int const1 = 1;

                /**
                 * method1 docu
                 */
                String method1();

                const String const2 = "two";
                int method2();
            }rest"#,
        );
        let (input, interface) = parse_item(input)?;

        assert_eq!(
            interface,
            Item::Interface {
                name: "Potato".to_string(),
                docu: " JavaDoc of Potato ".to_string(), // TODO: without spaces
                consts: vec![
                    Const::new(
                        "const1",
                        Type::with_name("int"),
                        "1",
                        " const1 docu ".to_string(),
                        Vec::new(),
                    ),
                    Const::new(
                        "const2",
                        Type::with_name("String"),
                        "two",
                        String::new(),
                        Vec::new()
                    )
                ],
                methods: vec![
                    Method::new(
                        false,
                        "method1",
                        Type::with_name("String"),
                        Vec::new(),
                        " method1 docu ".to_string(),
                        Vec::new(),
                    ),
                    Method::new(
                        false,
                        "method2",
                        Type::with_name("int"),
                        Vec::new(),
                        String::new(),
                        Vec::new(),
                    ),
                ],
                annotations: Vec::new(),
            }
        );
        assert_eq!(*input, "rest");

        Ok(())
    }
    #[test]
    fn test_interface_with_annotation() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"
            @InterfaceAnnotation1
            @InterfaceAnnotation2
            interface Potato {
            }"#,
        );
        let (_, interface) = parse_item(input)?;

        assert_eq!(
            interface,
            Item::Interface {
                name: "Potato".to_string(),
                docu: String::new(),
                consts: Vec::new(),
                methods: Vec::new(),
                annotations: vec![
                    Annotation("InterfaceAnnotation1".to_string()),
                    Annotation("InterfaceAnnotation2".to_string())
                ],
            }
        );

        Ok(())
    }

    #[test]
    fn test_error_inside_interface() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"
            /**
             * JavaDoc of Potato
             */
            interface Potato {
                String method1();
                completely_unexpected;
                int method2();
            }rest"#,
        );
        let result = parse_item(input);

        if let Err(nom::Err::Failure(e)) = result {
            assert_eq!(*e.input, "completely_unexpected");
            assert_eq!(e.input.location_line(), 7);
        } else {
            assert!(false);
        }

        Ok(())
    }

    #[test]
    fn test_parcelable() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"
            /**
             * JavaDoc of Tomato
             */
            parcelable Tomato {
                /**
                 * member1 docu
                 */
                int member1;

                String member2; // inline comment
            }rest"#,
        );
        let (input, parcelable) = parse_item(input)?;

        assert_eq!(
            parcelable,
            Item::Parcelable {
                name: "Tomato".to_string(),
                docu: " JavaDoc of Tomato ".to_string(), // TODO: without spaces
                members: vec![
                    Member::new(
                        "member1",
                        Type::with_name("int"),
                        " member1 docu ".to_string(),
                        Vec::new(),
                    ),
                    Member::new(
                        "member2",
                        Type::with_name("String"),
                        String::new(),
                        Vec::new()
                    )
                ],
                annotations: Vec::new(),
            }
        );
        assert_eq!(*input, "rest");

        Ok(())
    }

    #[test]
    fn test_enum() -> Result<(), Box<dyn std::error::Error>> {
        let input = Span::new(
            r#"
            /**
             * JavaDoc of Paprika
             */
            enum Paprika {
                /**
                 * element1 docu
                 */
                ELEMENT1 = 3,

                ELEMENT2 = "quattro",
                ELEMENT3
            }rest"#,
        );
        let (input, enumeration) = parse_item(input)?;

        assert_eq!(
            enumeration,
            Item::Enum {
                name: "Paprika".to_string(),
                docu: " JavaDoc of Paprika ".to_string(), // TODO: without spaces
                elements: vec![
                    EnumElement {
                        name: "ELEMENT1".to_string(),
                        docu: " element1 docu ".to_string(),
                        value: "3".to_string(),
                    },
                    EnumElement {
                        name: "ELEMENT2".to_string(),
                        docu: String::new(),
                        value: "quattro".to_string(),
                    },
                    EnumElement {
                        name: "ELEMENT3".to_string(),
                        docu: String::new(),
                        value: String::new(),
                    },
                ],
                annotations: Vec::new(),
            }
        );
        assert_eq!(*input, "rest");

        Ok(())
    }
}
