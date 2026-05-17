// 4. Ownership, Borrowing e Lifetimes
// 4.3 - Lifetimes
//
// Lifetimes descrevem por quanto tempo referências são válidas.
// Elas não fazem um valor viver mais; elas apenas dão nomes às relações
// entre referências para que o compilador consiga provar que o código é seguro.
//
// Em Rust, referências não são ponteiros soltos sem controle. Toda referência
// precisa apontar para um valor ainda vivo. Lifetimes são a forma de o compilador
// comparar "a vida" da referência com "a vida" do valor referenciado.
//
// Sem essa análise, seria possível retornar ou guardar referências para dados
// que já foram destruídos, criando dangling references. Em linguagens sem essa
// checagem, esse tipo de erro pode aparecer como uso de memória inválida,
// corrupção de dados ou bugs difíceis de reproduzir. Rust rejeita esses casos
// em tempo de compilação.
//
// Na maior parte do código, o compilador infere os lifetimes. Escrevemos
// lifetimes explicitamente quando há mais de uma referência e precisamos dizer
// qual relação existe entre entrada e saída, ou quando um tipo guarda referências.
//
// Sintaxe:
// - nomes de lifetime começam com apóstrofo: `'a`, `'b`, `'texto`;
// - apesar da aparência, lifetime não é label de loop;
// - label de loop aparece antes de um laço, como `'saida: loop { ... }`;
// - lifetime aparece em tipos e assinaturas, como `&'a str`.

use std::borrow::Cow;

fn lifetime_implicito() {
    // Muitas referências têm lifetime implícito.
    // Aqui não escrevemos lifetime nenhum, mas o compilador sabe que `referencia`
    // não pode viver mais que `texto`.
    println!("Lifetime implícito:");

    let texto = String::from("Rust");
    let referencia = &texto;
    println!("referencia={referencia}");

    // A referência é válida enquanto `texto` ainda estiver vivo.
    println!("texto ainda existe={texto}");
}

fn lifetime_explicito() {
    // Lifetime explícito dá nome à relação entre uma referência recebida e
    // uma referência retornada.
    //
    // `fn primeiro<'a>(texto: &'a str) -> &'a str`
    //
    // Leia assim:
    // - a função recebe uma referência válida por algum lifetime `'a`;
    // - a referência retornada será válida pelo mesmo `'a`;
    // - a função não cria uma nova vida útil, apenas descreve a relação.
    fn primeiro_caractere<'a>(texto: &'a str) -> &'a str {
        &texto[0..1]
    }

    println!("Lifetime explícito:");
    let linguagem = String::from("Rust");
    let primeiro = primeiro_caractere(&linguagem);
    println!("primeiro={primeiro}");
}

fn parametros_de_lifetime() {
    // Parâmetros de lifetime ficam entre `<...>`, como parâmetros genéricos de tipo.
    // A função abaixo diz que `a`, `b` e o retorno precisam ser válidos por um
    // mesmo lifetime chamado `'a`.
    //
    // Por que isso não compilaria sem lifetime explícito?
    //
    // A função recebe duas referências: `a` e `b`.
    // O retorno pode ser `a` ou pode ser `b`, dependendo do `if`.
    // Sem a anotação, o compilador não sabe se o retorno deve ficar ligado
    // ao lifetime de `a`, ao lifetime de `b`, ou a alguma outra relação.
    //
    // Ao escrever `<'a>` e usar `&'a str` nos dois parâmetros e no retorno,
    // dizemos: "o retorno será uma referência válida enquanto for seguro usar
    // tanto `a` quanto `b` dentro de um mesmo lifetime comum".
    fn maior_texto<'a>(a: &'a str, b: &'a str) -> &'a str {
        if a.len() >= b.len() { a } else { b }
    }

    println!("Parâmetros de lifetime:");
    let curto = String::from("Rust");
    let longo = String::from("Ferrugem");
    let maior = maior_texto(&curto, &longo);
    println!("maior={maior}");

    // A assinatura não significa que `a` e `b` vivem exatamente o mesmo tempo.
    // Ela significa que o retorno só será considerado válido enquanto ambos
    // puderem ser usados com segurança.
}

fn lifetime_elision() {
    // Lifetime elision são regras que permitem omitir lifetimes em casos comuns.
    //
    // Esta função:
    fn tamanho(texto: &str) -> usize {
        texto.len()
    }

    // É entendida como se fosse:
    fn tamanho_explicito<'a>(texto: &'a str) -> usize {
        texto.len()
    }

    // Para uma única referência de entrada, o compilador consegue ligar a saída
    // a essa entrada quando o retorno também é referência.
    fn primeiro(texto: &str) -> &str {
        &texto[0..1]
    }

    println!("Lifetime elision:");
    let texto = "Rust";
    println!("tamanho={}", tamanho(texto));
    println!("tamanho_explicito={}", tamanho_explicito(texto));
    println!("primeiro={}", primeiro(texto));
}

fn lifetime_em_structs() {
    // Structs podem guardar referências, mas precisam declarar por quanto tempo
    // essas referências são válidas.
    //
    // `Trecho<'a>` significa: uma instância de `Trecho` não pode viver mais que
    // a referência `texto` guardada dentro dela.
    #[derive(Debug)]
    struct Trecho<'a> {
        texto: &'a str,
    }

    println!("Lifetime em structs:");
    let frase = String::from("Rust é seguro");
    let trecho = Trecho { texto: &frase[0..4] };
    println!("trecho={trecho:?}, texto={}", trecho.texto);

    // Não seria possível retornar `Trecho` apontando para uma `String` local
    // destruída no fim de uma função.
}

fn lifetime_em_enums() {
    // Enums também podem carregar referências e, por isso, podem ter lifetimes.
    #[derive(Debug)]
    enum Conteudo<'a> {
        Texto(&'a str),
        Numero(i32),
    }

    println!("Lifetime em enums:");
    let titulo = String::from("Capítulo");
    let item = Conteudo::Texto(&titulo);
    let numero = Conteudo::Numero(4);

    match item {
        Conteudo::Texto(texto) => println!("texto={texto}"),
        Conteudo::Numero(valor) => println!("numero={valor}"),
    }

    println!("outro item={numero:?}");
}

fn lifetime_em_impl_blocks() {
    // Quando uma struct tem lifetime, o bloco `impl` normalmente repete esse
    // parâmetro para implementar métodos daquele tipo.
    struct Documento<'a> {
        titulo: &'a str,
        corpo: &'a str,
    }

    impl<'a> Documento<'a> {
        fn novo(titulo: &'a str, corpo: &'a str) -> Documento<'a> {
            Documento { titulo, corpo }
        }

        fn titulo(&self) -> &str {
            self.titulo
        }

        fn resumo(&self) -> &str {
            &self.corpo[0..self.corpo.len().min(10)]
        }
    }

    println!("Lifetime em impl blocks:");
    let titulo = String::from("Lifetimes");
    let corpo = String::from("Referências com validade controlada");
    let documento = Documento::novo(&titulo, &corpo);
    println!("titulo={}, resumo={}", documento.titulo(), documento.resumo());
}

fn lifetime_em_traits() {
    // Traits também podem usar lifetimes.
    // Aqui, o trait promete devolver uma referência válida por `'a`.
    trait Nomeado<'a> {
        fn nome(&self) -> &'a str;
    }

    struct Usuario<'a> {
        nome: &'a str,
    }

    impl<'a> Nomeado<'a> for Usuario<'a> {
        fn nome(&self) -> &'a str {
            self.nome
        }
    }

    println!("Lifetime em traits:");
    let nome = String::from("Ana");
    let usuario = Usuario { nome: &nome };
    println!("usuario.nome()={}", usuario.nome());
}

fn lifetime_static() {
    // `'static` significa que uma referência pode viver durante todo o programa.
    // Strings literais são `&'static str`, porque ficam embutidas no binário.

    let literal: &'static str = "texto embutido no binário";

    println!("'static:");
    println!("literal={literal}");

    // Atenção: `T: 'static` não significa necessariamente que o valor já vive
    // para sempre. Significa que ele não contém referências não-estáticas.
    fn exige_static<T: 'static + std::fmt::Debug>(valor: T) {
        println!("valor aceito por T: 'static = {valor:?}");
    }

    exige_static(String::from("String owned também satisfaz T: 'static"));

    // A linha abaixo não compilaria, porque `referencia` depende de `local`.
    //
    // let local = String::from("curto");
    // let referencia = &local;
    // exige_static(referencia);
}

fn higher_ranked_trait_bounds() {
    // Higher-ranked trait bounds usam `for<'a>`.
    //
    // Sintaxe:
    // for<'a> Fn(&'a str) -> usize
    //
    // Leia assim:
    // "este valor chamável precisa aceitar uma referência `&str` com qualquer
    // lifetime escolhido por quem chama".
    //
    // Essa sintaxe não é exclusiva de closures.
    // Ela pode aparecer em bounds de traits para funções, closures, function pointers
    // e tipos que implementam um trait para qualquer lifetime.
    //
    // Sem `for<'a>`, um bound poderia ser interpretado como algo amarrado a um
    // lifetime específico. Com `for<'a>`, exigimos algo mais forte: a função
    // precisa funcionar para referências temporárias, locais, `'static`, curtas
    // ou longas. Isso aparece em APIs que recebem callbacks e querem chamá-los
    // com referências criadas dentro da própria função, sem expor esses lifetimes
    // para quem chamou.
    fn aceita_qualquer_lifetime<F>(f: F)
    where
        F: for<'a> Fn(&'a str) -> usize,
    {
        let local = String::from("abc");
        let literal: &'static str = "abcdef";

        println!("local={}", f(&local));
        println!("literal={}", f(literal));
    }

    fn tamanho(texto: &str) -> usize {
        texto.len()
    }

    println!("Higher-ranked trait bounds: for<'a>");
    aceita_qualquer_lifetime(tamanho);
}

fn variancia_de_lifetimes() {
    // Variância descreve quando um tipo com lifetime maior pode ser usado onde
    // um lifetime menor é esperado.
    //
    // Referências imutáveis são covariantes:
    // - um `&'static str` pode ser usado como um `&'a str` mais curto.
    fn usar_texto<'a>(texto: &'a str) {
        println!("texto={texto}");
    }

    println!("Variância de lifetimes:");
    let texto_estatico: &'static str = "vive até o fim do programa";
    usar_texto(texto_estatico);

    // Referências mutáveis são mais restritas porque permitem alterar o valor
    // apontado. Em capítulos avançados, isso aparece como invariância em alguns
    // tipos mutáveis e containers.
    //
    // Por enquanto, a regra prática é:
    // - `&T` costuma ser flexível;
    // - `&mut T` exige mais cuidado, porque representa acesso exclusivo.
}

fn subtyping_de_lifetimes() {
    // Subtyping de lifetimes aparece na ideia de que um lifetime maior pode ser
    // encurtado para um contexto menor.
    //
    // `'static` é o lifetime mais longo. Ele pode ser usado onde uma referência
    // de lifetime menor é suficiente.
    fn escolher<'a>(texto: &'a str, _fallback: &'a str) -> &'a str {
        texto
    }

    println!("Subtyping de lifetimes:");
    let local = String::from("local");
    let estatico: &'static str = "estático";

    let escolhido = escolher(estatico, &local);
    println!("escolhido={escolhido}");

    // O retorno é tratado como válido apenas enquanto o lifetime comum for seguro.
    // Mesmo recebendo `'static`, o resultado aqui não precisa ser usado como `'static`.
}

fn lifetimes_em_async() {
    // Funções async podem capturar referências.
    // A future gerada não pode viver mais que as referências que ela captura.
    async fn tamanho_async(texto: &str) -> usize {
        texto.len()
    }

    println!("Lifetimes em async:");
    let texto = String::from("async");
    let future = tamanho_async(&texto);

    // Não vamos executar a future aqui para evitar introduzir runtime async agora.
    // O ponto importante é: `future` guarda uma referência para `texto`,
    // então ela precisa ser descartada antes de `texto` sair de escopo.
    drop(future);
    println!("future criada e descartada antes de `texto`: {texto}");

    // Em código real, se uma future for enviada para outro thread ou guardada por
    // muito tempo, ela geralmente precisa ser `'static` ou possuir os dados.
}

fn lifetimes_com_cow() {
    // `Cow` significa Clone On Write.
    // Ele pode guardar:
    // - uma referência emprestada: `Cow::Borrowed`;
    // - um valor owned: `Cow::Owned`.
    //
    // `Cow<'a, str>` significa que, se estiver emprestado, o texto precisa viver
    // pelo menos pelo lifetime `'a`.
    //
    // Às vezes uma função recebe texto e talvez precise modificá-lo. Se o texto
    // já estiver no formato correto, podemos devolver uma referência para o texto
    // original sem alocar nada. Se for necessário alterar, criamos uma `String`
    // owned apenas nesse caso.
    //
    // Assim, `Cow<'a, str>` deixa a API dizer:
    // - "posso devolver algo emprestado do argumento";
    // - "mas também posso devolver um valor owned se precisei transformar".
    //
    // Isso evita `clone`/alocação desnecessária em caminhos comuns.
    fn normalizar<'a>(texto: &'a str) -> Cow<'a, str> {
        if texto.chars().all(char::is_lowercase) { Cow::Borrowed(texto) } else { Cow::Owned(texto.to_lowercase()) }
    }

    println!("Lifetimes com Cow:");
    let minusculo = normalizar("rust");
    let misto = normalizar("Rust");

    println!("minusculo={minusculo} ({})", tipo_cow(&minusculo));
    println!("misto={misto} ({})", tipo_cow(&misto));
}

fn tipo_cow(valor: &Cow<'_, str>) -> &'static str {
    match valor {
        Cow::Borrowed(_) => "Borrowed",
        Cow::Owned(_) => "Owned",
    }
}

fn lifetimes_com_iteradores() {
    // Iteradores podem produzir referências.
    // Nesse caso, as referências produzidas não podem viver mais que a coleção
    // ou texto de origem.
    fn palavras<'a>(texto: &'a str) -> impl Iterator<Item = &'a str> {
        texto.split_whitespace()
    }

    println!("Lifetimes com iteradores:");
    let frase = String::from("Rust sem garbage collector");

    for palavra in palavras(&frase) {
        println!("palavra={palavra}");
    }

    // O iterador abaixo empresta `frase`.
    let mut iterador = palavras(&frase);
    println!("primeira={:?}", iterador.next());
    println!("segunda={:?}", iterador.next());

    // Não podemos deixar esse iterador escapar para além da vida de `frase`,
    // porque ele produz `&str` apontando para dentro dela.
}

pub fn run() {
    println!("##### 4.3 Lifetimes #####");
    lifetime_implicito();
    lifetime_explicito();
    parametros_de_lifetime();
    lifetime_elision();
    lifetime_em_structs();
    lifetime_em_enums();
    lifetime_em_impl_blocks();
    lifetime_em_traits();
    lifetime_static();
    higher_ranked_trait_bounds();
    variancia_de_lifetimes();
    subtyping_de_lifetimes();
    lifetimes_em_async();
    lifetimes_com_cow();
    lifetimes_com_iteradores();
}
