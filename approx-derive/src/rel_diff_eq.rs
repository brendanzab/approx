use crate::args_parsing::*;
use crate::base_types::*;
use crate::AbsDiffEqParser;

impl AbsDiffEqParser {
    pub fn get_rel_eq_single_field(
        &self,
        xi: syn::Ident,
        yi: syn::Ident,
        field_with_args: &FieldWithArgs,
    ) -> Option<proc_macro2::TokenStream> {
        if let Some(FieldFormatted {
            base_type,
            own_field,
            other_field,
            epsilon,
            max_relative,
            set_equal,
            mapping,
            use_iterator,
        }) = self.format_nth_field(0, field_with_args, Some((xi, yi)))
        {
            if set_equal {
                Some(quote::quote!(#own_field == #other_field))
            } else if let Some(map) = mapping {
                Some(quote::quote!(
                    (if let ((Some(a), Some(b))) = (
                        (#map)(#own_field),
                        (#map)(#other_field)
                    ) {
                        #ApproxName::RelativeEq::relative_eq(&a, &b, #epsilon, #max_relative)
                    } else {
                        false
                    })
                ))
            } else if use_iterator {
                Some(quote::quote!(({
                    let mut iter1 = core::iter::IntoIterator::into_iter(*#own_field);
                    let mut iter2 = core::iter::IntoIterator::into_iter(*#other_field);
                    let mut res = true;
                    loop {
                        match (iter1.next(), iter2.next()) {
                            (None, None) => break,
                            (Some(a), Some(b)) => {
                                if !#ApproxName::RelativeEq::relative_eq(a, b, #epsilon, #max_relative) {
                                    res = false;
                                    break;
                                }
                            },
                            _ => {
                                res = false;
                                break;
                            }
                        }
                    }
                    res
                })))
            } else {
                Some(quote::quote!(
                    <#base_type as #ApproxName::RelativeEq>::relative_eq(
                        #own_field,
                        #other_field,
                        #epsilon,
                        #max_relative
                    )
                ))
            }
        } else {
            None
        }
    }

    fn get_rel_eq_struct_fields(
        &self,
        fields_with_args: &[FieldWithArgs],
    ) -> Vec<proc_macro2::TokenStream> {
        // We need to extend the where clause for all generics
        let fields = fields_with_args
            .iter()
            .enumerate()
            .filter_map(|(n, field_with_args)| {
                if let Some(FieldFormatted {
                    base_type,
                    own_field,
                    other_field,
                    epsilon,
                    #[allow(unused)]
                    max_relative,
                    set_equal,
                    mapping,
                    use_iterator,
                }) = self.format_nth_field(n, field_with_args, None)
                {
                    if set_equal {
                        Some(quote::quote!(#own_field == #other_field &&))
                    } else if let Some(map) = mapping {
                        Some(quote::quote!(
                            (if let ((Some(a), Some(b))) = (
                                (#map)(#own_field),
                                (#map)(#other_field)
                            ) {
                                #ApproxName::RelativeEq::relative_eq(&a, &b, #epsilon, #max_relative)
                            } else {
                                false
                            }) &&
                        ))
                    } else if use_iterator {
                        Some(quote::quote!(({
                            let mut iter1 = core::iter::IntoIterator::into_iter(#own_field);
                            let mut iter2 = core::iter::IntoIterator::into_iter(#other_field);
                            let mut res = true;
                            loop {
                                match (iter1.next(), iter2.next()) {
                                    (None, None) => break,
                                    (Some(a), Some(b)) => {
                                        if !#ApproxName::RelativeEq::relative_eq(
                                                a,
                                                b,
                                                #epsilon,
                                                #max_relative
                                            ) {
                                            res = false;
                                            break;
                                        }
                                    },
                                    _ => {
                                        res = false;
                                        break;
                                    }
                                }
                            }
                            res
                        }) &&))
                    } else {
                        Some(quote::quote!(
                            <#base_type as #ApproxName::RelativeEq>::relative_eq(
                                #own_field,
                                #other_field,
                                #epsilon,
                                #max_relative,
                            ) &&
                        ))
                    }
                } else {
                    None
                }
            });
        fields.collect()
    }

    fn get_rel_eq_variants(
        &self,
        variants_with_args: &[EnumVariant],
    ) -> Vec<proc_macro2::TokenStream> {
        variants_with_args
            .iter()
            .map(|variant_with_args| {
                let variant = &variant_with_args.ident;
                use syn::spanned::Spanned;

                let gen_field_names = |var: &str| -> Vec<syn::Ident> {
                    variant_with_args
                        .fields_with_args
                        .iter()
                        .enumerate()
                        .map(|(n, field)| syn::Ident::new(&format!("{var}{n}"), field.ident.span()))
                        .collect()
                };
                if variant_with_args
                    .fields_with_args
                    .first()
                    .and_then(|f| f.ident.clone())
                    .is_some()
                {
                    let field_placeholders1 = gen_field_names("x");
                    let field_placeholders2 = gen_field_names("y");
                    let gen_combos = |iterator: Vec<syn::Ident>| {
                        iterator
                            .iter()
                            .zip(&variant_with_args.fields_with_args)
                            .map(|(fph, fwa)| {
                                let id = &fwa.ident;
                                quote::quote!(#id: #fph)
                            })
                            .collect::<Vec<_>>()
                    };
                    let comps: Vec<_> = field_placeholders1
                        .iter()
                        .zip(field_placeholders2.iter())
                        .zip(variant_with_args.fields_with_args.iter())
                        .map(|((xi, yi), field)| {
                            self.get_rel_eq_single_field(xi.clone(), yi.clone(), field)
                        })
                        .collect();
                    let field_name_placeholder_combos1 = gen_combos(field_placeholders1);
                    let field_name_placeholder_combos2 = gen_combos(field_placeholders2);
                    quote::quote!(
                        (
                            Self:: #variant {
                                #(#field_name_placeholder_combos1),*
                            },
                            Self:: #variant {
                                #(#field_name_placeholder_combos2),*
                            }
                        ) => #(#comps) &&*,
                    )
                } else if !variant_with_args.fields_with_args.is_empty() {
                    let field_names1 = gen_field_names("x");
                    let field_names2 = gen_field_names("y");
                    let comps: Vec<_> = field_names1
                        .iter()
                        .zip(field_names2.iter())
                        .zip(variant_with_args.fields_with_args.iter())
                        .map(|((xi, yi), field)| {
                            self.get_rel_eq_single_field(xi.clone(), yi.clone(), field)
                        })
                        .collect();
                    quote::quote!(
                        (
                            Self:: #variant (#(#field_names1),*),
                            Self:: #variant (#(#field_names2),*)
                        ) => {#(#comps) &&*},
                    )
                } else {
                    quote::quote!(
                        (Self::#variant, Self::#variant) => true,
                    )
                }
            })
            .collect()
    }

    pub fn implement_derive_rel_diff_eq(&self) -> proc_macro2::TokenStream {
        let obj_name = &self.base_type.ident();
        let epsilon_type = self.get_epsilon_parent_type();
        let max_relative_default_value = self.get_max_relative_default_value();

        let (impl_generics, ty_generics, _) = self.base_type.generics().split_for_impl();
        let where_clause = self.generate_where_clause(false);

        match &self.base_type {
            #[allow(unused)]
            BaseType::Struct {
                item_struct,
                fields_with_args,
            } => {
                let fields = self.get_rel_eq_struct_fields(fields_with_args);

                quote::quote!(
                    const _ : () = {
                        #[automatically_derived]
                        impl #impl_generics #ApproxName::RelativeEq for #obj_name #ty_generics
                        #where_clause
                        {
                            fn default_relative_epsilon() -> Self::Epsilon {
                                <#epsilon_type as #ApproxName::RelativeEq>::default_relative_epsilon()
                            }

                            fn default_max_relative() -> Self::Epsilon {
                                #max_relative_default_value
                            }

                            fn relative_eq(
                                &self,
                                other: &Self,
                                epsilon: Self::Epsilon,
                                max_relative: Self::Epsilon
                            ) -> bool {
                                #(#fields)*
                                true
                            }
                        }
                    };
                )
            }
            #[allow(unused)]
            BaseType::Enum {
                item_enum,
                variants_with_args,
            } => {
                let variants = self.get_rel_eq_variants(variants_with_args);
                quote::quote!(
                    const _: () = {
                        #[automatically_derived]
                        impl #impl_generics #ApproxName::RelativeEq for #obj_name #ty_generics
                        #where_clause
                        {
                            fn default_relative_epsilon() -> Self::Epsilon {
                                <#epsilon_type as #ApproxName::RelativeEq>::default_relative_epsilon()
                            }

                            fn default_max_relative() -> Self::Epsilon {
                                #max_relative_default_value
                            }

                            fn relative_eq(
                                &self,
                                other: &Self,
                                epsilon: Self::Epsilon,
                                max_relative: Self::Epsilon
                            ) -> bool {
                                match (self, other) {
                                    #(#variants)*
                                    _ => false,
                                }
                            }
                        }
                    };
                )
            }
        }
    }
}
