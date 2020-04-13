use proc_macro2::TokenStream;
use syn::{
    Attribute, Data, DeriveInput, Fields, FieldsNamed, GenericArgument, Generics, Ident, Lit, Meta,
    NestedMeta, Path, PathArguments, Type, TypePath,
};

pub fn expand_derive_vst3_impl(input: &DeriveInput) -> Result<TokenStream, String> {
    let vst3_impl = Vst3Impl::parse(input)?;
    let result = vst3_impl.quote();
    Ok(result)
}

struct Vst3Impl<'a> {
    name: &'a Ident,
    vtbl_member: &'a Ident,
    refc_member: &'a Ident,
    other_members: Vec<Mem<'a>>,
    interfaces: Vec<Type>,
    generics: &'a Generics,
}

impl<'a> Vst3Impl<'a> {
    fn quote(&self) -> TokenStream {
        let create_raw = self.quote_create_raw();
        let funknown_vtbl = self.quote_funknown_vtbl();
        let funknown_impl = self.quote_funknown_impl();

        quote! {
            #create_raw
            #funknown_vtbl
            #funknown_impl
        }
    }

    fn quote_create_raw(&self) -> TokenStream {
        let name = self.name;
        let vtbl = self.vtbl_member;
        let refcount = self.refc_member;
        let (impgen, tygen, wherec) = self.generics.split_for_impl();
        let params = self.other_members.iter().map(|m| m.quote_param());
        let inits = self.other_members.iter().map(|m| m.quote_init());

        quote! {
            impl #impgen #name #tygen #wherec {
                fn create_raw(#(#params),*) -> *mut Self {
                    Box::into_raw(Box::new(#name {
                        #vtbl: <Self as vst3_impl::BuildVTable<_>>::static_vtable(),
                        #refcount: Default::default(),
                        #(#inits,)*
                    }))
                }
            }
        }
    }

    fn quote_funknown_vtbl(&self) -> TokenStream {
        let name = self.name;
        let (impgen, tygen, wherec) = self.generics.split_for_impl();
        let buildvtbl = quote! { vst3_impl::BuildVTable<vst3_interfaces::FUnknownVtbl> };

        quote! {
            unsafe impl #impgen #buildvtbl for #name #tygen #wherec {
                const VTBL: vst3_interfaces::FUnknownVtbl = vst3_interfaces::FUnknownVtbl {
                    addRef: Self::__vst3_impl__FUnknown__addRef,
                    release: Self::__vst3_impl__FUnknown__release,
                    queryInterface: Self::__vst3_impl__FUnknown__queryInterface,
                };

                fn static_vtable() -> vst3_impl::VTable<vst3_interfaces::FUnknownVtbl> {
                    vst3_impl::VTable::new(&Self::VTBL)
                }
            }
        }
    }

    fn quote_funknown_impl(&self) -> TokenStream {
        let name = self.name;
        let refcount = self.refc_member;
        let (impgen, tygen, wherec) = self.generics.split_for_impl();
        let is_equal_iid = self.interfaces.iter().map(|path| {
            quote! { {
                let mut iid_ = <#path as vst3_interfaces::Interface>::iid();
                vst3_interfaces::is_iid_equal (
                    riid,
                    iid_.as_ptr()
                )
                }
            }
        });

        quote! {
            #[allow(non_snake_case)]
            impl #impgen #name #tygen #wherec {
                #[inline(never)]
                unsafe extern "system" fn __vst3_impl__FUnknown__addRef(
                    this: *mut vst3_interfaces::FUnknown,
                ) -> u32 {
                    let this = &*(this as *const Self);
                    this.#refcount.add_ref()
                }

                #[inline(never)]
                unsafe extern "system" fn __vst3_impl__FUnknown__release(
                    this: *mut vst3_interfaces::FUnknown,
                ) -> u32 {
                    let ptr = this as *mut Self;
                    let count = (*ptr).#refcount.release();
                    if count == 0 {
                        // This was the last ref
                        Box::from_raw(ptr);
                    }
                    count
                }

                #[inline(never)]
                unsafe extern "system" fn __vst3_impl__FUnknown__queryInterface(
                    this: *mut vst3_interfaces::FUnknown,
                    riid: *const i8,
                    ppv: *mut *mut std::os::raw::c_void,
                ) -> tresult {
                    if ppv.is_null() {
                        return -1;
                    }
                    if #( #is_equal_iid )||* {
                        *ppv = this as *mut std::os::raw::c_void;
                        0
                    } else {
                        *ppv = std::ptr::null_mut();
                        -1
                    }
                }
            }
        }
    }

    // ----------------------------------------------------------------

    fn parse(input: &'a DeriveInput) -> Result<Self, String> {
        if !Self::is_repr_c(input) {
            return Err("Your struct *must* be #[repr(C)] for Vst3Impl.".into());
        }

        let data = match &input.data {
            Data::Struct(data) => data,
            _ => return Err("Vst3Impl will only work with structs with named members.".into()),
        };
        let fields = match &data.fields {
            Fields::Named(fields) => fields,
            _ => return Err("Vst3Impl will only work with structs with named members.".into()),
        };

        let name = &input.ident;
        let vtbl_member = Self::determine_vtbl_member(fields)?;
        let refc_member = Self::determine_refcount_member(fields)?;
        let other_members = Self::parse_members(fields, vtbl_member, refc_member);
        let interfaces = Self::determine_interfaces(&input.attrs, fields, vtbl_member)?;
        let generics = &input.generics;

        Ok(Vst3Impl {
            name,
            vtbl_member,
            refc_member,
            other_members,
            interfaces,
            generics,
        })
    }

    fn is_repr_c(input: &'a DeriveInput) -> bool {
        for attr in &input.attrs {
            if attr.path.segments.len() != 1 || attr.path.segments[0].ident != "repr" {
                continue;
            }

            let meta = match attr.parse_meta() {
                Ok(meta) => meta,
                Err(_) => continue,
            };

            let list = match &meta {
                Meta::List(list) if list.nested.len() > 0 => list,
                _ => continue,
            };

            match &list.nested[0] {
                NestedMeta::Meta(Meta::Word(id)) if id == "C" => return true,
                _ => continue,
            }
        }
        false
    }

    fn determine_vtbl_member(fields: &FieldsNamed) -> Result<&Ident, String> {
        for field in fields.named.iter() {
            let ty = Self::ty_stem(&field.ty);
            let ty = match ty {
                Some(ty) => ty,
                None => continue,
            };
            if ty != "VTable" {
                continue;
            }

            return Ok(field.ident.as_ref().unwrap());
        }

        Err("Could not find a vst3_impl::VTable member".into())
    }

    fn determine_refcount_member(fields: &FieldsNamed) -> Result<&Ident, String> {
        for field in fields.named.iter() {
            let ty = Self::ty_stem(&field.ty);
            let ty = match ty {
                Some(ty) => ty,
                None => continue,
            };
            if ty != "Refcount" {
                continue;
            }

            return Ok(field.ident.as_ref().unwrap());
        }

        Err("Could not find a vst3_impl::Refcount member".into())
    }

    fn parse_members<'b>(fields: &'b FieldsNamed, vtbl: &Ident, refc: &Ident) -> Vec<Mem<'b>> {
        fields
            .named
            .iter()
            .filter_map(|f| {
                let name = f.ident.as_ref().unwrap();
                if name == vtbl || name == refc {
                    return None;
                }
                let ty = &f.ty;
                Some(Mem { name, ty })
            })
            .collect()
    }

    fn determine_interfaces(
        attrs: &[Attribute],
        fields: &FieldsNamed,
        vtbl: &Ident,
    ) -> Result<Vec<Type>, String> {
        for attr in attrs {
            if attr.path.segments.len() != 1 || attr.path.segments[0].ident != "interfaces" {
                continue;
            }

            let meta = attr.parse_meta().map_err(|e| e.to_string())?;
            let list = match &meta {
                Meta::List(list) => list,
                _ => return Err("Invalid syntax for #[interfaces]".into()),
            };

            let interfaces = Some(Ok(Self::funknown_path()))
                .into_iter()
                .chain(list.nested.iter().map(|m| match m {
                    NestedMeta::Meta(Meta::Word(word)) => Ok(Type::from(TypePath {
                        qself: None,
                        path: Path::from(word.clone()),
                    })),
                    NestedMeta::Literal(Lit::Str(lit)) => {
                        syn::parse_str(&lit.value()).map_err(|e| e.to_string())
                    }
                    _ => Err("Bad syntax for #[interfaces]".into()),
                }))
                .collect();

            return interfaces;
        }

        for field in fields.named.iter() {
            if field.ident.as_ref() != Some(vtbl) {
                continue;
            }
            let mut vtbl_ty = Self::vtbl_generic(&field.ty)?.clone();
            match &mut vtbl_ty {
                Type::Path(path) => {
                    let mut last = path.path.segments.last_mut().unwrap();
                    let mut last = last.value_mut();
                    let s = last.ident.to_string();
                    if s.ends_with("Vtbl") {
                        let nonv = &s[..s.len() - 4];
                        if nonv == "FUnknown" {
                            return Ok(vec![Self::funknown_path()]);
                        }
                        let new_end = Ident::new(nonv, last.ident.span());
                        last.ident = new_end;
                    } else {
                        break;
                    }
                }
                _ => unreachable!(),
            };

            return Ok(vec![Self::funknown_path(), vtbl_ty]);
        }

        Err("Could not determine the Vst3 interfaces you would like to implement.".into())
    }

    fn funknown_path() -> Type {
        syn::parse_str("vst3_interfaces::FUnknown").unwrap()
    }

    fn vtbl_generic(ty: &Type) -> Result<&Type, String> {
        let segments = match ty {
            Type::Path(typath) => &typath.path.segments,
            _ => return Err("A Vst3Impl struct must have a VTable member.".into()),
        };

        let final_seg = match segments.last() {
            Some(seg) => *seg.value(),
            None => return Err("A Vst3Impl struct must have a VTable member.".into()),
        };

        if final_seg.ident != "VTable" {
            return Err("A Vst3Impl struct must have a VTable member.".into());
        }

        let args = match &final_seg.arguments {
            PathArguments::AngleBracketed(args) => &args.args,
            _ => return Err("Invalid generic arguments to VTable.".into()),
        };

        if args.len() != 1 {
            return Err("Invalid generic arguments to VTable.".into());
        }

        let itype = match &args[0] {
            GenericArgument::Type(ty) => ty,
            _ => return Err("Invalid generic arguments to VTable.".into()),
        };

        Ok(itype)
    }

    fn ty_stem(ty: &Type) -> Option<&Ident> {
        let segments = match ty {
            Type::Path(typath) => &typath.path.segments,
            _ => return None,
        };

        let final_seg = *segments.last()?.value();
        Some(&final_seg.ident)
    }
}

struct Mem<'a> {
    name: &'a Ident,
    ty: &'a Type,
}

impl<'a> Mem<'a> {
    fn quote_param(&self) -> TokenStream {
        let (name, ty) = (self.name, self.ty);
        quote! { #name: #ty }
    }

    fn quote_init(&self) -> TokenStream {
        let name = self.name;
        quote! { #name: #name }
    }
}
