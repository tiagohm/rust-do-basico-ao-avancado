// 2. Controle de fluxo
// 2.2 - Laços
//
// Laços permitem repetir um bloco de código enquanto uma condição for verdadeira,
// enquanto houver valores para processar ou até que uma interrupção explícita aconteça.

fn loop_basico() {
    // `loop` cria um laço infinito.
    // Ele continua executando até encontrar um `break`, um `return` ou uma falha como `panic!`.
    let mut contador = 0;

    println!("loop:");
    loop {
        contador += 1;
        println!("contador={contador}");

        if contador == 3 {
            break;
        }
    }
}

fn while_basico() {
    // `while` repete enquanto a condição for `true`.
    // A condição é testada antes de cada repetição.
    let mut numero = 3;

    println!("while:");
    while numero > 0 {
        println!("{numero}");
        numero -= 1;
    }

    println!("fim do while");
}

fn while_let() {
    // `while let` repete enquanto um valor combina com um padrão.
    // É comum com `Option`, `Result` e estruturas que removem valores aos poucos.
    let mut pilha = vec!["terceiro", "segundo", "primeiro"];

    println!("while let:");
    while let Some(item) = pilha.pop() {
        println!("removido da pilha: {item}");
    }

    println!("pilha vazia");
}

fn for_basico() {
    // `for` percorre valores de algo que implementa `IntoIterator`.
    // Ele é o laço mais usado para percorrer coleções e intervalos.
    let nomes = ["Ana", "Bia", "Caio"];

    println!("for:");
    for nome in nomes {
        println!("nome={nome}");
    }
}

fn break_e_continue() {
    // `break` encerra o laço atual.
    // `continue` pula o restante da repetição atual e vai para a próxima.
    println!("break e continue:");

    for numero in 1..=10 {
        if numero % 2 == 0 {
            continue; // pula números pares
        }

        if numero > 7 {
            break; // encerra o laço
        }

        println!("ímpar menor ou igual a 7: {numero}");
    }
}

fn labels_em_loops() {
    // Labels nomeiam laços.
    //
    // Sintaxe:
    // 'nome_do_label: for item in colecao { ... }
    //
    // O label:
    // - começa com apóstrofo `'`;
    // - usa um nome em snake_case;
    // - termina com `:`;
    // - fica imediatamente antes do laço que ele nomeia.
    //
    // Apesar de parecer com lifetime, label de laço e lifetime são conceitos diferentes.
    // O label é apenas um alvo para `break` e `continue`.
    //
    // Onde é possível usar labels em laços:
    // - antes de `loop`;
    // - antes de `while`;
    // - antes de `while let`;
    // - antes de `for`.
    //
    // Como usar:
    // - `break 'label;` encerra o laço nomeado;
    // - `continue 'label;` pula para a próxima repetição do laço nomeado.
    //
    // Onde não faz sentido ou não é permitido:
    // - não dá para usar `continue 'label` apontando para algo que não seja laço;
    // - não dá para usar label como `goto` para entrar em um laço de fora para dentro;
    // - não dá para colocar label em `if`, `match`, chamada de função ou expressão comum
    //   esperando que `break`/`continue` pulem para lá;
    // - labels só são visíveis dentro do bloco que eles nomeiam.
    //
    // Em Rust também existem blocos rotulados, mas eles são usados com `break`, não com `continue`.
    println!("labels em loops:");

    'linhas: for linha in 1..=3 {
        for coluna in 1..=3 {
            if linha == 2 && coluna == 2 {
                println!("parando no ponto linha={linha}, coluna={coluna}");
                break 'linhas;
            }

            println!("linha={linha}, coluna={coluna}");
        }
    }

    println!("continue com label:");
    'tabuada: for multiplicador in 1..=3 {
        for numero in 1..=3 {
            if multiplicador == 2 && numero == 2 {
                println!("pulando o restante da tabuada do {multiplicador}");
                continue 'tabuada;
            }

            println!("{multiplicador} x {numero} = {}", multiplicador * numero);
        }
    }
}

fn retornar_valor_de_loop() {
    // `loop` pode retornar um valor com `break valor`.
    // Isso é útil quando repetimos até encontrar um resultado.
    let mut tentativa = 0;

    println!("retornar valor de loop:");
    let resultado = loop {
        tentativa += 1;

        if tentativa == 4 {
            break tentativa * 10;
        }
    };

    println!("resultado={resultado}");
}

fn iteracao_sobre_ranges() {
    // Ranges são muito usados com `for`.
    // `0..5` vai de 0 até antes de 5.
    // `0..=5` vai de 0 até 5, incluindo o final.
    println!("iteração sobre ranges:");

    print!("0..5:");
    for numero in 0..5 {
        print!(" {numero}");
    }
    println!();

    print!("0..=5:");
    for numero in 0..=5 {
        print!(" {numero}");
    }
    println!();

    print!("range reverso:");
    for numero in (1..=3).rev() {
        print!(" {numero}");
    }
    println!();
}

fn iteracao_sobre_arrays_slices_vetores_e_iteradores() {
    println!("iteração sobre arrays, slices, vetores e iteradores:");

    // Array: tamanho fixo conhecido em tempo de compilação.
    let array = [10, 20, 30];
    for valor in array {
        println!("array por valor: {valor}");
    }

    // Slice: visão emprestada de parte de uma sequência.
    let numeros = [1, 2, 3, 4, 5];
    let slice = &numeros[1..4];
    for valor in slice {
        println!("slice por referência: {valor}");
    }

    // Vetor: coleção dinâmica alocada no heap.
    let vetor = vec!["Rust", "Cargo", "Clippy"];
    for item in &vetor {
        println!("vetor emprestado: {item}");
    }

    // `iter()` cria um iterador por referência.
    let soma: i32 = numeros.iter().sum();
    println!("soma com iter()={soma}");

    // Adaptadores de iterador, como `map` e `filter`, são preguiçosos:
    // só executam quando consumidos por `collect`, `sum`, `for` etc.
    let dobrados_pares: Vec<i32> = numeros.iter().filter(|valor| **valor % 2 == 0).map(|valor| valor * 2).collect();
    println!("pares dobrados com iteradores: {dobrados_pares:?}");

    // `enumerate()` combina índice e valor.
    for (indice, valor) in vetor.iter().enumerate() {
        println!("vetor[{indice}]={valor}");
    }

    // `into_iter()` consome a coleção quando usado sobre um `Vec<T>` possuído.
    let linguagens = vec!["Rust", "TypeScript"];
    for linguagem in linguagens.into_iter() {
        println!("consumido por into_iter(): {linguagem}");
    }

    // A linha abaixo não compilaria, porque `linguagens` foi movido pelo `into_iter()`.
    // println!("{linguagens:?}");
}

pub fn run() {
    println!("##### 2.2 Laços #####");
    loop_basico();
    while_basico();
    while_let();
    for_basico();
    break_e_continue();
    labels_em_loops();
    retornar_valor_de_loop();
    iteracao_sobre_ranges();
    iteracao_sobre_arrays_slices_vetores_e_iteradores();
}
