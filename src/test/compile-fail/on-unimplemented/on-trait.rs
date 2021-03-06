// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
// ignore-tidy-linelength

#![feature(on_unimplemented)]

#[rustc_on_unimplemented = "test error `{Self}` with `{Bar}` `{Baz}` `{Quux}`"]
trait Foo<Bar, Baz, Quux>
{}

fn foobar<U: Clone, T: Foo<u8, U, u32>>() -> T {

}

#[rustc_on_unimplemented="a collection of type `{Self}` cannot be built from an iterator over elements of type `{A}`"]
trait MyFromIterator<A> {
    /// Build a container with elements from an external iterator.
    fn my_from_iter<T: Iterator<Item=A>>(iterator: T) -> Self;
}

fn collect<A, I: Iterator<Item=A>, B: MyFromIterator<A>>(it: I) -> B {
    MyFromIterator::my_from_iter(it)
}

pub fn main() {
    let x = vec!(1u8, 2, 3, 4);
    let y: Option<Vec<u8>> = collect(x.iter()); // this should give approximately the same error for x.iter().collect()
    //~^ ERROR
    //~^^ NOTE a collection of type `std::option::Option<std::vec::Vec<u8>>` cannot be built from an iterator over elements of type `&u8`
    //~^^^ NOTE required by `collect`
    //~| NOTE trait `std::option::Option<std::vec::Vec<u8>>: MyFromIterator<&u8>` not satisfied
    let x: String = foobar(); //~ ERROR
    //~^ NOTE test error `std::string::String` with `u8` `_` `u32`
    //~^^ NOTE required by `foobar`
    //~| NOTE trait `std::string::String: Foo<u8, _, u32>` not satisfied
}
