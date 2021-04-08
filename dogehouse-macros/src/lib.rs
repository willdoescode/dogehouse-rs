extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
  println!("attr: \"{}\"", attr.to_string());
  println!("item: \"{}\"", item.to_string());
  item
}

