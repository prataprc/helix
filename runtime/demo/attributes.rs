#![crate_type = "lib"]
#[test]
#[cfg(target_os = "linux")]
#[allow(non_camel_case_types)]
#![allow(unused_variables)]
#[macro_attr1] // expanded first
#[doc = mac!()] // `mac!` is expanded fourth.
#[macro_attr2] // expanded second
#[derive(MacroDerive1, MacroDerive2)] // expanded third
#[no_std]
#[doc = "example"]
#[allow(unused, clippy::inline_always)]
#[macro_use(foo, bar)]
#[link(name = "CoreFoundation", kind = "framework")]
#[rustfmt::skip]

