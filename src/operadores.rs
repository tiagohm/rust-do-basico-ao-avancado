// 1. Fundamentos da linguagem
// 1.3 - Operadores
//
// Operadores são símbolos que executam operações sobre valores.
// Eles aparecem em expressões e produzem novos valores, alteram variáveis
// ou criam formas de acessar dados.

fn operadores_aritmeticos() {
    // Operadores aritméticos trabalham com números.
    let a = 10;
    let b = 3;

    println!("Operadores aritméticos:");
    println!("{a} + {b} = {}", a + b); // soma
    println!("{a} - {b} = {}", a - b); // subtração
    println!("{a} * {b} = {}", a * b); // multiplicação
    println!("{a} / {b} = {}", a / b); // divisão inteira, porque `a` e `b` são inteiros
    println!("{a} % {b} = {}", a % b); // resto da divisão

    // Para divisão com casas decimais, use floats.
    let x = 10.0;
    let y = 3.0;
    println!("{x} / {y} = {}", x / y);
}

fn operadores_de_comparacao() {
    // Operadores de comparação sempre produzem um `bool`.
    let a = 10;
    let b = 3;

    println!("Operadores de comparação:");
    println!("{a} == {b}: {}", a == b); // igual
    println!("{a} != {b}: {}", a != b); // diferente
    println!("{a} < {b}: {}", a < b); // menor que
    println!("{a} > {b}: {}", a > b); // maior que
    println!("{a} <= {b}: {}", a <= b); // menor ou igual
    println!("{a} >= {b}: {}", a >= b); // maior ou igual

    // Comparações são muito usadas em `if`, `while` e filtros de iteradores.
    if a > b {
        println!("{a} é maior que {b}");
    }
}

fn operadores_booleanos() {
    // Operadores booleanos trabalham com valores `bool`.
    let usuario_logado = true;
    let usuario_admin = false;

    println!("Operadores booleanos:");
    println!("logado && admin: {}", usuario_logado && usuario_admin); // E lógico
    println!("logado || admin: {}", usuario_logado || usuario_admin); // OU lógico
    println!("!logado: {}", !usuario_logado); // NÃO lógico

    // `&&` e `||` fazem short-circuit:
    // - em `false && ...`, o lado direito nem é avaliado;
    // - em `true || ...`, o lado direito nem é avaliado.
    let pode_acessar = usuario_logado && usuario_admin;
    println!("pode_acessar={pode_acessar}");
}

fn operadores_bitwise() {
    // Operadores bitwise trabalham bit a bit sobre inteiros.
    // Eles são comuns em flags, máscaras, protocolos binários e código de baixo nível.
    let a: u8 = 0b1010;
    let b: u8 = 0b1100;

    println!("Operadores bitwise:");
    println!("{a:04b} & {b:04b} = {:04b}", a & b); // AND bit a bit
    println!("{a:04b} | {b:04b} = {:04b}", a | b); // OR bit a bit
    println!("{a:04b} ^ {b:04b} = {:04b}", a ^ b); // XOR bit a bit
    println!("{a:04b} << 1 = {:04b}", a << 1); // desloca bits para a esquerda
    println!("{a:04b} >> 1 = {:04b}", a >> 1); // desloca bits para a direita

    // Exemplo simples de flags.
    const LER: u8 = 0b0001;
    const ESCREVER: u8 = 0b0010;
    const EXECUTAR: u8 = 0b0100;

    let permissoes = LER | ESCREVER;
    println!("permissões={permissoes:03b}");
    println!("pode ler? {}", permissoes & LER != 0);
    println!("pode escrever? {}", permissoes & ESCREVER != 0);
    println!("pode executar? {}", permissoes & EXECUTAR != 0);
}

fn operadores_de_atribuicao() {
    // `=` atribui um valor inicial ou substitui o valor de uma variável mutável.
    let mut contador = 10;

    println!("Operadores de atribuição:");
    println!("contador inicial={contador}");

    contador += 5; // contador = contador + 5
    println!("depois de += 5: {contador}");

    contador -= 3; // contador = contador - 3
    println!("depois de -= 3: {contador}");

    contador *= 2; // contador = contador * 2
    println!("depois de *= 2: {contador}");

    contador /= 4; // contador = contador / 4
    println!("depois de /= 4: {contador}");

    // Também existem `%=`, `&=`, `|=`, `^=`, `<<=` e `>>=`.
    contador %= 3;
    println!("depois de %= 3: {contador}");
}

fn operadores_de_range() {
    // Ranges representam intervalos.
    // `..` cria um intervalo exclusivo no final.
    // `..=` cria um intervalo inclusivo no final.
    println!("Operadores de range:");

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

    let letras = ['a', 'b', 'c', 'd', 'e'];
    let meio = &letras[1..4];
    println!("slice com 1..4: {meio:?}");

    // Outras formas úteis:
    println!("..3 pega do começo até antes do índice 3: {:?}", &letras[..3]);
    println!("2.. pega do índice 2 até o fim: {:?}", &letras[2..]);
    println!(".. pega tudo: {:?}", &letras[..]);
}

fn operador_de_referencia() {
    // `&` cria uma referência, ou seja, um empréstimo de um valor.
    // Referências permitem acessar um valor sem tomar ownership dele.
    let nome = String::from("Rust");
    let referencia: &String = &nome;

    println!("Operador de referência:");
    println!("nome original={nome}");
    println!("referência={referencia}");

    // Em muitos casos, preferimos receber `&str` em vez de `&String`,
    // porque `&str` aceita tanto strings literais quanto partes de `String`.
    imprime_tamanho(&nome);
    imprime_tamanho("texto literal");

    // Referência mutável: `&mut`.
    let mut contador = 0;
    incrementa(&mut contador);
    println!("contador depois de &mut: {contador}");
}

fn imprime_tamanho(texto: &str) {
    println!("'{texto}' tem {} bytes", texto.len());
}

fn incrementa(valor: &mut i32) {
    *valor += 1;
}

fn operador_de_dereference() {
    // `*` dereferencia uma referência ou ponteiro inteligente.
    // Dereferenciar significa acessar o valor apontado pela referência.
    let numero = 41;
    let referencia = &numero;

    println!("Operador de dereference:");
    println!("referencia aponta para {}", *referencia);

    let mut contador = 10;
    let referencia_mutavel = &mut contador;
    *referencia_mutavel += 5;
    println!("contador depois de modificar via *: {contador}");

    // `Box<T>` é um ponteiro inteligente que guarda valor no heap.
    // Ao usar `*`, acessamos o valor dentro do Box.
    let alocado = Box::new(99);
    println!("valor dentro do Box: {}", *alocado);

    // Em chamadas de método e acesso simples, Rust aplica deref coercion em vários casos,
    // então nem sempre precisamos escrever `*` manualmente.
}

pub fn run() {
    println!("##### 1.3 Operadores #####");
    operadores_aritmeticos();
    operadores_de_comparacao();
    operadores_booleanos();
    operadores_bitwise();
    operadores_de_atribuicao();
    operadores_de_range();
    operador_de_referencia();
    operador_de_dereference();
}
