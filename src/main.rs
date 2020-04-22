use lib;

#[cfg(test)]
mod main_test;

pub fn main() {
    main_gen();

    lib::main();
}

fn main_gen() -> u8 {
    1
}
