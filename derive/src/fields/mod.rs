use darling::{FromField, FromVariant};

pub mod enum_inner;
pub mod enum_struct;
pub mod enum_struct_field;
pub mod enum_tuple;
pub mod struct_field;
pub mod struct_index;
pub mod struct_inner;

/// A parsed struct field alongside its attributes.
#[derive(Debug, Eq, PartialEq, Clone, FromField)]
#[darling(attributes(revision), forward_attrs)]
pub struct ParsedField {
	ident: Option<syn::Ident>,
	ty: syn::Type,
	vis: syn::Visibility,
	#[darling(default)]
	start: Option<u16>,
	#[darling(default)]
	end: Option<u16>,
	#[darling(default)]
	default_fn: Option<String>,
	#[darling(default)]
	convert_fn: Option<String>,
	attrs: Vec<syn::Attribute>,
}

/// A parsed enum variant alongside its attributes.
#[derive(Debug, Eq, PartialEq, Clone, FromVariant)]
#[darling(attributes(revision), forward_attrs)]
struct ParsedEnumVariant {
	ident: syn::Ident,
	discriminant: Option<syn::Expr>,
	#[darling(default)]
	start: Option<u16>,
	#[darling(default)]
	end: Option<u16>,
	#[darling(default)]
	default_fn: Option<String>,
	#[darling(default)]
	convert_fn: Option<String>,
	fields: darling::ast::Fields<ParsedField>,
	attrs: Vec<syn::Attribute>,
}
