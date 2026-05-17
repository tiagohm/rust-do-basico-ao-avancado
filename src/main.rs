mod borrowing;
mod condicionais;
mod funcoes;
mod lacos;
mod modulos_basicos;
mod operadores;
mod ownership;
mod pattern_matching;
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

    println!("##### 3. Funções e organização inicial #####");
    funcoes::run();
    modulos_basicos::run();

    println!("##### 4. Ownership, Borrowing e Lifetimes #####");
    ownership::run();
    borrowing::run();
}
