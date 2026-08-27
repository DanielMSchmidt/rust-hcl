//! Derive macros mirroring gohcl's struct-tag reflection (hcl v2: `gohcl`).
//!
//! Go's gohcl package drives decoding with struct tags like
//! `hcl:"name,attr"`. The Rust analogue is a derive macro plus `#[hcl(...)]`
//! helper attributes:
//!
//! | Go struct tag        | Rust field attribute            |
//! |----------------------|---------------------------------|
//! | `hcl:"name"` / `hcl:"name,attr"` | `#[hcl(attr = "name")]` |
//! | `hcl:"name,optional"`| `#[hcl(attr = "name", optional)]` |
//! | `hcl:"type,block"`   | `#[hcl(block = "type")]`        |
//! | `hcl:"name,label"`   | `#[hcl(label = "name")]`        |
//! | `hcl:",remain"`      | `#[hcl(remain)]`                |
//! | `hcl:",body"`        | `#[hcl(body)]`                  |
//! | `hcl:",range"`       | `#[hcl(range)]`                 |
//! | `hcl:"name,label_range"` | `#[hcl(label_range = "name")]` |
//! | `hcl:",def_range"`   | `#[hcl(def_range)]`             |
//! | `hcl:",type_range"`  | `#[hcl(type_range)]`            |
//! | `hcl:"name,attr_range"` | `#[hcl(attr_range = "name")]` |
//! | `hcl:"name,attr_name_range"` | `#[hcl(attr_name_range = "name")]` |
//! | `hcl:"name,attr_value_range"` | `#[hcl(attr_value_range = "name")]` |
//!
//! `#[hcl(optional)]` on its own is shorthand for
//! `#[hcl(attr = "<field name>", optional)]`. Fields without an `#[hcl(...)]`
//! attribute are ignored, as gohcl ignores fields without an `hcl:` tag.
//!
//! **Stub phase:** the derives parse and validate the attribute grammar so
//! annotated structs compile, but every emitted method body is `todo!()`.
//! No decoding or encoding logic is implemented here.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Which kind of field mapping an `#[hcl(...)]` attribute selects.
#[derive(PartialEq, Clone, Copy)]
enum FieldKind {
    Attr,
    Block,
    Label,
    Remain,
    Body,
    Range,
    LabelRange,
    DefRange,
    TypeRange,
    AttrRange,
    AttrNameRange,
    AttrValueRange,
}

/// One parsed `#[hcl(...)]` field annotation.
struct FieldSpec {
    kind: FieldKind,
    /// The HCL-side name (attribute name, block type, or label name).
    /// `None` for `remain`/`body`/`range`, and for a bare `optional`,
    /// where the field name is used.
    name: Option<String>,
    optional: bool,
}

/// Parses the `#[hcl(...)]` attributes on one struct field, enforcing the
/// grammar documented at the crate root. Returns `Ok(None)` for fields with
/// no `#[hcl(...)]` attribute (ignored, as in gohcl).
fn parse_field_attrs(field: &syn::Field) -> syn::Result<Option<FieldSpec>> {
    let mut spec: Option<FieldSpec> = None;
    for attr in &field.attrs {
        if !attr.path().is_ident("hcl") {
            continue;
        }
        let mut kind: Option<FieldKind> = None;
        let mut name: Option<String> = None;
        let mut optional = false;
        attr.parse_nested_meta(|meta| {
            let set_kind = |kind_slot: &mut Option<FieldKind>, k: FieldKind| {
                if kind_slot.is_some() {
                    return Err(meta.error("conflicting #[hcl(...)] field kinds"));
                }
                *kind_slot = Some(k);
                Ok(())
            };
            if meta.path.is_ident("attr") {
                set_kind(&mut kind, FieldKind::Attr)?;
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else if meta.path.is_ident("block") {
                set_kind(&mut kind, FieldKind::Block)?;
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else if meta.path.is_ident("label") {
                set_kind(&mut kind, FieldKind::Label)?;
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else if meta.path.is_ident("label_range") {
                set_kind(&mut kind, FieldKind::LabelRange)?;
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else if meta.path.is_ident("attr_range") {
                set_kind(&mut kind, FieldKind::AttrRange)?;
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else if meta.path.is_ident("attr_name_range") {
                set_kind(&mut kind, FieldKind::AttrNameRange)?;
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else if meta.path.is_ident("attr_value_range") {
                set_kind(&mut kind, FieldKind::AttrValueRange)?;
                name = Some(meta.value()?.parse::<syn::LitStr>()?.value());
                Ok(())
            } else if meta.path.is_ident("remain") {
                set_kind(&mut kind, FieldKind::Remain)
            } else if meta.path.is_ident("body") {
                set_kind(&mut kind, FieldKind::Body)
            } else if meta.path.is_ident("range") {
                set_kind(&mut kind, FieldKind::Range)
            } else if meta.path.is_ident("def_range") {
                set_kind(&mut kind, FieldKind::DefRange)
            } else if meta.path.is_ident("type_range") {
                set_kind(&mut kind, FieldKind::TypeRange)
            } else if meta.path.is_ident("optional") {
                optional = true;
                Ok(())
            } else {
                Err(meta.error(
                    "unknown #[hcl(...)] argument; expected one of \
                     attr, block, label, remain, body, range, label_range, \
                     def_range, type_range, attr_range, attr_name_range, \
                     attr_value_range, optional",
                ))
            }
        })?;
        // A bare `#[hcl(optional)]` means an optional attribute named after
        // the field.
        let kind = kind.unwrap_or(FieldKind::Attr);
        if optional && kind != FieldKind::Attr {
            return Err(syn::Error::new_spanned(
                attr,
                "#[hcl(optional)] only applies to attribute fields",
            ));
        }
        if kind == FieldKind::Attr && name.is_none() {
            if !optional {
                return Err(syn::Error::new_spanned(
                    attr,
                    "#[hcl(...)] needs a field kind, e.g. #[hcl(attr = \"name\")]",
                ));
            }
            name = field.ident.as_ref().map(|id| id.to_string());
        }
        if spec.is_some() {
            return Err(syn::Error::new_spanned(
                attr,
                "duplicate #[hcl(...)] attribute on field",
            ));
        }
        spec = Some(FieldSpec {
            kind,
            name,
            optional,
        });
    }
    Ok(spec)
}

/// Validates the `#[hcl(...)]` grammar across a whole struct. The parsed
/// specs are discarded afterwards: in the stub phase the derives emit
/// `todo!()` bodies, so only grammar acceptance matters.
fn validate_struct(input: &DeriveInput) -> syn::Result<()> {
    let Data::Struct(data) = &input.data else {
        return Err(syn::Error::new_spanned(
            input,
            "hcl derives only support structs with named fields",
        ));
    };
    let Fields::Named(fields) = &data.fields else {
        return Err(syn::Error::new_spanned(
            input,
            "hcl derives only support structs with named fields",
        ));
    };
    for field in &fields.named {
        let spec = parse_field_attrs(field)?;
        // Silence the "never read" warnings while the emitted bodies are
        // todo!(): the parsed values are what a future implementation will
        // consume.
        if let Some(spec) = spec {
            let _ = (spec.kind, spec.name, spec.optional);
        }
    }
    Ok(())
}

/// Derives `hcl::gohcl::FromBody` (hcl v2: `gohcl.DecodeBody` /
/// `gohcl.ImpliedBodySchema` struct-tag reflection).
///
/// The emitted impl's method bodies are `todo!()`; the derive exists so that
/// annotated structs compile against the stubbed API.
#[proc_macro_derive(FromBody, attributes(hcl))]
pub fn derive_from_body(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Err(err) = validate_struct(&input) {
        return err.to_compile_error().into();
    }
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics ::hcl::gohcl::FromBody for #name #ty_generics #where_clause {
            fn from_body(
                body: &dyn ::hcl::Body,
                ctx: ::core::option::Option<&::hcl::EvalContext>,
            ) -> (Self, ::hcl::Diagnostics) {
                let _ = (body, ctx);
                todo!()
            }

            fn implied_body_schema() -> (::hcl::BodySchema, bool) {
                todo!()
            }
        }
    };
    expanded.into()
}

/// Derives `hcl::gohcl::EncodeBody` (hcl v2: `gohcl.EncodeIntoBody` /
/// `gohcl.EncodeAsBlock` struct-tag reflection).
///
/// The emitted impl's method bodies are `todo!()`; the derive exists so that
/// annotated structs compile against the stubbed API.
#[proc_macro_derive(EncodeBody, attributes(hcl))]
pub fn derive_encode_body(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let Err(err) = validate_struct(&input) {
        return err.to_compile_error().into();
    }
    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let expanded = quote! {
        impl #impl_generics ::hcl::gohcl::EncodeBody for #name #ty_generics #where_clause {
            fn encode_into_body(&self, dst: &::hcl::hclwrite::Body) {
                let _ = dst;
                todo!()
            }

            fn encode_as_block(&self, block_type: &str) -> ::hcl::hclwrite::Block {
                let _ = block_type;
                todo!()
            }
        }
    };
    expanded.into()
}
