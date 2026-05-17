// 1. Fundamentos da linguagem
// 1.2 - Tipos primitivos
//
// Tipos primitivos são os tipos mais básicos fornecidos pela linguagem.
// Eles formam a base para construir structs, enums, coleções e APIs maiores.

fn inteiros_com_sinal() {
    // Inteiros com sinal aceitam valores negativos, zero e positivos.
    // O `i` vem de integer, e o número indica quantos bits o tipo ocupa.
    let pequeno: i8 = -128;
    let curto: i16 = -32_768;
    let padrao: i32 = -2_147_483_648; // tipo inteiro padrão quando não há contexto.
    let longo: i64 = -9_223_372_036_854_775_808;
    let enorme: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;

    // `isize` tem o mesmo tamanho de um ponteiro da plataforma:
    // - 32 bits em alvos 32-bit;
    // - 64 bits em alvos 64-bit.
    // Ele aparece bastante em índices, diferenças entre posições e APIs de baixo nível.
    let dependente_da_plataforma: isize = -10;

    println!("Inteiros com sinal:");
    println!("i8={pequeno}, i16={curto}, i32={padrao}, i64={longo}");
    println!("i128={enorme}");
    println!("isize={dependente_da_plataforma}, isize::MIN={}, isize::MAX={}", isize::MIN, isize::MAX);
}

fn inteiros_sem_sinal() {
    // Inteiros sem sinal não aceitam valores negativos.
    // O `u` vem de unsigned.
    let pequeno: u8 = 255;
    let curto: u16 = 65_535;
    let padrao_explicito: u32 = 4_294_967_295;
    let longo: u64 = 18_446_744_073_709_551_615;
    let enorme: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;

    // `usize` também acompanha o tamanho do ponteiro da plataforma.
    // Ele é o tipo usado para índices e tamanhos de coleções.
    let indice: usize = 2;
    let nomes = ["Ana", "Bia", "Caio"];

    println!("Inteiros sem sinal:");
    println!("u8={pequeno}, u16={curto}, u32={padrao_explicito}, u64={longo}");
    println!("u128={enorme}");
    println!("usize={indice}, nomes[{indice}]={}", nomes[indice]);
}

fn floats() {
    // Floats representam números com parte decimal.
    // Rust tem dois tipos primitivos de ponto flutuante:
    // - `f32`: 32 bits, menor e menos preciso;
    // - `f64`: 64 bits, padrão quando não há contexto, mais preciso.
    let preco: f32 = 19.90;
    let distancia: f64 = 384_400.0;
    let divisao = 10.0 / 3.0; // f64 por inferência.

    println!("Floats:");
    println!("f32 preco={preco}");
    println!("f64 distancia={distancia}");
    println!("10.0 / 3.0 = {divisao}");

    // Floats podem representar valores especiais.
    println!("f64::INFINITY = {}", f64::INFINITY);
    println!("f64::NAN é NaN? {}", f64::NAN.is_nan());

    // Em código real, evite comparar floats com `==` quando houver cálculos envolvidos;
    // prefira comparar usando uma tolerância.
    let quase_zero_tres = 0.1_f64 + 0.2_f64;
    let diferenca = (quase_zero_tres - 0.3_f64).abs();
    println!("0.1 + 0.2 = {quase_zero_tres}, diferença para 0.3 = {diferenca}");
}

fn literais_numericos_e_sufixos() {
    // Literais numéricos são valores escritos diretamente no código, como `10`.
    // Rust permite escrever esses literais de várias formas para melhorar a leitura.
    //
    // Separador visual:
    // - `_` pode ser usado dentro de números para separar grupos;
    // - ele não muda o valor, apenas facilita a leitura.
    let um_milhao = 1_000_000;

    // Bases numéricas:
    // - decimal: `255`;
    // - hexadecimal: `0xff`;
    // - octal: `0o377`;
    // - binário: `0b1111_1111`;
    // - byte ASCII: `b'A'`, que produz um `u8`.
    let decimal = 255;
    let hexadecimal = 0xff;
    let octal = 0o377;
    let binario = 0b1111_1111;
    let byte_ascii = b'A';

    // Sufixo de tipo:
    // - `10u8` força o literal a ser `u8`;
    // - `10_i64` força o literal a ser `i64`;
    // - `3.14f32` força o literal a ser `f32`.
    //
    // O sufixo é útil quando a inferência não tem contexto suficiente
    // ou quando queremos ensinar claramente o tipo usado.
    let pequeno = 10u8;
    let longo = 10_i64;
    let pi_aproximado = 3.14f32;

    println!("Literais numéricos e sufixos:");
    println!("um_milhao={um_milhao}");
    println!("decimal={decimal}, hexadecimal={hexadecimal}, octal={octal}, binario={binario}");
    println!("byte ASCII b'A'={byte_ascii}");
    println!("pequeno={pequeno}, longo={longo}, pi_aproximado={pi_aproximado}");
}

fn booleanos() {
    // `bool` possui apenas dois valores: `true` e `false`.
    // Ele é usado em condicionais, laços e expressões lógicas.
    let usuario_ativo: bool = true;
    let possui_permissao = false; // tipo inferido como bool.

    println!("Booleanos:");
    println!("usuario_ativo={usuario_ativo}, possui_permissao={possui_permissao}");
}

fn caracteres() {
    // `char` representa um valor Unicode escalar, não apenas um byte ASCII.
    // Por isso, um `char` em Rust ocupa 4 bytes.
    let letra: char = 'R';
    let cedilha: char = 'ç';
    let emoji: char = '🦀';

    println!("Caracteres:");
    println!("letra={letra}, cedilha={cedilha}, emoji={emoji}");
    println!("tamanho de char em bytes: {}", std::mem::size_of::<char>());
}

fn strings_literais() {
    // `&str` é uma fatia de string, geralmente apontando para texto UTF-8.
    // Strings literais ficam embutidas no binário e têm tempo de vida `'static`.
    let saudacao: &str = "Olá, Rust!";

    println!("Strings literais:");
    println!("{saudacao}");
    println!("len() conta bytes, não caracteres: {}", saudacao.len());
    println!("chars().count() conta caracteres Unicode: {}", saudacao.chars().count());

    // `&str` é imutável. Para texto mutável e alocado no heap, use `String`,
    // que será estudado com mais detalhes no tópico de strings.
    // saudacao.push_str("!"); // não compila: `&str` não tem `push_str`.
}

fn tuplas() {
    // Tuplas agrupam valores de tipos possivelmente diferentes em uma ordem fixa.
    let pessoa: (&str, u8, bool) = ("Tiago", 35, true);

    // Acesso por índice.
    println!("Tuplas:");
    println!("nome={}, idade={}, ativo={}", pessoa.0, pessoa.1, pessoa.2);

    // Desestruturação: extrai os campos para variáveis.
    let (nome, idade, ativo) = pessoa;
    println!("{nome}: idade={idade}, ativo={ativo}");
}

fn arrays() {
    // Arrays têm tamanho fixo e todos os elementos têm o mesmo tipo.
    // O tipo `[i32; 4]` significa: array de `i32` com 4 posições.
    let numeros: [i32; 4] = [10, 20, 30, 40];
    let repetidos = [0; 5]; // cria [0, 0, 0, 0, 0].

    println!("Arrays:");
    println!("numeros={numeros:?}");
    println!("primeiro={}, tamanho={}", numeros[0], numeros.len());
    println!("repetidos={repetidos:?}");

    // Acesso fora dos limites causa panic em tempo de execução.
    // println!("{}", numeros[99]);
}

fn slices() {
    // Slice é uma visão emprestada sobre uma sequência contígua de elementos.
    // Diferente do array, o tamanho do slice pode ser conhecido apenas em tempo de execução.
    let numeros = [10, 20, 30, 40, 50];
    let todos: &[i32] = &numeros;
    let meio: &[i32] = &numeros[1..4];

    println!("Slices:");
    println!("todos={todos:?}");
    println!("meio={meio:?}");
    println!("soma do meio={}", meio.iter().sum::<i32>());

    // `&str` também é um tipo de slice: uma fatia de bytes UTF-8 válidos.
    let texto = "Rust";
    let texto_como_slice: &str = &texto[0..2];
    println!("slice de string={texto_como_slice}");
}

fn unit_type() {
    // O unit type é `()`.
    // Ele representa ausência de valor significativo.
    let valor_unitario: () = ();

    // Funções sem tipo de retorno explícito retornam `()` por padrão.
    fn apenas_imprime() {
        println!("Esta função retorna ().");
    }

    // Um bloco terminado com ponto e vírgula também produz `()`.
    let resultado_do_bloco = {
        let numero = 10;
        let _ = numero * 2;
    };

    println!("Unit type:");
    apenas_imprime();
    println!("valor_unitario={valor_unitario:?}");
    println!("resultado_do_bloco={resultado_do_bloco:?}");
}

fn never_type() {
    // O never type é `!`.
    // Ele representa algo que nunca produz um valor porque nunca retorna normalmente.
    //
    // Exemplos comuns:
    // - `panic!()`;
    // - `loop {}` infinito;
    // - funções que encerram o processo;
    // - caminhos impossíveis em controle de fluxo.
    //
    // A função abaixo retorna `!`. Ela compila, mas não será chamada aqui,
    // porque chamá-la encerraria este roteiro com panic.
    fn falha(mensagem: &str) -> ! {
        panic!("{mensagem}");
    }

    let exemplo_de_funcao_que_nunca_retorna: fn(&str) -> ! = falha;
    println!("Never type:");
    println!("Uma função com assinatura `fn(&str) -> !` nunca retorna normalmente.");

    // O tipo `!` pode ser usado em lugares onde outro tipo era esperado,
    // porque um valor real nunca chega a existir.
    let talvez_numero = Some(42);
    let numero: i32 = match talvez_numero {
        Some(valor) => valor,
        None => exemplo_de_funcao_que_nunca_retorna("sem número disponível"),
    };

    println!("Número obtido sem acionar o caminho `!`: {numero}");
}

pub fn run() {
    println!("##### 1.2 Tipos primitivos #####");
    inteiros_com_sinal();
    inteiros_sem_sinal();
    floats();
    literais_numericos_e_sufixos();
    booleanos();
    caracteres();
    strings_literais();
    tuplas();
    arrays();
    slices();
    unit_type();
    never_type();
}
