# AGENTS.md

## Projeto

Este repositório é um roteiro de estudo para aprender Rust do básico ao avançado. O código deve ser escrito como material didático: simples, progressivo, executável e com comentários explicativos em português do Brasil.

Use o `README.md` como mapa geral dos tópicos. Cada módulo em `src/` deve representar uma parte do roteiro e ser chamado a partir de `src/main.rs` quando fizer sentido.

## Estilo do Código

- Use Rust 2024, conforme `Cargo.toml`.
- Mantenha identificadores de código em ASCII e seguindo as convenções de Rust:
  - funções, variáveis e módulos em `snake_case`;
  - constantes e `static` em `SCREAMING_SNAKE_CASE`;
  - tipos em `PascalCase`.
- Mantenha comentários e textos exibidos em português com diacríticos corretos.
- Prefira exemplos pequenos e autocontidos, que compilem e possam ser executados com `cargo run`.
- Ao mostrar código que não deve compilar, deixe-o comentado e explique o motivo.
- Evite abstrações avançadas antes de o tópico correspondente aparecer no roteiro.
- Preserve o caráter educativo: comentários devem explicar o conceito, não apenas repetir a linha de código.

## Formatação

- Respeite `rustfmt.toml`; este projeto usa `max_width = 999`.
- Rode `cargo fmt` após editar arquivos Rust.
- Não reformate arquivos sem necessidade fora do escopo da tarefa.

## Validação

Depois de alterar código Rust, rode pelo menos:

```bash
cargo fmt --check
cargo check
```

Quando a mudança alterar exemplos executáveis ou saída didática, rode também:

```bash
cargo run
```

Se algum comando falhar por problema ambiental, reporte claramente o comando, o erro e o que foi ou não validado.

## Organização Didática

- Prefira criar um arquivo por tópico principal do roteiro, por exemplo `src/sintaxe_basica.rs`.
- Exponha uma função `pub fn run()` em cada módulo didático para que `main.rs` possa chamar o tópico.
- Dentro de cada módulo, divida o conteúdo em funções menores por subtópico.
- Use `println!` para tornar os exemplos observáveis durante `cargo run`.
- Evite entrada interativa nos exemplos iniciais; isso mantém o roteiro simples de executar.

## Git e Escopo

- Não reverta mudanças do usuário.
- Faça alterações estreitamente relacionadas ao pedido.
- Antes de editar, confira rapidamente o arquivo alvo e o estado relevante do projeto.

# Codebase Knowledge Graph

- Esse projeto não está indexado, portanto, NUNCA utilize a ferramenta `codebase-memory-mcp`.
