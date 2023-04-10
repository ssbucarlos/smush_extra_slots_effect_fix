#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]

mod fox;
mod sonic;
mod helper;
mod yoshi;
mod packun;
mod mewtwo;
mod samusd;
mod duckhunt;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    sonic::install();
    fox::install();
    yoshi::install();
    packun::install();
    mewtwo::install();
    samusd::install();
    duckhunt::install();
}