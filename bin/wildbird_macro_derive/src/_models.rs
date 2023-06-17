#![allow(dead_code, unused_imports)]

use core::fmt;
use std::fmt::Debug;
use quote::ToTokens;
use syn::{Field, Fields, Type, Ident};

#[derive(Debug)]
pub struct WType {
    pub name: String,
    pub generic: Option<Box<WType>>,
}

#[derive(Debug)]
pub struct WAtribute {
    name: String,
    value: String,
}

pub struct WField<'a> {
    pub name: Option<&'a Ident>,
    pub ty: &'a Type,
}

impl<'z> From<&'z Field> for WField<'z> {
    fn from(value: &'z Field) -> Self {
        WField {
            name: value.ident.as_ref(),
            ty: &value.ty,
        }
    }
}

impl Debug for WField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("WField")
            .field(&format! {"{}: {}", "name", &self.name_str()})
            .field(&format! {"{}: {}", "ty", &self.type_str()})
            .finish()
    }
}

impl WField<'_> {
    fn name_str(&self) -> String {
        self.name.to_token_stream()
            .to_string()
    }

    fn type_str(&self) -> String {
        self.ty.to_token_stream()
            .to_string()
            .replace(" ", "")
    }
}

pub fn parse_fields(fileds: &Fields) -> Vec<WField> {
    let mut result: Vec<WField> = vec![];

    for f in fileds.iter() {
        result.push(f.into());
    }

    result

    // for f in fileds.iter() {
    //     let n = f.ident.to_token_stream().to_string();
    //     let t = f.ty.to_token_stream().to_string().replace(" ", "");
    //     t.span();
    //
    //     println!("field: {n}: {t}")
    //
    // }
    //
    // match fileds {
    //     Fields::Named(named) => {
    //         for pair in named.named.pairs() {
    //             let field = pair.value();
    //             let name = field.ident.as_ref().map(|i| i.to_string()).unwrap_or("".to_string());
    //
    //             match &field.ty {
    //                 Type::Array(_) => { println!("Array") }
    //                 Type::BareFn(_) => { println!("BareFn") }
    //                 Type::Group(_) => { println!("Group") }
    //                 Type::ImplTrait(_) => { println!("ImplTrait") }
    //                 Type::Infer(_) => { println!("Infer") }
    //                 Type::Macro(_) => { println!("Macro") }
    //                 Type::Never(_) => { println!("Never") }
    //                 Type::Paren(_) => { println!("Paren") }
    //                 Type::Path(path) => {
    //                     if let Some(indent) = path.path.get_ident() {
    //                         result.push(
    //                             WField {
    //                                 name: name,
    //                                 ty: WType { name: indent.to_string(), generic: None },
    //                             }
    //                         );
    //                     };
    //
    //
    //                     for segment in path.path.segments.iter() {
    //                         let s = segment.ident.to_string();
    //                         println!("s: {}", s);
    //
    //
    //                         if let PathArguments::AngleBracketed(b) = &segment.arguments {
    //                             for x in b.args.iter() {
    //                                 match x {
    //                                     GenericArgument::Lifetime(_) => {}
    //                                     GenericArgument::Type(t) => {
    //                                         if let Type::Path(p) = t {
    //                                             for x in p.path.segments.iter() {
    //                                                 let s = x.ident.to_string();
    //                                                 println!("s: {:?}", s)
    //                                             }
    //                                         }
    //                                     }
    //                                     GenericArgument::Const(_) => {}
    //                                     GenericArgument::AssocType(_) => {}
    //                                     GenericArgument::AssocConst(_) => {}
    //                                     GenericArgument::Constraint(_) => {}
    //                                     _ => {}
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 }
    //                 Type::Ptr(_) => { println!("Ptr") }
    //                 Type::Reference(_) => { println!("Reference") }
    //                 Type::Slice(_) => { println!("Slice") }
    //                 Type::TraitObject(_) => { println!("TraitObject") }
    //                 Type::Tuple(_) => { println!("Tuple") }
    //                 Type::Verbatim(_) => { println!("Verbatim") }
    //                 _ => {
    //                     println!("Unknown")
    //                 }
    //             };
    //         }
    //
    //         for field in named.named.pairs() {
    //             let t = field.value().to_token_stream();
    //             println!("pair: {}", t)
    //         }
    //
    //         for field in named.named.iter() {
    //             let t = field.to_token_stream();
    //             println!("field: {}", t)
    //         }
    //     }
    //     Fields::Unnamed(unamed) => {
    //         println!("Unnamed")
    //     }
    //     Fields::Unit => {
    //         println!("unit")
    //     }
    // }
    //
    // result
}

#[cfg(test)]
mod parser_tests {
    use proc_macro::TokenStream;

    use quote::*;
    use syn::*;

    use super::parse_fields;
    use crate::_test_utils::*;
    use crate::q;

    #[test]
    fn should_parse_fields() {
        let r: ItemStruct = q!(
                struct Test {
                    name: String,
                    opt: Option<u32>
                }
        );

        println!("{:?}", &r.to_token_stream());
        let parsed = DeriveInput::from(r);

        let Data::Struct(data) = parsed.data else { panic!("invalid type") };
        let fields = parse_fields(&data.fields);
        println!("{:#?}", fields)
    }
}