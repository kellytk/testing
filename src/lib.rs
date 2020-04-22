pub mod foo;

#[cfg(test)]
mod lib_test;

pub fn main() {
    lib_gen();

    foo::foo_gen();

    println!("Hello, {}", lib_gen() + foo::foo_gen());
}

pub fn lib_gen() -> u8 {
    1
}
