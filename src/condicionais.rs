// 2. Controle de fluxo
// 2.1 - Condicionais
//
// Condicionais permitem executar partes diferentes do programa de acordo
// com valores booleanos, padrões ou resultados de operações.

fn if_basico() {
    // `if` executa um bloco quando a condição é `true`.
    // Em Rust, a condição precisa ser um `bool`; números não são tratados
    // automaticamente como verdadeiro ou falso.
    let idade = 18;

    println!("if:");
    if idade >= 18 {
        println!("idade={idade}: pode entrar");
    }

    // A linha abaixo não compilaria, porque `idade` é um número, não um bool.
    // if idade {
    //     println!("isso não funciona em Rust");
    // }
}

fn else_basico() {
    // `else` executa um bloco alternativo quando a condição do `if` é `false`.
    let saldo = 25;
    let preco = 40;

    println!("else:");
    if saldo >= preco {
        println!("compra aprovada");
    } else {
        println!("saldo insuficiente");
    }
}

fn else_if() {
    // `else if` permite testar várias condições em sequência.
    // A primeira condição verdadeira vence; as demais não são executadas.
    let nota = 8;

    println!("else if:");
    if nota >= 9 {
        println!("conceito A");
    } else if nota >= 7 {
        println!("conceito B");
    } else if nota >= 5 {
        println!("conceito C");
    } else {
        println!("precisa revisar o conteúdo");
    }
}

fn if_como_expressao() {
    // Em Rust, `if` é uma expressão.
    // Isso significa que ele pode produzir um valor.
    let temperatura = 31;

    println!("if como expressão:");
    let clima = if temperatura >= 30 { "quente" } else { "agradável" };
    println!("temperatura={temperatura}, clima={clima}");

    // Quando `if` produz valor, todos os ramos precisam retornar o mesmo tipo.
    let pontos = if temperatura >= 30 { 10 } else { 5 };
    println!("pontos={pontos}");

    // A linha abaixo não compilaria, porque um ramo retorna `i32` e o outro `&str`.
    // let valor = if temperatura >= 30 { 10 } else { "frio" };
}

fn let_com_if() {
    // Uma forma comum de usar `if` como expressão é atribuir o resultado com `let`.
    // Isso evita criar uma variável mutável apenas para preenchê-la depois.
    let conectado = true;

    println!("let com if:");
    let status = if conectado { "online" } else { "offline" };

    println!("status={status}");

    // Comparação com um estilo mais verboso usando `let mut`.
    let mut status_manual = "offline";
    if conectado {
        status_manual = "online";
    }
    println!("status_manual={status_manual}");
}

fn if_let() {
    // `if let` combina `if` com pattern matching.
    // Ele é útil quando queremos tratar apenas um padrão específico.
    let talvez_nome = Some("Tiago");

    println!("if let:");
    if let Some(nome) = talvez_nome {
        println!("nome encontrado: {nome}");
    } else {
        println!("nenhum nome encontrado");
    }

    // Também funciona com `Result`.
    let numero = "42".parse::<i32>();
    if let Ok(valor) = numero {
        println!("número convertido com sucesso: {valor}");
    }

    // Sem `if let`, teríamos que usar `match` mesmo quando só um caso importa.
    let talvez_idade: Option<u8> = None;
    match talvez_idade {
        Some(idade) => println!("idade={idade}"),
        None => println!("idade ausente"),
    }
}

fn let_else() {
    // `let else` também usa pattern matching, mas é pensado para sair cedo
    // quando o valor não corresponde ao padrão esperado.
    //
    // A parte `else` precisa divergir: normalmente usa `return`, `break`,
    // `continue` ou `panic!`.
    println!("let else:");

    imprime_numero_convertido("123");
    imprime_numero_convertido("abc");
    imprime_primeiro_nome(&["Ana", "Bia", "Caio"]);
    imprime_primeiro_nome(&[]);
}

fn imprime_numero_convertido(texto: &str) {
    let Ok(numero) = texto.parse::<i32>() else {
        println!("'{texto}' não pôde ser convertido para i32");
        return;
    };

    println!("'{texto}' convertido para {numero}");
}

fn imprime_primeiro_nome(nomes: &[&str]) {
    let [primeiro, ..] = nomes else {
        println!("lista de nomes vazia");
        return;
    };

    println!("primeiro nome={primeiro}");
}

pub fn run() {
    println!("##### 2.1 Condicionais #####");
    if_basico();
    else_basico();
    else_if();
    if_como_expressao();
    let_com_if();
    if_let();
    let_else();
}
