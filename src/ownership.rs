// 4. Ownership, Borrowing e Lifetimes
// 4.1 - Ownership
//
// Ownership é uma das partes mais importantes de Rust.
// Ela permite que Rust gerencie memória sem garbage collector e sem exigir
// `free()` manual como em C.
//
// Regras centrais:
// 1. Cada valor em Rust tem um owner, ou seja, um dono.
// 2. Só pode existir um owner por vez.
// 3. Quando o owner sai de escopo, o valor é destruído.
//
// Neste arquivo, vamos focar em ownership. Borrowing e lifetimes aparecem depois.

fn regras_basicas_de_ownership() {
    // Um valor pertence à variável que o guarda.
    // Aqui, `curso` é o owner da `String`.
    let curso = String::from("Rust");

    println!("Regras básicas de ownership:");
    println!("curso={curso}");

    // Quando `curso` sair do escopo desta função, a `String` será destruída automaticamente.
    // Não chamamos `free`, `delete` nem nada parecido.
}

fn move_semantics() {
    // Move semantics significa: transferir ownership de uma variável para outra.
    //
    // `String` é um tipo que aloca memória no heap.
    // Por isso, copiar seus bytes automaticamente poderia ser caro e ambíguo.
    // Então Rust move o ownership por padrão.
    let original = String::from("valor no heap");
    let novo_dono = original;

    println!("Move semantics:");
    println!("novo_dono={novo_dono}");

    // A linha abaixo não compilaria.
    // Depois do move, `original` não é mais válido.
    //
    // println!("original={original}");

    // Pense assim:
    // - antes: `original` era dono da String;
    // - depois: `novo_dono` passou a ser dono;
    // - Rust impede usar o dono antigo para evitar acesso a valor movido.
}

fn copy_semantics() {
    // Copy semantics significa: copiar o valor automaticamente em vez de mover.
    //
    // Tipos simples como inteiros, bool e char ficam inteiros no stack
    // e são baratos de copiar.
    let a = 10;
    let b = a;

    println!("Copy semantics:");
    println!("a={a}, b={b}");

    // `a` continua válido porque `i32` implementa `Copy`.
    let ativo = true;
    let outro_ativo = ativo;
    println!("ativo={ativo}, outro_ativo={outro_ativo}");

    let letra = 'R';
    let outra_letra = letra;
    println!("letra={letra}, outra_letra={outra_letra}");
}

fn trait_copy() {
    // `Copy` é um trait marcador.
    // Ele indica que um valor pode ser duplicado com uma cópia simples,
    // sem invalidar a variável original.
    //
    // Tipos que normalmente implementam `Copy`:
    // - inteiros;
    // - floats;
    // - bool;
    // - char;
    // - tuplas/arrays compostos apenas por tipos `Copy`;
    // - structs que derivam `Copy` e têm apenas campos `Copy`.
    #[derive(Copy, Clone, Debug)]
    struct Ponto {
        x: i32,
        y: i32,
    }

    let p1 = Ponto { x: 3, y: 4 };
    let p2 = p1;

    println!("O trait Copy:");
    println!("p1={p1:?}, p2={p2:?}");
    println!("p1.x + p1.y = {}", p1.x + p1.y);

    // `p1` ainda é válido porque `Ponto` implementa `Copy`.
    //
    // Um tipo que implementa `Drop` não pode implementar `Copy`,
    // porque seria confuso destruir automaticamente várias cópias do mesmo recurso.
}

fn trait_clone() {
    // `Clone` representa cópia explícita.
    // Diferente de `Copy`, ela pode executar lógica e pode ser mais cara.
    //
    // `String` implementa `Clone`, mas não implementa `Copy`.
    // Ao chamar `.clone()`, pedimos explicitamente uma nova alocação com o mesmo texto.
    let original = String::from("Rust");
    let copia = original.clone();

    println!("O trait Clone:");
    println!("original={original}");
    println!("copia={copia}");

    // Sem `.clone()`, aconteceria move:
    //
    // let copia = original;
    // println!("{original}"); // não compila depois do move
    //
    // Use `clone` quando você realmente precisa de dois owners independentes.
    // Não use `clone` só para "acalmar" o compilador sem entender o ownership.
}

fn quando_um_valor_e_movido() {
    // Um valor é movido quando:
    // - atribuímos um valor não-`Copy` a outra variável;
    // - passamos um valor não-`Copy` para uma função por valor;
    // - retornamos um valor e transferimos ownership para quem chamou;
    // - colocamos um valor em uma coleção ou struct que passa a ser dona dele.
    println!("Quando um valor é movido:");

    let texto = String::from("movido para outra variável");
    let texto_movido = texto;
    println!("texto_movido={texto_movido}");

    // `texto` foi movido e não pode mais ser usado.
    // println!("{texto}");

    let item = String::from("movido para um Vec");
    let itens = vec![item];
    println!("itens={itens:?}");

    // `item` foi movido para dentro do vetor.
    // println!("{item}");
}

fn quando_um_valor_e_copiado() {
    // Um valor é copiado quando seu tipo implementa `Copy`.
    // A cópia acontece implicitamente em atribuições e passagens por valor.
    println!("Quando um valor é copiado:");

    let numero = 42;
    let outro_numero = numero;
    println!("numero={numero}, outro_numero={outro_numero}");

    let par = (10, true);
    let outro_par = par;
    println!("par={par:?}, outro_par={outro_par:?}");

    // Esta tupla não seria `Copy`, porque `String` não é `Copy`.
    let tupla_com_string = (String::from("Rust"), 2024);
    let nova_tupla = tupla_com_string;
    println!("nova_tupla={nova_tupla:?}");

    // A linha abaixo não compilaria, porque a tupla inteira foi movida.
    // println!("{tupla_com_string:?}");
}

fn ownership_em_funcoes() {
    fn recebe_string(texto: String) {
        println!("função recebeu String: {texto}");
        // `texto` será destruído ao final desta função.
    }

    fn recebe_i32(numero: i32) {
        println!("função recebeu i32: {numero}");
    }

    println!("Ownership em funções:");

    let linguagem = String::from("Rust");
    recebe_string(linguagem);

    // `linguagem` foi movida para a função.
    // A linha abaixo não compilaria:
    // println!("{linguagem}");

    let ano = 2024;
    recebe_i32(ano);

    // `i32` é `Copy`, então `ano` continua válido.
    println!("ano ainda pode ser usado: {ano}");
}

fn ownership_em_retornos() {
    fn criar_nome() -> String {
        String::from("Rust")
    }

    fn adicionar_sufixo(mut texto: String) -> String {
        texto.push_str(" moderno");
        texto
    }

    // Uma função pode receber ownership e devolver ownership.
    // Isso é comum quando queremos transformar um valor e continuar usando o resultado.
    println!("Ownership em retornos:");

    let nome = criar_nome();
    println!("nome criado={nome}");

    let nome = adicionar_sufixo(nome);
    println!("nome transformado={nome}");

    // O `nome` antigo foi movido para `adicionar_sufixo`.
    // O `nome` novo é o valor retornado pela função.
}

fn escopo_e_destruicao() {
    // Escopo é a região do código onde uma variável é válida.
    // Quando o escopo acaba, Rust destrói os valores que ainda têm owner ali.
    println!("Escopo e destruição:");

    {
        let interno = String::from("valor interno");
        println!("dentro do bloco: {interno}");
    }

    // `interno` saiu de escopo e foi destruído.
    // A linha abaixo não compilaria:
    // println!("{interno}");

    let externo = String::from("valor externo");
    println!("fora do bloco: {externo}");
}

fn drop_trait() {
    // `Drop` é o trait chamado automaticamente quando um valor sai de escopo.
    // Ele é usado para liberar recursos: memória, arquivos, conexões, locks etc.
    //
    // Normalmente você não chama o método `drop` diretamente.
    // Rust chama automaticamente no fim do escopo.
    struct Recurso {
        nome: &'static str,
    }

    impl Drop for Recurso {
        fn drop(&mut self) {
            println!("Drop chamado para '{}'", self.nome);
        }
    }

    println!("Drop:");

    {
        let _arquivo = Recurso { nome: "arquivo temporário" };
        println!("dentro do escopo do recurso");
    }

    println!("depois do escopo do recurso");

    let conexao = Recurso { nome: "conexão encerrada manualmente" };
    println!("antes de drop(conexao)");

    // `std::mem::drop` recebe ownership e força a destruição naquele ponto.
    // Depois disso, `conexao` não pode mais ser usada.
    drop(conexao);
    println!("depois de drop(conexao)");

    // A linha abaixo não compilaria, porque `conexao` foi movida para `drop`.
    // println!("{}", conexao.nome);
}

pub fn run() {
    println!("##### 4.1 Ownership #####");
    regras_basicas_de_ownership();
    move_semantics();
    copy_semantics();
    trait_copy();
    trait_clone();
    quando_um_valor_e_movido();
    quando_um_valor_e_copiado();
    ownership_em_funcoes();
    ownership_em_retornos();
    escopo_e_destruicao();
    drop_trait();
}
