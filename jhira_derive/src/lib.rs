use structopt::{StructOpt, clap};
use syn::parse;
use quote::quote;
use syn::{ItemStruct};
use proc_macro::{TokenStream, Ident, TokenTree};
use proc_macro2::Span;

#[proc_macro_attribute]
pub fn cli(attrs: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse::<ItemStruct>(input).expect("PARSE");
    let target_ident = ast.ident.clone();

    let idents = attrs.into_iter()
        .filter_map(|t| {
            match t {
                TokenTree::Ident(ident) => Some(ident),
                _ => None
            }
        })
        .map(|ident| syn::Ident::new(&ident.to_string(), ident.span().into()));

    let enum_members = idents.clone()
        .map(|ident| quote!{ #ident(#ident) });

    let matches = idents.clone()
        .map(|ident| quote!{ Self::#ident(inner) => Box::new(inner) });

    let subcommands = quote! {
        #[derive(StructOpt, Debug)]
        #[structopt(setting = clap::AppSettings::InferSubcommands)]
        pub enum Subcommands {
            #(#enum_members),*
        }
        impl Subcommands {
            pub fn select(self) -> Box<dyn Exec> {
                match self {
                    #(#matches),*
                }
            }
        }
    };
    let output = quote! {
        #ast
        #[derive(StructOpt, Debug)]
        pub struct Main {
            #[structopt(long="dryrun")]
            pub dryrun: bool,
            #[structopt(long="config")]
            /// path to a wf2.yml config file
            pub config: Option<std::path::PathBuf>,
            #[structopt(long="cwd")]
            /// Sets the CWD for all docker commands
            pub cwd: Option<std::path::PathBuf>,
            #[structopt(subcommand)]
            pub cmd: Subcommands
        }
        #subcommands
        impl M2 {
            pub fn get_cli(input: Vec<impl Into<String>>) -> Result<Main, clap::Error> {
                Main::from_iter_safe(input.into_iter().map(|x| x.into()).collect::<Vec<String>>())
            }
        }
    };

    println!("{}", output.to_string());

    output.into()
}
