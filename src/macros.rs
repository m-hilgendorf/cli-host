// todo: modify to be more compatible with VST3, less windows focused 
// From the winapi crate
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

#[macro_export]
macro_rules! RIDL {
    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) {$(
        $(#[$($attrs:tt)*])* fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    )+}) => (
        RIDL!{@vtbl $interface $vtbl () $(
            $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr,
        )+}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        impl $interface {
            $(RIDL!{@method $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr})+
        }
        RIDL!{@uuid $interface $($uuid),+}
    );

    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {}) => (
        RIDL!{@vtbl $interface $vtbl (pub parent: $pvtbl,)}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        RIDL!{@deref $interface $pinterface}
        RIDL!{@uuid $interface $($uuid),+}
    );

    (#[uuid($($uuid:expr),+)]
    interface $interface:ident ($vtbl:ident) : $pinterface:ident ($pvtbl:ident) {$(
        $(#[$($attrs:tt)*])* fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    )+}) => (
        RIDL!{@vtbl $interface $vtbl (pub parent: $pvtbl,) $(
            $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr,
        )+}
        #[repr(C)]
        pub struct $interface {
            pub lpVtbl: *const $vtbl,
        }
        impl $interface {
            $(RIDL!{@method $(#[$($attrs)*])* fn $method($($p: $t,)*) -> $rtr})+
        }
        RIDL!{@deref $interface $pinterface}
        RIDL!{@uuid $interface $($uuid),+}
    );

    (@deref $interface:ident $pinterface:ident) => (
        impl $crate::_core::ops::Deref for $interface {
            type Target = $pinterface;
            #[inline]
            fn deref(&self) -> &$pinterface {
                unsafe { &*(self as *const $interface as *const $pinterface) }
            }
        }
    );

    (@method fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty) => (
        #[inline] pub unsafe fn $method(&self, $($p: $t,)*) -> $rtr {
            ((*self.lpVtbl).$method)(self as *const _ as *mut _, $($p,)*)
        }
    );

    (@method #[fixme] fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty) => (
        #[inline] pub unsafe fn $method(&self, $($p: $t,)*) -> $rtr {
            let mut ret = $crate::_core::mem::uninitialized();
            ((*self.lpVtbl).$method)(self as *const _ as *mut _, &mut ret, $($p,)*);
            ret
        }
    );

    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        $(fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,)*
    ) => (
        RIDL!{@item #[repr(C)]
        pub struct $vtbl {
            $($fields)*
            $(pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                $($p: $t,)*
            ) -> $rtr,)*
        }}
    );

    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    $($tail:tt)*) => (
        RIDL!{@vtbl $interface $vtbl (
            $($fields)*
            pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                $($p: $t,)*
            ) -> $rtr,
        ) $($tail)*}
    );

    (@vtbl $interface:ident $vtbl:ident ($($fields:tt)*)
        #[fixme] fn $method:ident($($p:ident : $t:ty,)*) -> $rtr:ty,
    $($tail:tt)*) => (
        RIDL!{@vtbl $interface $vtbl (
            $($fields)*
            pub $method: unsafe extern "system" fn(
                This: *mut $interface,
                ret: *mut $rtr,
                $($p: $t,)*
            ) -> *mut $rtr,
        ) $($tail)*}
    );

    (@uuid $interface:ident
        $w0:expr, $w1:expr, $w2:expr, $w3:expr
    ) => (
        impl $crate::Interface for $interface {
            #[inline]
            fn uuidof() -> $crate::pluginterfaces::TUID {
                let bytes : [u32; 4] = [$w0, $w1, $w2, $w3];
                let mut tuid  : [i8; 16] = [0;16];
                for i in 0..4 {
                    let big_e = bytes[i].to_be_bytes();
                    for k in 0..4 {
                        tuid[i*4 + k] = unsafe { std::mem::transmute(big_e[k]) };
                    }
                }
                tuid
            }
        }
    );

    (@item $thing:item) => ($thing);
}