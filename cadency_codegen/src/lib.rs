#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod derive;

#[proc_macro_derive(CommandBaseline, attributes(name, description, deferred))]
pub fn derive_command_baseline(input_item: TokenStream) -> TokenStream {
    // Parse token stream into derive syntax tree
    let tree: DeriveInput = parse_macro_input!(input_item);
    // Implement command trait
    derive::impl_command_baseline(tree)
}
