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

macro_rules! swap_statements {
    ($x:stmt, $y:stmt) => { $y $x };
}

macro_rules! swap_function_application {
    ($f:ident ($g:ident ($args:expr) ) ) => {
        $g($f($args))
    };
}

/// Only serves to be able to show my macros expanded:
/// cargo expand --example 01_simple
fn main() {
    let mut _x = 0;
    double_statement!(_x += 2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn double_statement() {
        let mut x = 0;
        double_statement!(x += 2);
        assert_eq!(x, 4);
    }

    #[test]
    fn swap_statements() {
        let mut x = 0;
        swap_statements!(x += 3, x *= 2);
        assert_eq!(x, 3);
    }

    #[test]
    fn swap_function_application() {
        fn incr(x: i32) -> i32 {
            x + 1
        }
        fn double(x: i32) -> i32 {
            x * 2
        }

        let res = swap_function_application!(incr(double(3)));
        assert_eq!(res, 8);
    }
}
