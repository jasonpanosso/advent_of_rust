use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use std::path::{Path, PathBuf};

#[proc_macro]
pub fn generate_days_enum(_input: TokenStream) -> TokenStream {
    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable is not set"),
    );

    let days_path = manifest_dir.join("src/days");

    let mut modules = vec![];
    if let Ok(entries) = fs::read_dir(days_path) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.path().file_stem() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.starts_with("day") {
                        modules.push(file_name_str.to_string());
                    }
                }
            }
        }
    }

    let enum_variants = modules.iter().map(|module| {
        let day_number = module[3..].parse::<usize>().unwrap();
        let variant_ident = syn::Ident::new(
            &format!("Day{:02}", day_number),
            proc_macro2::Span::call_site(),
        );
        quote! { #variant_ident }
    });

    let from_day_arms = modules.iter().map(|module| {
        let day_number = module[3..].parse::<usize>().unwrap();
        let variant_ident = syn::Ident::new(
            &format!("Day{:02}", day_number),
            proc_macro2::Span::call_site(),
        );
        quote! { #day_number => Some(Self::#variant_ident), }
    });

    let part_one_arms = modules.iter().map(|module| {
        let day_number = module[3..].parse::<usize>().unwrap();
        let variant_ident = syn::Ident::new(
            &format!("Day{:02}", day_number),
            proc_macro2::Span::call_site(),
        );
        let module_ident = syn::Ident::new(module, proc_macro2::Span::call_site());
        let binding = manifest_dir.join(format!("../data/inputs/day{:02}.txt", day_number));
        let input_path = binding.to_string_lossy();
        quote! {
            Self::#variant_ident => {
                let input = std::fs::read_to_string(#input_path)
                    .expect("Failed to load input file");
                #module_ident::DayStruct.part_one(&input)
            }
        }
    });

    let part_two_arms = modules.iter().map(|module| {
        let day_number = module[3..].parse::<usize>().unwrap();
        let variant_ident = syn::Ident::new(
            &format!("Day{:02}", day_number),
            proc_macro2::Span::call_site(),
        );
        let module_ident = syn::Ident::new(module, proc_macro2::Span::call_site());
        let binding = manifest_dir.join(format!("../data/inputs/day{:02}.txt", day_number));
        let input_path = binding.to_string_lossy();
        quote! {
            Self::#variant_ident => {
                let input = std::fs::read_to_string(#input_path)
                    .expect("Failed to load input file");
                #module_ident::DayStruct.part_two(&input)
            }
        }
    });

    let expanded = quote! {
        pub enum Days {
            #(#enum_variants),*
        }

        impl Days {
            pub fn from_day_number(day: usize) -> Option<Self> {
                match day {
                    #(#from_day_arms)*
                    _ => None,
                }
            }

            pub fn part_one(&self) -> impl std::fmt::Debug {
                match self {
                    #(#part_one_arms)*
                }
            }

            pub fn part_two(&self) -> impl std::fmt::Debug {
                match self {
                    #(#part_two_arms)*
                }
            }
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn generate_day_modules(_input: TokenStream) -> TokenStream {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let days_path = Path::new(&manifest_dir).join("src/days");

    let mut modules = vec![];
    if let Ok(entries) = fs::read_dir(&days_path) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.path().file_stem() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.starts_with("day") {
                        modules.push(file_name_str.to_string());
                    }
                }
            }
        }
    }

    let pub_uses = modules.iter().map(|module| {
        let module_ident = syn::Ident::new(module, proc_macro2::Span::call_site());
        quote! {
            pub mod #module_ident;
        }
    });

    let expanded = quote! {
        #(#pub_uses)*
    };

    expanded.into()
}
