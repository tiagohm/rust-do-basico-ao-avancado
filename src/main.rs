mod condicionais;
mod lacos;
mod pattern_matching;
mod operadores;
mod sintaxe_basica;
mod tipos_primitivos;

fn main() {
    println!("##### 1. Fundamentos da linguagem #####");
    sintaxe_basica::run();
    tipos_primitivos::run();
    operadores::run();

    println!("##### 2. Controle de fluxo #####");
    condicionais::run();
    lacos::run();
    pattern_matching::run();
}
