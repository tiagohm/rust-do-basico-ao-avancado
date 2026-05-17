// 1. Fundamentos da linguagem
// 1.1 - Sintaxe básica

// Constantes são substituídas em tempo de compilação e precisam ter tipo explícito.
// Por convenção, nomes de constantes usam SCREAMING_SNAKE_CASE.
const VERSAO_DO_CURSO: u8 = 1;
const HORAS_POR_DIA: u8 = 24;

// `static` cria um valor com endereço fixo durante toda a execução do programa.
// Também usa SCREAMING_SNAKE_CASE por convenção.
static NOME_DO_CURSO: &str = "Rust - do básico ao avançado";

fn estrutura_de_um_programa() {
    // Um programa Rust normalmente é organizado em:
    // - crate: unidade de compilação. Este projeto é uma crate binária.
    // - módulos: arquivos ou blocos `mod` que agrupam código.
    // - itens: funções, structs, enums, constantes, traits etc.
    // - função `main`: ponto de entrada de uma crate binária.
    println!("Curso: {NOME_DO_CURSO}");
    println!("Versão do roteiro: {VERSAO_DO_CURSO}");
}

/// Comentário de documentação:
/// normalmente aparece antes de funções, structs, enums e módulos públicos.
/// A ferramenta `cargo doc` usa esse tipo de comentário para gerar docs.
fn comentarios() {
    // Comentário de linha: usado para explicar uma linha ou pequeno trecho.

    /*
        Comentário de bloco:
        pode ocupar várias linhas e também pode ser usado temporariamente
        para desativar pequenos trechos durante o estudo.
    */

    println!("Comentários também fazem parte da leitura do código.");
}

fn blocos() {
    // Blocos são delimitados por chaves `{ }`.
    // Eles criam um novo escopo: variáveis criadas dentro do bloco não existem fora.
    let externo = "estou no escopo da função";

    {
        let interno = "estou no escopo do bloco";
        println!("{externo}");
        println!("{interno}");
    }

    // A linha abaixo não compilaria, porque `interno` só existe dentro do bloco.
    // println!("{interno}");

    // Blocos também podem produzir valores.
    let dobro = {
        let numero = 21;
        numero * 2 // sem ponto e vírgula: esta expressão vira o valor do bloco
    };

    println!("O dobro calculado por um bloco é {dobro}");
}

fn expressoes_vs_statements() {
    // Statement: executa uma ação, mas não produz um valor para ser usado.
    let numero = 10;

    // Expression: produz um valor.
    let soma = numero + 5;

    // `if` em Rust é uma expressão, então pode ser usado para preencher variáveis.
    let classificacao = if soma >= 15 { "alto" } else { "baixo" };

    println!("numero = {numero}, soma = {soma}, classificacao = {classificacao}");

    // Ponto e vírgula transforma uma expressão em statement.
    let com_valor = {
        let x = 2;
        x + 3
    };

    let sem_valor = {
        let x = 2;
        let _ = x + 3; // calculado e ignorado de propósito
    };

    println!("Bloco sem ; retorna {com_valor}");
    println!("Bloco com ; retorna o valor unitário: {sem_valor:?}");
}

fn println_e_eprintln() {
    // `println!` é uma macro, não uma função comum.
    // O `!` no final indica chamada de macro.
    //
    // Ela escreve texto na saída padrão do programa, chamada stdout.
    // Ao final, `println!` sempre adiciona uma quebra de linha.
    println!("println! escreve uma linha em stdout.");

    // O texto base é chamado de string de formato.
    // As chaves `{}` são espaços reservados para valores.
    let linguagem = "Rust";
    let ano = 2024;
    println!("Estou estudando {linguagem} na edição {ano}.");

    // Também é possível passar valores depois da string de formato.
    // Nesse caso, cada `{}` usa o próximo argumento na ordem.
    println!("{} é uma linguagem de sistemas lançada em {}.", "Rust", 2015);

    // Argumentos posicionais permitem reutilizar ou reorganizar valores.
    println!("{0} apareceu antes de {1}; depois repetimos {0}.", "Rust", "Go");

    // Argumentos nomeados deixam mensagens longas mais legíveis.
    println!("{nome} concluiu {concluidas} de {total} aulas.", nome = "Tiago", concluidas = 3, total = 12);

    // Para imprimir chaves literalmente, duplique as chaves.
    println!("Em Rust, blocos usam chaves assim: {{ }}");

    // `print!` é parecido com `println!`, mas não adiciona quebra de linha no final.
    print!("Esta parte fica na mesma linha; ");
    println!("esta completa a linha.");

    // `format!` usa a mesma sintaxe de formatação, mas retorna uma String
    // em vez de escrever no terminal.
    let mensagem = format!("Mensagem montada com format!: {linguagem}");
    println!("{mensagem}");

    // `eprintln!` usa a mesma sintaxe de `println!`, mas escreve na saída de erro,
    // chamada stderr. Ela é usada para erros, alertas, logs e diagnósticos.
    eprintln!("eprintln! escreve em stderr, não em stdout.");

    // Diferença prática:
    // - stdout é a saída principal do programa, normalmente usada para resultados;
    // - stderr é separada, normalmente usada para mensagens de erro ou diagnóstico;
    // - em terminais comuns as duas aparecem juntas, mas podem ser redirecionadas separadamente.
    //
    // Exemplo em terminal:
    // cargo run > saida.txt 2> erros.txt
    //
    // Nesse comando, o que foi escrito com `println!` vai para `saida.txt`;
    // o que foi escrito com `eprintln!` vai para `erros.txt`.
}

fn formatacao_no_println() {
    // Em `println!`, as chaves `{}` indicam onde um valor será colocado.
    // Existem diferentes "formatadores" para mostrar valores.
    //
    // `{}` usa o formatador Display:
    // - bom para mensagens finais ao usuário;
    // - funciona apenas com tipos que sabem se apresentar de forma amigável.
    let linguagem = "Rust";
    println!("Com Display: {linguagem}");

    // `{:?}` usa o formatador Debug:
    // - bom para estudar, testar e inspecionar valores durante o desenvolvimento;
    // - mostra uma representação técnica do valor;
    // - funciona para muitos tipos da biblioteca padrão;
    // - para tipos criados por nós, normalmente usamos `#[derive(Debug)]`.
    let numeros = [10, 20, 30];
    println!("Array com Debug: {numeros:?}");

    #[derive(Debug)]
    struct Coordenada {
        x: i32,
        y: i32,
    }

    let ponto = Coordenada { x: 3, y: 7 };
    println!("Campos acessados diretamente: x = {}, y = {}", ponto.x, ponto.y);

    // Uma struct própria não pode ser impressa com `{}` sem implementarmos Display.
    // A linha abaixo não compilaria:
    // println!("Ponto com Display: {ponto}");

    // Como usamos `#[derive(Debug)]`, podemos imprimir com `{:?}`.
    println!("Ponto com Debug: {ponto:?}");

    // `:#?` também usa Debug, mas em várias linhas, facilitando a leitura.
    println!("Ponto com Debug formatado:\n{ponto:#?}");
}

fn let_e_let_mut() {
    // `let` cria uma variável imutável por padrão.
    let linguagem = "Rust";
    println!("Linguagem escolhida: {linguagem}");

    // A linha abaixo não compilaria, porque `linguagem` não é mutável.
    // linguagem = "Go";

    // `let mut` cria uma variável que pode ser alterada.
    let mut tentativas = 1;
    println!("Tentativas antes: {tentativas}");

    tentativas = tentativas + 1;
    println!("Tentativas depois: {tentativas}");
}

fn shadowing() {
    // Shadowing acontece quando declaramos uma nova variável com o mesmo nome.
    // A nova variável "sombreia" a anterior a partir daquele ponto.
    let valor = 5;
    let valor = valor + 10;
    let valor = format!("{valor} pontos");

    println!("Valor depois de shadowing: {valor}");

    // Diferente de `mut`, shadowing permite mudar o tipo do valor.
    let espacos = "   ";
    let espacos = espacos.len();

    println!("Quantidade de espaços: {espacos}");
}

fn inferencia_e_anotacoes_de_tipo() {
    // Inferência de tipos: Rust deduz o tipo quando há contexto suficiente.
    let idade = 30; // por padrão, aqui Rust usa i32.
    let preco = 19.90; // por padrão, aqui Rust usa f64.
    let ativo = true;

    println!("idade = {idade}, preco = {preco}, ativo = {ativo}");

    // Anotação explícita de tipo: útil quando queremos controlar exatamente o tipo.
    let contador: u32 = 42;
    let temperatura: f32 = 23.5;
    let inicial: char = 'R';
    let nome: &str = "Rust";

    println!("contador = {contador}, temperatura = {temperatura}, inicial = {inicial}, nome = {nome}");

    // Algumas situações precisam de anotação, porque existem vários tipos possíveis.
    let numero_parseado: i32 = "123".parse().expect("a string literal '123' deve ser um inteiro válido");

    println!("Número convertido de texto: {numero_parseado}");
}

fn constantes_e_estaticos() {
    // `const`:
    // - sempre exige tipo explícito;
    // - deve receber um valor computável em tempo de compilação;
    // - pode ser usada em qualquer escopo;
    // - não tem um endereço fixo garantido.
    const MINUTOS_POR_HORA: u8 = 60;

    let minutos_por_dia = HORAS_POR_DIA as u16 * MINUTOS_POR_HORA as u16;
    println!("Um dia tem {minutos_por_dia} minutos.");

    // `static`:
    // - vive durante toda a execução do programa;
    // - tem endereço fixo;
    // - é usado para dados globais compartilhados.
    println!("Static global: {NOME_DO_CURSO}");

    // Rust também permite `static mut`, mas acessar e alterar estado global mutável
    // exige `unsafe`. Para código iniciante, prefira constantes, parâmetros e retornos.
}

fn convencoes_de_nomes() {
    // Rust usa convenções de nomes para deixar o código previsível:
    //
    // - variáveis e funções: snake_case
    // - constantes e statics: SCREAMING_SNAKE_CASE
    // - tipos como structs/enums/traits: PascalCase
    // - crates e módulos: snake_case

    let nome_do_usuario = "Tiago";
    let total_de_aulas = 12;

    struct AlunoMatriculado {
        nome: String,
        aulas_concluidas: u8,
    }

    let aluno = AlunoMatriculado { nome: nome_do_usuario.to_string(), aulas_concluidas: 3 };

    println!("{nome} concluiu {concluidas}/{total} aulas.", nome = aluno.nome, concluidas = aluno.aulas_concluidas, total = total_de_aulas);
}

pub fn run() {
    println!("##### 1.1 Sintaxe básica #####");
    estrutura_de_um_programa();
    comentarios();
    blocos();
    expressoes_vs_statements();
    println_e_eprintln();
    formatacao_no_println();
    let_e_let_mut();
    shadowing();
    inferencia_e_anotacoes_de_tipo();
    constantes_e_estaticos();
    convencoes_de_nomes();
}
