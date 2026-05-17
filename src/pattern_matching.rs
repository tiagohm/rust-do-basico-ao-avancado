// 2. Controle de fluxo
// 2.3 - Pattern matching básico
//
// Pattern matching é a ideia de comparar um valor contra padrões.
// Em Rust, isso aparece em `match`, `if let`, `while let`, `let else`
// e em macros como `matches!`.

fn match_basico() {
    // `match` compara um valor com vários braços.
    // Cada braço tem a forma `padrao => expressao`.
    let codigo_http = 200;

    println!("match:");
    match codigo_http {
        200 => println!("requisição bem-sucedida"),
        404 => println!("recurso não encontrado"),
        500 => println!("erro interno do servidor"),
        _ => println!("código desconhecido"),
    }

    // `match` também é uma expressão, então pode produzir um valor.
    let mensagem = match codigo_http {
        200 => "ok",
        404 => "não encontrado",
        500 => "erro",
        _ => "outro status",
    };

    println!("mensagem={mensagem}");
}

fn coringa_underline() {
    // `_` é o padrão coringa.
    // Ele combina com qualquer valor que não tenha sido tratado antes.
    //
    // Use `_` quando:
    // - você não precisa do valor;
    // - quer cobrir todos os casos restantes;
    // - quer manter o `match` exaustivo.
    let caractere = '7';

    println!("_:");
    match caractere {
        '0'..='9' => println!("'{caractere}' é um dígito"),
        'a'..='z' | 'A'..='Z' => println!("'{caractere}' é uma letra ASCII"),
        _ => println!("'{caractere}' é outro tipo de caractere"),
    }
}

fn match_exaustivo() {
    // `match` precisa ser exaustivo: todos os casos possíveis devem ser tratados.
    // Isso é uma das maiores forças do pattern matching em Rust.
    #[derive(Copy, Clone)]
    enum Direcao {
        Norte,
        Sul,
        Leste,
        Oeste,
    }

    fn descrever(direcao: Direcao) -> &'static str {
        match direcao {
            Direcao::Norte => "indo para o norte",
            Direcao::Sul => "indo para o sul",
            Direcao::Leste => "indo para o leste",
            Direcao::Oeste => "indo para o oeste",
        }
    }

    println!("match exaustivo:");
    for direcao in [Direcao::Norte, Direcao::Sul, Direcao::Leste, Direcao::Oeste] {
        println!("{}", descrever(direcao));
    }

    // Se removermos um dos braços acima, o código não compila,
    // porque alguma variante de `Direcao` ficaria sem tratamento.
}

fn match_com_multiplos_padroes() {
    // O operador `|` permite tratar vários padrões no mesmo braço.
    let dia_da_semana = 6;

    println!("match com múltiplos padrões:");
    let tipo_de_dia = match dia_da_semana {
        1..=5 => "dia útil",
        6 | 7 => "fim de semana",
        _ => "dia inválido",
    };

    println!("dia {dia_da_semana}: {tipo_de_dia}");

    let letra = 's';
    match letra {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("'{letra}' é uma vogal minúscula"),
        _ => println!("'{letra}' não é uma vogal minúscula"),
    }
}

fn bindings_em_padroes() {
    // Um padrão pode criar variáveis novas. Essas variáveis são chamadas de bindings.
    // Isso é diferente de comparar com uma variável existente.
    //
    // No braço abaixo, `valor` não é uma variável antiga sendo comparada.
    // Ele é uma variável nova que recebe o conteúdo de `Some(...)`.
    let talvez_numero = Some(7);

    println!("bindings em padrões:");
    match talvez_numero {
        Some(valor) => println!("Some com valor={valor}"),
        None => println!("sem valor"),
    }

    // Essa sintaxe pode confundir:
    // `esperado` dentro do padrão abaixo cria um novo binding e combina com qualquer número.
    // Ela não compara com a variável `esperado` declarada fora.
    let esperado = 10;
    let recebido = 20;
    println!("variável externa esperado={esperado}");

    match recebido {
        esperado => println!("este `esperado` é um novo binding com valor={esperado}"),
    }

    // Para comparar com uma variável externa, use uma guard.
    let esperado = 10;
    let recebido = 20;

    match recebido {
        valor if valor == esperado => println!("recebido é igual ao esperado"),
        valor => println!("recebido={valor}, esperado={esperado}"),
    }

    // O `_` não cria binding; ele descarta o valor.
    // Já um nome começando com `_`, como `_valor`, cria binding, mas silencia aviso de não uso.
    let tupla = ("Rust", 2024);
    match tupla {
        (_nome, ano) => println!("ano={ano}; `_nome` recebeu o texto, mas não será usado"),
    }
}

fn match_guards() {
    // Match guard é uma condição extra colocada depois do padrão com `if`.
    // O braço só é usado se o padrão combinar e a condição for verdadeira.
    let numero = 12;

    println!("match guards:");
    match numero {
        valor if valor < 0 => println!("{valor} é negativo"),
        valor if valor == 0 => println!("{valor} é zero"),
        valor if valor % 2 == 0 => println!("{valor} é positivo e par"),
        valor => println!("{valor} é positivo e ímpar"),
    }

    let ponto = (3, 4);
    match ponto {
        (x, y) if x == y => println!("ponto na diagonal principal"),
        (x, y) if x == 0 || y == 0 => println!("ponto sobre um eixo"),
        (x, y) => println!("ponto comum: x={x}, y={y}"),
    }
}

fn if_let_basico() {
    // `if let` é uma forma curta de fazer pattern matching quando só um padrão importa.
    // Ele é menos verboso que `match` para casos simples.
    let talvez_numero = Some(42);

    println!("if let:");
    if let Some(numero) = talvez_numero {
        println!("número encontrado: {numero}");
    }

    let resultado = "abc".parse::<i32>();
    if let Err(erro) = resultado {
        println!("falha ao converter número: {erro}");
    }
}

fn while_let_basico() {
    // `while let` repete enquanto o valor combinar com o padrão.
    // É comum quando consumimos valores até uma estrutura ficar vazia.
    let mut tarefas = vec!["testar", "compilar", "formatar"];

    println!("while let:");
    while let Some(tarefa) = tarefas.pop() {
        println!("executando tarefa: {tarefa}");
    }
}

fn macro_matches() {
    // `matches!` retorna `true` ou `false` se um valor combina com um padrão.
    // É útil em condições, filtros e testes.
    let status = Some(200);

    println!("matches!:");
    println!("status de sucesso? {}", matches!(status, Some(200..=299)));

    let caracteres = ['a', '7', 'Z', '?'];
    let quantidade_de_digitos = caracteres.iter().filter(|caractere| matches!(caractere, '0'..='9')).count();
    println!("quantidade de dígitos={quantidade_de_digitos}");

    #[derive(Copy, Clone)]
    enum EstadoDaTarefa {
        Pendente,
        EmAndamento,
        Concluida,
        Cancelada,
    }

    let estados = [EstadoDaTarefa::Pendente, EstadoDaTarefa::EmAndamento, EstadoDaTarefa::Concluida, EstadoDaTarefa::Cancelada];

    let finalizadas = estados.iter().filter(|estado| matches!(estado, EstadoDaTarefa::Concluida | EstadoDaTarefa::Cancelada)).count();
    println!("tarefas finalizadas ou canceladas={finalizadas}");

    // `matches!` também aceita guard.
    let texto = Some("rustaceano");
    let comeca_com_rust = matches!(texto, Some(valor) if valor.starts_with("rust"));
    println!("começa com rust? {comeca_com_rust}");
}

pub fn run() {
    println!("##### 2.3 Pattern matching básico #####");
    match_basico();
    coringa_underline();
    match_exaustivo();
    match_com_multiplos_padroes();
    bindings_em_padroes();
    match_guards();
    if_let_basico();
    while_let_basico();
    macro_matches();
}
