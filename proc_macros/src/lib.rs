use proc_macro::TokenStream;

#[proc_macro]
pub fn get_third_token(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    iter.next();
    iter.next();
    match iter.next() {
        Some(token) => token.into(),
        None => panic!("Expected at least three tokens"),
    }
}
