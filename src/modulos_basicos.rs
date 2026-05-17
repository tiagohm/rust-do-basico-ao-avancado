// 3. Funções e organização inicial
// 3.2 - Módulos básicos
//
// Módulos organizam código em namespaces.
// Eles ajudam a separar responsabilidades, controlar visibilidade e evitar conflitos de nomes.

mod mensagens {
    // `mod mensagens { ... }` declara um módulo inline.
    // "Inline" significa que o conteúdo do módulo está dentro deste próprio arquivo.
    //
    // Por padrão, itens em Rust são privados ao módulo pai.
    // Esta função não tem `pub`, então só pode ser chamada dentro de `mensagens`
    // ou por módulos filhos de `mensagens`.
    fn prefixo() -> &'static str {
        "[mensagens]"
    }

    // `pub` torna o item visível para o módulo pai.
    // Aqui, `modulos_basicos` pode chamar `mensagens::boas_vindas()`.
    pub fn boas_vindas(nome: &str) -> String {
        format!("{} Olá, {nome}!", prefixo())
    }

    pub mod erros {
        // Um módulo filho pode acessar itens privados do módulo pai usando `super::`.
        // `super::` significa "o módulo acima do módulo atual".
        pub fn erro_de_validacao(campo: &str) -> String {
            format!("{} campo inválido: {campo}", super::prefixo())
        }
    }
}

mod matematica {
    pub const VALOR_PADRAO: i32 = 10;

    pub fn dobrar(valor: i32) -> i32 {
        valor * 2
    }

    pub fn triplicar(valor: i32) -> i32 {
        valor * 3
    }

    pub mod interna {
        pub fn metade(valor: i32) -> i32 {
            valor / 2
        }
    }
}

mod caminhos {
    pub fn demonstrar_caminhos() {
        // Caminho absoluto com `crate::`:
        // - começa na raiz da crate;
        // - funciona a partir de qualquer módulo;
        // - é útil quando queremos deixar claro de onde o item vem.
        let dobro = crate::modulos_basicos::matematica::dobrar(5);

        // Caminho relativo com `self::`:
        // - começa no módulo atual;
        // - dentro deste módulo, `self` é `caminhos`.
        let nome_do_modulo = self::nome();

        // Caminho com `super::`:
        // - sobe um nível;
        // - aqui, sai de `caminhos` e volta para `modulos_basicos`.
        let triplo = super::matematica::triplicar(5);

        println!("Caminhos com crate::, self:: e super:::");
        println!("crate::...::dobrar(5) = {dobro}");
        println!("self::nome() = {nome_do_modulo}");
        println!("super::matematica::triplicar(5) = {triplo}");
    }

    fn nome() -> &'static str {
        "caminhos"
    }
}

fn mod_basico() {
    // `mod` declara um módulo.
    //
    // Existem duas formas comuns:
    //
    // 1. Módulo inline:
    //    mod exemplo {
    //        pub fn executar() {}
    //    }
    //
    // 2. Módulo em outro arquivo:
    //    mod exemplo;
    //
    // Na segunda forma, Rust procura o conteúdo em `exemplo.rs`
    // ou em `exemplo/mod.rs`, dependendo da organização usada.
    println!("mod:");
    println!("`mod mensagens` e `mod matematica` foram declarados neste arquivo.");
}

fn pub_basico() {
    // `pub` controla visibilidade.
    //
    // Por padrão:
    // - funções são privadas;
    // - structs são privadas;
    // - campos de structs são privados;
    // - módulos são privados.
    //
    // `pub` deixa o item acessível ao módulo pai.
    // Mais tarde, existem formas mais específicas, como `pub(crate)` e `pub(super)`.
    println!("pub:");
    println!("{}", mensagens::boas_vindas("Tiago"));
    println!("{}", mensagens::erros::erro_de_validacao("email"));

    // A linha abaixo não compilaria, porque `prefixo` não é público.
    // println!("{}", mensagens::prefixo());
}

fn use_basico() {
    // `use` traz um caminho para o escopo atual.
    // Ele não copia nem importa código como em algumas linguagens;
    // ele cria um atalho de nome para um item existente.
    use matematica::dobrar;
    use matematica::interna::metade;

    println!("use:");
    println!("dobrar(8) = {}", dobrar(8));
    println!("metade(8) = {}", metade(8));

    // `as` cria um nome alternativo para evitar conflito ou melhorar leitura.
    use matematica::triplicar as multiplicar_por_tres;
    println!("multiplicar_por_tres(8) = {}", multiplicar_por_tres(8));

    // Também é possível agrupar imports.
    use matematica::{VALOR_PADRAO, triplicar};
    println!("VALOR_PADRAO={VALOR_PADRAO}, triplicar(VALOR_PADRAO)={}", triplicar(VALOR_PADRAO));
}

fn arquivos_main_e_lib() {
    // `main.rs` e `lib.rs` têm papéis diferentes:
    //
    // - `src/main.rs` cria uma crate binária, ou seja, um programa executável.
    //   Ele precisa ter uma função `fn main()`.
    //
    // - `src/lib.rs` cria uma crate de biblioteca.
    //   Bibliotecas expõem funções, tipos e módulos para serem usados por outros códigos.
    //
    // Um mesmo pacote Cargo pode ter os dois arquivos:
    // - `main.rs` para rodar o programa;
    // - `lib.rs` para concentrar lógica reutilizável.
    println!("Arquivos main.rs e lib.rs:");
    println!("Este projeto usa `src/main.rs` como ponto de entrada do roteiro.");
}

fn separacao_em_multiplos_arquivos() {
    // Separar módulos em múltiplos arquivos deixa o projeto mais organizado.
    //
    // Exemplo de estrutura:
    //
    // src/
    //   main.rs
    //   modulos_basicos.rs
    //   calculadora.rs
    //
    // Em `main.rs`, declararíamos:
    //
    // mod calculadora;
    //
    // E em `src/calculadora.rs`, escreveríamos:
    //
    // pub fn somar(a: i32, b: i32) -> i32 {
    //     a + b
    // }
    //
    // Quando há submódulos, também é comum usar uma pasta:
    //
    // src/
    //   geometria.rs
    //   geometria/
    //     circulo.rs
    //     retangulo.rs
    //
    // Nesse caso, `geometria.rs` pode declarar:
    //
    // pub mod circulo;
    // pub mod retangulo;
    println!("Separação em múltiplos arquivos:");
    println!("Use um arquivo por tópico principal quando o módulo crescer.");
}

pub fn run() {
    println!("##### 3.2 Módulos básicos #####");
    mod_basico();
    pub_basico();
    use_basico();
    caminhos::demonstrar_caminhos();
    arquivos_main_e_lib();
    separacao_em_multiplos_arquivos();
}
