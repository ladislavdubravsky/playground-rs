#![allow(unused_macros)]
#![allow(dead_code)]

macro_rules! identity {
    () => {};
}

macro_rules! hello {
    () => {
        println!("Hello!");
    };
}

macro_rules! double_statement {
    ($x:stmt) => { $x $x };
}

/// Only serves to be able to show my macros expanded:
/// cargo expand --lib
fn main() {
    let mut _x = 0;
    double_statement!(_x += 2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn double() {
        let mut x = 0;
        double_statement!(x += 2);
        assert_eq!(x, 4);
    }
}
