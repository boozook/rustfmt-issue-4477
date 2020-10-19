// #[path = "./module_bar.rs"] // breaks the rustfmt because ./ rel path
#[path = "module_bar.rs"] // ok
mod module_bar;

#[path = "../module_foo.rs"] // breaks the rustfmt because ./ rel path
mod module_foo;
