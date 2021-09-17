#![no_std]

#[macro_export]
macro_rules! __use_stylist_item {
    (($dol:tt) , $mgr:ident, use css as $i:ident) => {
        macro_rules! $i {
            ($dol( $dol args:tt )*) => {
                ::stylist::css!($dol( $dol args )*).with_manager($mgr.clone())
            };
        }
    };
    (($dol:tt) , $mgr:ident, use style as $i:ident) => {
        macro_rules! $i {
            ($dol( $dol args:tt )*) => {
                ::stylist::style!($dol( $dol args )*).with_manager($mgr.clone())
            };
        }
    };
    (($dol:tt) , $mgr:ident, use global_style as $i:ident) => {
        macro_rules! $i {
            ($dol( $dol args:tt )*) => {
                ::stylist::global_style!($dol( $dol args )*).with_manager($mgr.clone())
            };
        }
    };
}

#[macro_export]
macro_rules! __use_stylist_item_dispatch {
    ($mgr:ident, use css as $i:ident) => {
        $crate::__use_stylist_item!(($) , $mgr, use css as $i)
    };
    ($mgr:ident, use css) => {
        $crate::__use_stylist_item!(($) , $mgr, use css as css)
    };
    ($mgr:ident, use style as $i:ident) => {
        $crate::__use_stylist_item!(($) , $mgr, use style as $i)
    };
    ($mgr:ident, use style) => {
        $crate::__use_stylist_item!(($) , $mgr, use style as style)
    };
    ($mgr:ident, use global_style as $i:ident) => {
        $crate::__use_stylist_item!(($) , $mgr, use global_style as $i)
    };
    ($mgr:ident, use global_style) => {
        $crate::__use_stylist_item!(($) , $mgr, use global_style as global_style)
    };
}

#[macro_export]
macro_rules! __use_stylist {
    ($($l:ident $(as $i:ident)?),+) => {
        let __stylist_style_manager__ =
            ::yew::functional::use_context::<::stylist::manager::StyleManager>()
                .unwrap_or_default();
        $($crate::__use_stylist_item_dispatch!(__stylist_style_manager__, use $l$( as $i)?));*
    };
}
