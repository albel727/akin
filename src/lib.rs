use quote::quote;
use proc_macro::{Delimiter, TokenTree};
extern crate proc_macro;

#[proc_macro]
pub fn akin(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut vars: Vec<(String, Vec<String>)> = Vec::new();
    let mut tokens = input.clone().into_iter();
    
    while matches!(tokens.next().unwrap(), TokenTree::Ident(ident) if ident.to_string() == "let") && matches!(tokens.next().unwrap(), TokenTree::Punct(punct) if punct.to_string() == "&") {
        vars.push(parse_var(&mut tokens, &vars));
    }
    
    
    
    
    
    
    

    //let tokens = format!("proc_macro: {:#?}", input.into_iter().collect::<Vec<_>>());
    let tokens = format!("vars: {:#?}", vars);
    
    quote! {
        println!("{}", #tokens);
    }.into()
}

fn parse_var(tokens: &mut proc_macro::token_stream::IntoIter, vars: &[(String, Vec<String>)]) -> (String, Vec<String>) {
    let name = format!("*{}", tokens.next().unwrap());
    tokens.next().unwrap(); // skip '='
    let mut values: Vec<String> = Vec::new();
    if let TokenTree::Group(group) = tokens.next().unwrap() {
        if group.delimiter() == Delimiter::Bracket {
            for var in group.stream() {
                let txt = var.to_string();
                if txt != "," {
                    values.push(txt);
                }
            }
        } else {
            values.push(duplicate(&group.stream().to_string(), vars));
        }
        
    }
    tokens.next().unwrap(); // skip ';'

    (name, values)
}

fn duplicate(stream: &str, vars: &[(String, Vec<String>)]) -> String {
    let (vars, times) = get_used_vars(stream, vars);
    let mut out = String::new();
    out = format!("{vars:?}, {times}, {stream}");
    for i in 0..times {
        let mut temp = stream.to_owned();
        for var in &vars {
            temp = temp.replace(&var.0, var.1.get(i).unwrap_or_else(|| var.1.last().unwrap()))
        }
        out += &temp;
    }
    out
}

fn get_used_vars(stream: &str, vars: &[(String, Vec<String>)]) -> (Vec<(String, Vec<String>)>, usize) {
    let mut used = Vec::new();
    let mut times = 0;

    for var in vars {
        if stream.contains(&var.0) {
            used.push(var.clone());
            times = times.max(var.1.len());
        }
    }

    (used, times)
}