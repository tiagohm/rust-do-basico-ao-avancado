// 4. Ownership, Borrowing e Lifetimes
// 4.2 - Borrowing
//
// Borrowing significa "emprestar" um valor em vez de transferir ownership.
// Em Rust, isso é feito com referências:
//
// - `&T`: referência imutável, também chamada de shared reference;
// - `&mut T`: referência mutável, também chamada de mutable reference.
//
// A ideia central:
// - quem empresta pode usar o valor sem se tornar dono dele;
// - o owner continua responsável por destruir o valor no fim do escopo;
// - o borrow checker garante que essas referências sejam válidas e seguras.

fn referencias_imutaveis() {
    // `&T` cria uma referência imutável para um valor do tipo `T`.
    // A referência permite ler o valor sem mover ownership.
    fn tamanho(texto: &String) -> usize {
        texto.len()
    }

    let linguagem = String::from("Rust");
    let referencia: &String = &linguagem;

    println!("Referências imutáveis: &T");
    println!("referencia={referencia}");
    println!("tamanho(&linguagem)={}", tamanho(&linguagem));

    // `linguagem` continua válida porque foi apenas emprestada.
    println!("owner ainda pode ser usado: {linguagem}");

    // Em APIs reais, normalmente preferimos `&str` em vez de `&String`,
    // porque `&str` aceita tanto `String` quanto string literal.
    fn tamanho_str(texto: &str) -> usize {
        texto.len()
    }

    println!("tamanho_str(&linguagem)={}", tamanho_str(&linguagem));
    println!("tamanho_str(\"literal\")={}", tamanho_str("literal"));
}

fn referencias_mutaveis() {
    // `&mut T` cria uma referência mutável.
    // Para emprestar mutavelmente, a variável original também precisa ser `mut`.
    fn adicionar_sufixo(texto: &mut String) {
        texto.push_str(" moderno");
    }

    let mut linguagem = String::from("Rust");

    println!("Referências mutáveis: &mut T");
    adicionar_sufixo(&mut linguagem);
    println!("linguagem={linguagem}");

    // A linha abaixo não compilaria, porque `linguagem` não seria mutável.
    //
    // let linguagem_imutavel = String::from("Rust");
    // adicionar_sufixo(&mut linguagem_imutavel);
}

fn regra_varias_imutaveis_ou_uma_mutavel() {
    // Regra essencial:
    //
    // - podemos ter várias referências imutáveis ao mesmo tempo;
    // - ou uma referência mutável;
    // - mas não podemos misturar referência mutável com referências ativas ao mesmo valor.
    //
    // Essa regra evita data races e leituras inconsistentes.
    let mut texto = String::from("Rust");

    println!("Regra: várias imutáveis ou uma mutável");

    let r1 = &texto;
    let r2 = &texto;
    println!("r1={r1}, r2={r2}");

    // Depois do último uso de `r1` e `r2`, o borrow imutável termina.
    // A partir daqui, podemos criar uma referência mutável.
    let r3 = &mut texto;
    r3.push_str("!");
    println!("r3={r3}");

    // Isto não compilaria, porque tentaria usar `texto` imutavelmente
    // enquanto `r3` ainda está emprestando mutavelmente:
    //
    // println!("{texto}");
    // println!("{r3}");
}

fn borrow_checker() {
    // Borrow checker é a parte do compilador que valida empréstimos.
    // Ele verifica, em tempo de compilação, se referências:
    // - não vivem mais que o valor referenciado;
    // - não violam a regra de várias imutáveis ou uma mutável;
    // - não acessam valor depois de move;
    // - não criam dangling references.
    println!("Borrow checker:");

    let mut numeros = vec![1, 2, 3];
    let primeiro = &numeros[0];
    println!("primeiro={primeiro}");

    // Depois que `primeiro` foi usado pela última vez, o borrow acaba.
    numeros.push(4);
    println!("numeros={numeros:?}");

    // A versão abaixo não compilaria, porque `push` poderia realocar o Vec
    // enquanto `primeiro` ainda estaria apontando para um elemento antigo.
    //
    // let primeiro = &numeros[0];
    // numeros.push(4);
    // println!("{primeiro}");
}

fn reborrowing() {
    // Reborrowing é criar um empréstimo temporário a partir de outra referência.
    // Isso é comum quando temos `&mut T`, mas queremos passar o valor para uma função
    // que só precisa de `&T`.
    fn imprimir(texto: &String) {
        println!("imprimir recebeu: {texto}");
    }

    fn alterar(texto: &mut String) {
        texto.push_str(" alterado");
    }

    println!("Reborrowing:");

    let mut texto = String::from("valor");
    let referencia_mutavel = &mut texto;

    // Aqui, Rust cria uma reborrow imutável temporária a partir de `&mut String`.
    imprimir(referencia_mutavel);

    // Quando a reborrow termina, podemos usar a referência mutável novamente.
    alterar(referencia_mutavel);
    println!("referencia_mutavel={referencia_mutavel}");

    // Também é possível escrever a reborrow explicitamente:
    let outra_referencia_mutavel = &mut texto;
    let somente_leitura: &String = &*outra_referencia_mutavel;
    println!("somente_leitura={somente_leitura}");
    outra_referencia_mutavel.push_str(" novamente");
    println!("texto={texto}");
}

fn borrow_splitting() {
    // Borrow splitting é quando Rust entende que estamos pegando partes diferentes
    // de um mesmo valor, permitindo empréstimos separados.
    #[derive(Debug)]
    struct Usuario {
        nome: String,
        idade: u8,
    }

    println!("Borrow splitting:");

    let mut usuario = Usuario { nome: String::from("Ana"), idade: 30 };

    // Campos diferentes podem ser emprestados mutavelmente ao mesmo tempo.
    let nome = &mut usuario.nome;
    let idade = &mut usuario.idade;

    nome.push_str(" Silva");
    *idade += 1;

    println!("usuario={usuario:?}");

    // Com slices, use APIs como `split_at_mut` para provar ao compilador
    // que as partes não se sobrepõem.
    let mut numeros = [1, 2, 3, 4];
    let (esquerda, direita) = numeros.split_at_mut(2);
    esquerda[0] = 10;
    direita[0] = 30;
    println!("numeros={numeros:?}");

    // A forma abaixo não compilaria, porque Rust não consegue garantir,
    // pela indexação direta, que os dois índices são diferentes em todos os casos.
    //
    // let a = &mut numeros[0];
    // let b = &mut numeros[1];
    // *a += 1;
    // *b += 1;
}

fn borrow_temporario() {
    // Borrow temporário acontece quando criamos uma referência apenas para uma expressão.
    // Ela dura só o necessário para aquela chamada ou expressão.
    fn mostrar(texto: &str) {
        println!("mostrar: {texto}");
    }

    println!("Borrow temporário:");

    mostrar(&String::from("String temporária"));

    // A `String` temporária existe durante a chamada de `mostrar`
    // e é destruída logo depois.
    let tamanho = String::from("abc").len();
    println!("tamanho de String temporária={tamanho}");

    // Este padrão é válido porque a referência não escapa da expressão.
    let grito = String::from("rust").to_uppercase();
    println!("grito={grito}");
}

fn non_lexical_lifetimes() {
    // NLL significa Non-Lexical Lifetimes.
    // Antes do NLL, alguns borrows duravam até o fim do bloco textual.
    // Hoje, Rust costuma encerrar o borrow no último uso real da referência.
    println!("Non-lexical lifetimes, NLL:");

    let mut texto = String::from("Rust");
    let leitura = &texto;
    println!("leitura={leitura}");

    // O borrow imutável de `leitura` terminou no último uso acima.
    // Por isso, agora podemos mutar `texto`.
    texto.push_str(" 2024");
    println!("texto={texto}");

    // Se usássemos `leitura` depois da mutação, não compilaria:
    //
    // let leitura = &texto;
    // texto.push_str("!");
    // println!("{leitura}");
}

fn dangling_references() {
    // Dangling reference é uma referência para algo que já foi destruído.
    // Rust impede isso em tempo de compilação.
    println!("Dangling references:");

    fn criar_string() -> String {
        String::from("valor válido")
    }

    let texto = criar_string();
    let referencia = &texto;
    println!("referencia={referencia}");

    // A função abaixo não compilaria:
    //
    // fn referencia_invalida() -> &String {
    //     let texto = String::from("morre no fim da função");
    //     &texto
    // }
    //
    // Motivo:
    // - `texto` seria destruído ao final da função;
    // - a referência retornada apontaria para memória inválida.
    //
    // Solução comum: retorne o valor owned.
}

fn emprestimo_de_campos_de_structs() {
    #[derive(Debug)]
    struct Perfil {
        nome: String,
        email: String,
        ativo: bool,
    }

    println!("Empréstimo de campos de structs:");

    let mut perfil = Perfil { nome: String::from("Tiago"), email: String::from("tiago@example.com"), ativo: true };

    let nome = &perfil.nome;
    let email = &perfil.email;
    println!("nome={nome}, email={email}");

    // Depois do último uso de `nome` e `email`, podemos mutar outro campo.
    let ativo = &mut perfil.ativo;
    *ativo = false;
    println!("perfil={perfil:?}");

    // Mover um campo não-`Copy` de uma struct impediria usar a struct inteira depois.
    // Por enquanto, prefira emprestar campos com `&perfil.campo`.
}

fn emprestimo_em_loops() {
    println!("Empréstimo em loops:");

    let nomes = vec![String::from("Ana"), String::from("Bia"), String::from("Caio")];

    // `for nome in &nomes` empresta cada item.
    // O vetor continua válido depois do loop.
    for nome in &nomes {
        println!("nome emprestado={nome}");
    }
    println!("nomes ainda existe: {nomes:?}");

    let mut numeros = vec![1, 2, 3];
    for numero in &mut numeros {
        *numero *= 10;
    }
    println!("numeros alterados={numeros:?}");

    // `for nome in nomes` moveria cada item para dentro do loop.
    // Depois disso, o vetor não poderia mais ser usado.
    //
    // for nome in nomes {
    //     println!("{nome}");
    // }
    // println!("{nomes:?}");
}

fn emprestimo_em_closures() {
    // Closures podem capturar valores por:
    // - referência imutável;
    // - referência mutável;
    // - move, quando usam `move` ou precisam tomar ownership.
    println!("Empréstimo em closures:");

    let linguagem = String::from("Rust");
    let mostrar = || {
        // Captura `linguagem` por referência imutável.
        println!("linguagem={linguagem}");
    };

    mostrar();
    println!("linguagem ainda existe={linguagem}");

    let mut contador = 0;
    let mut incrementar = || {
        // Captura `contador` por referência mutável.
        contador += 1;
        println!("contador dentro da closure={contador}");
    };

    incrementar();
    incrementar();

    // Depois do último uso da closure, o borrow mutável termina.
    println!("contador depois da closure={contador}");

    let texto = String::from("movido para closure");
    let consumir = move || {
        // `move` força a closure a capturar ownership de `texto`.
        println!("{texto}");
    };

    consumir();

    // A linha abaixo não compilaria, porque `texto` foi movido para a closure.
    // println!("{texto}");
}

pub fn run() {
    println!("##### 4.2 Borrowing #####");
    referencias_imutaveis();
    referencias_mutaveis();
    regra_varias_imutaveis_ou_uma_mutavel();
    borrow_checker();
    reborrowing();
    borrow_splitting();
    borrow_temporario();
    non_lexical_lifetimes();
    dangling_references();
    emprestimo_de_campos_de_structs();
    emprestimo_em_loops();
    emprestimo_em_closures();
}
