// 3. Funções e organização inicial
// 3.1 - Funções
//
// Funções agrupam código reutilizável.
// Em Rust, elas são declaradas com `fn`, podem receber parâmetros,
// podem retornar valores e também podem existir dentro de blocos `impl`.

fn declaracao_de_funcao() {
    // Sintaxe geral:
    //
    // fn nome_da_funcao(parametro: Tipo) -> TipoDeRetorno {
    //     corpo_da_funcao
    // }
    //
    // Partes importantes:
    // - `fn` inicia a declaração;
    // - o nome segue snake_case;
    // - parâmetros ficam entre parênteses;
    // - `-> Tipo` indica retorno;
    // - o corpo fica entre `{ }`.
    println!("Declaração de função:");
    println!("Esta função foi declarada com `fn declaracao_de_funcao()`.");
}

fn parametros() {
    // Parâmetros sempre precisam de tipo explícito.
    // Diferente de `let`, Rust não infere tipos na lista de parâmetros.
    fn soma(a: i32, b: i32) -> i32 {
        a + b
    }

    fn repetir(texto: &str, vezes: usize) {
        for _ in 0..vezes {
            println!("{texto}");
        }
    }

    println!("Parâmetros:");
    println!("soma(2, 3) = {}", soma(2, 3));
    repetir("Rust", 2);

    // A linha abaixo não compilaria, porque o parâmetro precisa ter tipo.
    // fn sem_tipo(valor) {
    //     println!("{valor}");
    // }
}

fn retorno() {
    // O tipo de retorno aparece depois de `->`.
    // Se a função não tiver `->`, ela retorna `()`.
    fn dobro(numero: i32) -> i32 {
        numero * 2
    }

    fn apenas_imprime(mensagem: &str) {
        println!("{mensagem}");
    }

    println!("Retorno:");
    println!("dobro(21) = {}", dobro(21));

    let retorno_unitario = apenas_imprime("Função sem `->` retorna ().");
    println!("retorno_unitario={retorno_unitario:?}");
}

fn retorno_implicito() {
    // Retorno implícito é a última expressão do bloco sem ponto e vírgula.
    // Essa é uma sintaxe comum em Rust e costuma causar confusão no início.
    fn area_do_retangulo(largura: u32, altura: u32) -> u32 {
        largura * altura
    }

    // Se colocarmos `;` na última linha, a expressão vira statement e a função
    // passaria a retornar `()`, causando erro porque prometemos retornar `u32`.
    //
    // fn errado(largura: u32, altura: u32) -> u32 {
    //     largura * altura;
    // }

    println!("Retorno implícito:");
    println!("area_do_retangulo(4, 5) = {}", area_do_retangulo(4, 5));
}

fn retorno_explicito_com_return() {
    // `return` encerra a função imediatamente e devolve um valor.
    // Em Rust, ele é mais usado para saídas antecipadas.
    fn classificar_idade(idade: u8) -> &'static str {
        if idade < 18 {
            return "menor de idade";
        }

        "maior de idade"
    }

    println!("Retorno explícito com return:");
    println!("17: {}", classificar_idade(17));
    println!("18: {}", classificar_idade(18));

    // Em funções simples, prefira retorno implícito no final.
    // Use `return` quando ele deixar uma saída antecipada mais clara.
}

struct Contador {
    valor: i32,
}

impl Contador {
    // Função associada:
    // - fica dentro de um bloco `impl`;
    // - não recebe `self`;
    // - é chamada com `Tipo::funcao()`.
    //
    // A sintaxe `::` acessa itens associados a um tipo ou módulo.
    fn novo(valor_inicial: i32) -> Contador {
        Contador { valor: valor_inicial }
    }

    // Método:
    // - também fica em `impl`;
    // - recebe `self`, `&self` ou `&mut self` como primeiro parâmetro;
    // - é chamado com a sintaxe de ponto: `valor.metodo()`.
    fn valor(&self) -> i32 {
        self.valor
    }

    // `&mut self` permite alterar a instância.
    fn incrementar(&mut self) {
        self.valor += 1;
    }

    // `self` por valor consome a instância.
    // Depois de chamar este método, o valor original não pode mais ser usado.
    fn consumir(self) -> i32 {
        self.valor
    }
}

fn funcoes_associadas() {
    println!("Funções associadas:");

    let contador = Contador::novo(10);
    println!("Contador::novo(10).valor() = {}", contador.valor());

    // `String::from` é outro exemplo comum de função associada.
    let linguagem = String::from("Rust");
    println!("String criada com String::from: {linguagem}");
}

fn metodos() {
    println!("Métodos:");

    let mut contador = Contador::novo(0);
    contador.incrementar();
    contador.incrementar();

    println!("contador.valor() = {}", contador.valor());

    let valor_final = contador.consumir();
    println!("contador.consumir() = {valor_final}");

    // A linha abaixo não compilaria, porque `consumir(self)` moveu `contador`.
    // println!("{}", contador.valor());
}

fn overloading_inexistente_em_rust() {
    // Rust não permite declarar duas funções com o mesmo nome no mesmo escopo,
    // mesmo que os tipos dos parâmetros sejam diferentes.
    //
    // Isso é diferente de linguagens com overloading por assinatura.
    //
    // As duas funções abaixo não poderiam coexistir:
    //
    // fn converter(valor: i32) -> String {
    //     valor.to_string()
    // }
    //
    // fn converter(valor: bool) -> String {
    //     valor.to_string()
    // }
    //
    // Alternativas comuns em Rust:
    // - usar nomes diferentes;
    // - usar generics;
    // - usar traits como `From`, `Into`, `AsRef` ou traits próprias.
    fn converter_i32(valor: i32) -> String {
        valor.to_string()
    }

    fn converter_bool(valor: bool) -> String {
        valor.to_string()
    }

    println!("Overloading inexistente em Rust:");
    println!("converter_i32(42) = {}", converter_i32(42));
    println!("converter_bool(true) = {}", converter_bool(true));
}

fn funcoes_genericas() {
    // Funções genéricas recebem parâmetros de tipo.
    // Sintaxe geral:
    //
    // fn nome<T>(valor: T) -> T { ... }
    //
    // `T` é um parâmetro de tipo. O nome `T` é convenção, não obrigação.
    // Generics evitam duplicar a mesma função para vários tipos.
    fn identidade<T>(valor: T) -> T {
        valor
    }

    // Para usar certas operações, precisamos limitar o tipo com trait bounds.
    // Aqui, `T: std::fmt::Debug` significa: T precisa implementar Debug.
    fn imprimir_debug<T: std::fmt::Debug>(valor: T) {
        println!("{valor:?}");
    }

    // Também podemos ter mais de um parâmetro de tipo.
    fn par<A, B>(primeiro: A, segundo: B) -> (A, B) {
        (primeiro, segundo)
    }

    println!("Funções genéricas:");
    println!("identidade(10) = {}", identidade(10));
    println!("identidade(\"Rust\") = {}", identidade("Rust"));

    print!("imprimir_debug([1, 2, 3]) = ");
    imprimir_debug([1, 2, 3]);

    let resultado = par("ano", 2024);
    println!("par(\"ano\", 2024) = {resultado:?}");

    // Quando Rust não consegue inferir o tipo genérico, podemos usar turbofish:
    // `::<Tipo>`.
    //
    // O nome "turbofish" é apelido da sintaxe `::<>`.
    let numero = identidade::<u32>(5);
    println!("identidade::<u32>(5) = {numero}");
}

pub fn run() {
    println!("##### 3.1 Funções #####");
    declaracao_de_funcao();
    parametros();
    retorno();
    retorno_implicito();
    retorno_explicito_com_return();
    funcoes_associadas();
    metodos();
    overloading_inexistente_em_rust();
    funcoes_genericas();
}
