#![allow(non_snake_case)]

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = String::from(&args[1]).to_lowercase();

    let mut palabras: Vec<&str> = Vec::new();
    let mut cuenta_palabras: Vec<u32> = Vec::new();

    let _palabras = input.split(" ");

    for (_, palabra) in _palabras.enumerate() {
        if !palabras.contains(&palabra) {
            palabras.push(palabra);
            cuenta_palabras.push(1);
        } else {
            if let Some((i, _)) = palabras
                .iter()
                .enumerate()
                .find(|&(_, palabra_vec)| palabra_vec as &str == palabra)
            {
                if let Some(valor) = cuenta_palabras.get_mut(i) {
                    *valor += 1
                }
            }
        }
    }

    for (i, palabra) in palabras.iter().enumerate() {
        print!("{}{}", palabra, cuenta_palabras[i])
    }
}
