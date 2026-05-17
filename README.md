# Rust - Do Básico Ao Avançado

Um único lugar com tudo que você precisa para apreder Rust, do básico ao avançado, com foco em domínio real da linguagem.

O arquivo [main.ts](src/main.rs) funciona como um roteiro de estudo executável. Leia os comentários e rode `cargo run` para ver os exemplos acontecendo.

Abaixo está a lista com tudo que você aprenderá:

## 1. Fundamentos da linguagem

### [Sintaxe básica](src/sintaxe_basica.rs)

* Estrutura de um programa Rust
* `fn main()`
* Comentários
* Blocos `{ }`
* Expressões vs statements
* println! e eprintln!
* Formatação no println
* `let`
* `let mut`
* Shadowing
* Inferência de tipos
* Anotações explícitas de tipo
* Constantes com `const`
* Estáticos com `static`
* Convenções de nomes

### [Tipos primitivos](src/tipos_primitivos.rs)

* Inteiros: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
* Inteiros sem sinal: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
* Floats: `f32`, `f64`
* Literais numéricos, bases e sufixos de tipo
* Booleanos: `bool`
* Caracteres: `char`
* Strings literais: `&str`
* Tuplas
* Arrays
* Slices
* Unit type: `()`
* Never type: `!`

### [Operadores](src/operadores.rs)

* Aritméticos: `+`, `-`, `*`, `/`, `%`
* Comparação: `==`, `!=`, `<`, `>`, `<=`, `>=`
* Booleanos: `&&`, `||`, `!`
* Bitwise: `&`, `|`, `^`, `<<`, `>>`
* Assignment: `=`, `+=`, `-=`, `*=`, `/=`
* Range: `..`, `..=`
* Referência: `&`
* Dereference: `*`
* Precedência de operadores e uso de parênteses

## 2. Controle de fluxo

### [Condicionais](src/condicionais.rs)

* `if`
* `else`
* `else if`
* `if` como expressão
* `let` com `if`
* `if let`
* `let else`

### [Laços](src/lacos.rs)

* `loop`
* `while`
* `while let`
* `for`
* `break`
* `continue`
* Labels em loops
* Retornar valor de `loop`
* Iteração sobre ranges
* Iteração sobre arrays, slices, vetores e iteradores

### [Pattern matching básico](src/pattern_matching.rs)

* `match`
* `_`
* Match exaustivo
* Match com múltiplos padrões
* Bindings em padrões
* Match guards
* `if let`
* `while let`
* `matches!`

## 3. Funções e organização inicial

### [Funções](src/funcoes.rs)

* Declaração de função
* Parâmetros
* Retorno
* Retorno implícito
* Retorno explícito com `return`
* Funções associadas
* Métodos
* `self`, `&self` e `&mut self`
* Overloading inexistente em Rust
* Funções genéricas
* Turbofish `::<T>`

### [Módulos básicos](src/modulos_basicos.rs)

* `mod`
* `pub`
* `use`
* Caminhos com `crate::`
* Caminhos com `self::`
* Caminhos com `super::`
* Arquivos `main.rs` e `lib.rs`
* Separação em múltiplos arquivos

## 4. Ownership, Borrowing e Lifetimes

Este é o núcleo da linguagem.

### Ownership

* Move semantics
* Copy semantics
* O trait `Copy`
* O trait `Clone`
* Quando um valor é movido
* Quando um valor é copiado
* Ownership em funções
* Ownership em retornos
* Escopo e destruição
* `Drop`

### Borrowing

* Referências imutáveis: `&T`
* Referências mutáveis: `&mut T`
* Regra: várias referências imutáveis ou uma mutável
* Borrow checker
* Reborrowing
* Borrow splitting
* Borrow temporário
* Non-lexical lifetimes, NLL
* Dangling references
* Empréstimo de campos de structs
* Empréstimo em loops
* Empréstimo em closures

### Lifetimes

* Lifetime implícito
* Lifetime explícito
* Parâmetros de lifetime
* Lifetime elision
* Lifetime em funções
* Lifetime em structs
* Lifetime em enums
* Lifetime em impl blocks
* Lifetime em traits
* `'static`
* Higher-ranked trait bounds: `for<'a>`
* Variância de lifetimes
* Subtyping de lifetimes
* Lifetimes em async
* Lifetimes com `Cow`
* Lifetimes com iteradores

## 5. Tipos compostos

### Structs

* Structs nomeadas
* Tuple structs
* Unit structs
* Campos públicos e privados
* Struct update syntax
* `impl`
* Métodos
* Associated functions
* Builders
* Newtype pattern
* Phantom types

### Enums

* Enums simples
* Enums com dados
* Enums recursivos
* Representação de estado com enums
* `Option<T>`
* `Result<T, E>`
* `Ordering`
* `ControlFlow`
* `Infallible`
* `Never`

### Pattern matching avançado

* Desestruturação de structs
* Desestruturação de enums
* Desestruturação de tuplas
* Desestruturação de arrays
* `ref`
* `ref mut`
* `@` bindings
* Ranges em patterns
* Match guards
* Patterns irrefutáveis
* Patterns refutáveis
* `let else`
* `if let`
* `while let`

## 6. Strings e texto

### Strings

* `&str`
* `String`
* Diferença entre `String` e `&str`
* UTF-8
* Indexação inexistente de string
* Iteração por bytes
* Iteração por chars
* Iteração por grapheme clusters
* `to_string()`
* `String::from`
* `push`
* `push_str`
* `format!`
* `write!`
* `writeln!`
* `Display`
* `Debug`
* Conversão entre `String`, `&str`, `Vec<u8>`, `OsString`, `PathBuf`

### Formatação

* `{}` com `Display`
* `{:?}` com `Debug`
* `{:#?}` pretty debug
* Formatação numérica
* Padding
* Alinhamento
* Precisão
* Implementação manual de `Display`
* Implementação manual de `Debug`

## 7. Coleções padrão

### Coleções principais

* `Vec<T>`
* `VecDeque<T>`
* `LinkedList<T>`
* `HashMap<K, V>`
* `BTreeMap<K, V>`
* `HashSet<T>`
* `BTreeSet<T>`
* `BinaryHeap<T>`

### Tópicos associados

* Capacidade vs tamanho
* `push`
* `pop`
* `insert`
* `remove`
* `retain`
* `drain`
* `split_off`
* `entry`
* `or_insert`
* `or_insert_with`
* Hashing
* Ordenação
* Comparação
* Iteração
* Mutação durante iteração
* Slices
* `sort`
* `sort_by`
* `sort_by_key`
* `binary_search`
* `dedup`
* `partition_point`

## 8. Iteradores

### Fundamentos

* `Iterator`
* `IntoIterator`
* `iter`
* `iter_mut`
* `into_iter`
* Lazy evaluation
* Consuming adaptors
* Iterator adaptors

### Métodos importantes

* `map`
* `filter`
* `filter_map`
* `flat_map`
* `flatten`
* `fold`
* `reduce`
* `for_each`
* `collect`
* `sum`
* `product`
* `count`
* `any`
* `all`
* `find`
* `position`
* `enumerate`
* `zip`
* `chain`
* `take`
* `skip`
* `take_while`
* `skip_while`
* `peekable`
* `rev`
* `cycle`
* `chunks`
* `windows`

### Avançado

* Criar iteradores próprios
* Implementar `Iterator`
* Iteradores com lifetimes
* Iteradores mutáveis
* Double-ended iterators
* Exact size iterators
* Fused iterators
* Streaming iterators
* Performance de iteradores
* Zero-cost abstractions

## 9. Traits

### Fundamentos

* Definir traits
* Implementar traits
* Métodos obrigatórios
* Métodos default
* Trait bounds
* `where`
* Traits como interfaces
* Traits como constraints genéricas

### Traits padrão essenciais

* `Debug`
* `Display`
* `Clone`
* `Copy`
* `Default`
* `PartialEq`
* `Eq`
* `PartialOrd`
* `Ord`
* `Hash`
* `From`
* `Into`
* `TryFrom`
* `TryInto`
* `AsRef`
* `AsMut`
* `Borrow`
* `BorrowMut`
* `Deref`
* `DerefMut`
* `Drop`
* `Iterator`
* `IntoIterator`
* `Read`
* `Write`
* `Send`
* `Sync`
* `Unpin`
* `Future`

### Traits avançados

* Associated types
* Generic associated types, GATs
* Associated constants
* Supertraits
* Trait objects
* `dyn Trait`
* Object safety
* `impl Trait`
* Return-position `impl Trait`
* Argument-position `impl Trait`
* Trait aliases
* Blanket implementations
* Coherence rules
* Orphan rule
* Negative impls
* Auto traits
* Marker traits
* Sealed traits
* Extension traits

## 10. Generics

### Básico

* Funções genéricas
* Structs genéricas
* Enums genéricos
* Traits genéricos
* Bounds
* Múltiplos parâmetros genéricos
* `where` clauses

### Avançado

* Monomorphization
* Static dispatch
* Dynamic dispatch
* `PhantomData`
* Type-state pattern
* Const generics
* Generic associated types
* Higher-ranked trait bounds
* Variância
* Especialização, conceito e limitações
* Zero-sized types
* Type-level programming

## 11. Error handling

### Básico

* `panic!`
* `Option<T>`
* `Result<T, E>`
* `unwrap`
* `expect`
* `?`
* `match` com erro
* Propagação de erro
* `main` retornando `Result`

### Intermediário

* `map_err`
* `ok_or`
* `ok_or_else`
* `and_then`
* `or_else`
* `transpose`
* `flatten`
* Erros customizados
* Enum de erro
* Implementar `std::error::Error`
* Implementar `Display` para erro
* Conversão com `From`

### Crates populares

* `thiserror`
* `anyhow`
* `eyre`
* `color-eyre`

### Avançado

* Backtraces
* Error contexts
* Erros em bibliotecas vs aplicações
* Design de API com erro
* Erros recuperáveis vs irrecuperáveis
* Erros em async
* Erros em FFI
* Erros sem alocação
* `no_std` error handling

## 12. Smart pointers e alocação

### Smart pointers principais

* `Box<T>`
* `Rc<T>`
* `Arc<T>`
* `Cell<T>`
* `RefCell<T>`
* `Mutex<T>`
* `RwLock<T>`
* `Cow<T>`
* `Pin<T>`
* `Weak<T>`

### Tópicos importantes

* Heap vs stack
* Ownership compartilhado
* Interior mutability
* Reference counting
* Ciclos de referência
* `Rc` vs `Arc`
* `RefCell` vs `Mutex`
* `Box` para tipos recursivos
* `Box<dyn Trait>`
* `Arc<Mutex<T>>`
* `Arc<RwLock<T>>`
* `Arc<Atomic*>`
* `Weak` para evitar ciclos
* `Cow` para clone sob demanda
* `Pin` para estabilidade de endereço

## 13. Memória e modelo de execução

### Fundamentos

* Stack
* Heap
* Layout de memória
* Alinhamento
* Padding
* Tamanho de tipos
* `std::mem::size_of`
* `std::mem::align_of`
* Move
* Drop
* RAII

### Avançado

* `MaybeUninit<T>`
* `ManuallyDrop<T>`
* `mem::replace`
* `mem::take`
* `mem::swap`
* `mem::forget`
* `transmute`
* Niches
* Null pointer optimization
* Layout de enums
* `repr(Rust)`
* `repr(C)`
* `repr(transparent)`
* `repr(packed)`
* `repr(align)`
* Provenance de ponteiros
* Stacked Borrows
* Undefined behavior
* Aliasing rules
* Allocators
* Custom allocators

## 14. Concorrência e paralelismo

### Threads

* `std::thread`
* `spawn`
* `join`
* Move closures em threads
* Thread-local storage

### Compartilhamento seguro

* `Send`
* `Sync`
* `Arc<T>`
* `Mutex<T>`
* `RwLock<T>`
* `Condvar`
* `Barrier`
* `Once`
* `OnceLock`
* `LazyLock`
* `thread::scope`

### Channels

* `std::sync::mpsc`
* Multi-producer single-consumer
* `crossbeam-channel`
* `flume`
* Backpressure
* Bounded channels
* Unbounded channels

### Atomics

* `AtomicBool`
* `AtomicUsize`
* `AtomicI64`
* `AtomicPtr`
* `Ordering`
* `Relaxed`
* `Acquire`
* `Release`
* `AcqRel`
* `SeqCst`
* Compare-and-swap
* Lock-free basics

### Paralelismo

* `rayon`
* Parallel iterators
* Work stealing
* Data parallelism
* Task parallelism
* Granularidade
* False sharing
* Cache locality

## 15. Async Rust

### Fundamentos

* `async fn`
* `.await`
* `Future`
* Estado interno de uma future
* Polling
* Wakers
* Executors
* Runtimes

### Tokio

* `tokio::main`
* `tokio::spawn`
* `JoinHandle`
* `tokio::select!`
* `tokio::time`
* `tokio::sync::mpsc`
* `tokio::sync::oneshot`
* `tokio::sync::broadcast`
* `tokio::sync::watch`
* `tokio::sync::Mutex`
* `tokio::sync::RwLock`
* `Semaphore`
* `Notify`

### Async avançado

* `Send` em futures
* Futures não-`Send`
* `LocalSet`
* Cancelamento
* Timeouts
* Backpressure
* Streams
* `StreamExt`
* Sinks
* Async traits
* `async-trait`
* Pinning em async
* Self-referential futures
* Structured concurrency
* Graceful shutdown
* Async I/O
* Async networking
* Async file I/O
* Blocking dentro de async
* `spawn_blocking`

## 16. Macros

### Macros declarativas

* `macro_rules!`
* Match de tokens
* Fragment specifiers
* Repetição
* Higiene
* Exportação de macros
* Macros recursivas
* DSLs simples com macros

### Macros procedurais

* Crates de proc macro
* Derive macros
* Attribute macros
* Function-like proc macros
* `TokenStream`
* `syn`
* `quote`
* `proc_macro2`
* Parsing
* Geração de código
* Spans
* Diagnósticos
* Hygiene em proc macros
* Testes de macros

### Macros padrão

* `println!`
* `format!`
* `vec!`
* `dbg!`
* `todo!`
* `unimplemented!`
* `panic!`
* `assert!`
* `assert_eq!`
* `matches!`
* `include_str!`
* `include_bytes!`
* `concat!`
* `env!`
* `option_env!`

## 17. Cargo e ecossistema

### Cargo básico

* `cargo new`
* `cargo init`
* `cargo build`
* `cargo run`
* `cargo test`
* `cargo check`
* `cargo doc`
* `cargo clean`
* `Cargo.toml`
* Dependências
* Dev-dependencies
* Build-dependencies
* Features
* Workspaces

### Cargo avançado

* Profiles
* `release`
* `dev`
* LTO
* Codegen units
* Incremental compilation
* Target-specific dependencies
* Patch
* Replace
* Vendoring
* Cargo registry
* Publicação no crates.io
* Versionamento semântico
* Lockfile
* `cargo metadata`
* Build scripts com `build.rs`
* Environment variables
* Cross-compilation
* Custom targets

### Ferramentas

* `rustup`
* `rustfmt`
* `clippy`
* `rust-analyzer`
* `miri`
* `cargo-expand`
* `cargo-audit`
* `cargo-deny`
* `cargo-nextest`
* `cargo-criterion`
* `cargo-fuzz`
* `cargo-udeps`
* `cargo-bloat`
* `cargo-llvm-lines`
* `cross`

## 18. Testes

### Testes básicos

* `#[test]`
* `assert!`
* `assert_eq!`
* `assert_ne!`
* Testes unitários
* Testes de integração
* Módulo `tests`
* Diretório `tests/`

### Testes avançados

* Fixtures
* Testes parametrizados
* Testes com erro
* Testes que devem causar panic
* `#[should_panic]`
* Ignorar testes
* Testes concorrentes
* Testes determinísticos
* Testes de documentação
* Property-based testing
* `proptest`
* `quickcheck`
* Fuzzing
* `cargo-fuzz`
* Snapshot testing
* `insta`
* Mutation testing
* Benchmark testing

## 19. Documentação

* Comentários `//`
* Comentários `///`
* Comentários `//!`
* `cargo doc`
* Exemplos em documentação
* Doctests
* Documentar panics
* Documentar erros
* Documentar safety invariants
* Documentar complexidade
* Documentar features
* Documentar módulos
* Documentar APIs públicas
* `#[doc(hidden)]`
* `#[cfg(doc)]`

## 20. Crates essenciais

### Serialização

* `serde`
* `serde_json`
* `toml`
* `bincode`
* `ron`
* `postcard`

### Logging e tracing

* `log`
* `env_logger`
* `tracing`
* `tracing-subscriber`
* `tracing-appender`

### CLI

* `clap`
* `argh`
* `dialoguer`
* `indicatif`

### Web e HTTP

* `reqwest`
* `hyper`
* `axum`
* `actix-web`
* `warp`
* `tower`
* `tower-http`

### Banco de dados

* `sqlx`
* `diesel`
* `sea-orm`
* `tokio-postgres`
* `rusqlite`

### Async

* `tokio`
* `async-std`
* `futures`
* `async-trait`
* `pin-project`

### Performance

* `rayon`
* `criterion`
* `smallvec`
* `arrayvec`
* `bytes`
* `parking_lot`
* `dashmap`
* `crossbeam`

### Datas e tempo

* `chrono`
* `time`
* `humantime`

### Erros

* `thiserror`
* `anyhow`
* `eyre`

### Matemática

* `num`
* `nalgebra`
* `glam`
* `ndarray`
* `approx`

## 21. Sistemas, arquivos e I/O

### Arquivos

* `std::fs`
* `File`
* `OpenOptions`
* `Read`
* `Write`
* `BufReader`
* `BufWriter`
* Paths
* `Path`
* `PathBuf`
* Diretórios
* Metadados
* Permissões

### I/O

* I/O bloqueante
* I/O assíncrono
* Buffers
* Streams
* Sockets TCP
* UDP
* Unix sockets
* Pipes
* Stdin/stdout/stderr
* Memory-mapped files
* `mmap`

### Sistema operacional

* Variáveis de ambiente
* Argumentos de linha de comando
* Processos
* `Command`
* Exit codes
* Signals
* Daemon/service
* Integração com systemd
* Portabilidade Linux/macOS/Windows

## 22. Unsafe Rust

### Fundamentos

* Blocos `unsafe`
* Funções `unsafe`
* Traits `unsafe`
* Impl `unsafe`
* O que `unsafe` permite fazer
* O que `unsafe` não desativa
* Invariantes de segurança

### Ponteiros crus

* `*const T`
* `*mut T`
* Criar ponteiros crus
* Dereferenciar ponteiros crus
* Pointer arithmetic
* `addr_of!`
* `addr_of_mut!`
* Ponteiros nulos
* Alignment
* Aliasing
* Provenance

### APIs unsafe

* `get_unchecked`
* `from_raw_parts`
* `from_raw_parts_mut`
* `Box::into_raw`
* `Box::from_raw`
* `Arc::into_raw`
* `Arc::from_raw`
* `Vec::from_raw_parts`
* `MaybeUninit`
* `ManuallyDrop`

### Design seguro sobre unsafe

* Criar abstrações seguras
* Minimizar região unsafe
* Documentar `## Safety`
* Testar com Miri
* Evitar undefined behavior
* FFI seguro
* Wrappers seguros
* Soundness

## 23. FFI e integração com C/C++

### FFI

* `extern "C"`
* `#[no_mangle]`
* `#[repr(C)]`
* Chamar C a partir de Rust
* Chamar Rust a partir de C
* Strings C
* `CString`
* `CStr`
* Ponteiros nulos
* Ownership entre linguagens
* Callbacks
* Tratamento de erro em FFI
* ABI
* Linkagem

### Ferramentas

* `bindgen`
* `cbindgen`
* `cc`
* `cmake`
* `pkg-config`

### Avançado

* FFI com C++
* `cxx`
* `autocxx`
* Bibliotecas dinâmicas
* Bibliotecas estáticas
* Cross-compilation
* Windows DLL
* Linux `.so`
* macOS `.dylib`
* WASM FFI

## 24. WebAssembly

* Compilar Rust para WASM
* `wasm32-unknown-unknown`
* `wasm-bindgen`
* `wasm-pack`
* `web-sys`
* `js-sys`
* Interop com JavaScript
* Passagem de strings
* Passagem de arrays
* Serialização
* Performance em WASM
* WASI
* `wasmtime`
* `wasmer`

## 25. `no_std` e embarcados

### `no_std`

* Diferença entre `std`, `core` e `alloc`
* `#![no_std]`
* `#![no_main]`
* Panic handlers
* Allocator global
* Collections com `alloc`
* APIs disponíveis em `core`
* APIs disponíveis em `alloc`

### Embedded

* HALs
* `embedded-hal`
* Interrupções
* Registradores
* Periféricos
* `cortex-m`
* `rtic`
* Drivers
* Comunicação serial
* SPI
* I2C
* Timers
* DMA
* Baixo consumo
* Real-time constraints

## 26. Performance e otimização

### Fundamentos

* Zero-cost abstractions
* Monomorphization
* Static dispatch
* Dynamic dispatch
* Inline
* Otimizações do compilador
* Release mode
* `cargo build --release`

### Técnicas

* Evitar alocações
* Reutilizar buffers
* Slices em vez de `Vec`
* `&str` em vez de `String`
* `Cow`
* `SmallVec`
* `ArrayVec`
* `Bytes`
* Cache locality
* Branch prediction
* SIMD
* Data-oriented design
* Pooling
* Arena allocation
* Lock contention
* False sharing

### Profiling

* Benchmark com `criterion`
* Flamegraphs
* `perf`
* `valgrind`
* `heaptrack`
* `cargo-bloat`
* `cargo-llvm-lines`
* Medir antes de otimizar
* Análise de alocação
* Análise de binário

## 27. SIMD e baixo nível

* SIMD portátil
* `std::simd`
* Intrinsics
* `core::arch`
* SSE
* AVX
* AVX2
* AVX-512
* NEON
* Runtime CPU feature detection
* `is_x86_feature_detected!`
* Alinhamento de dados
* Vetorização automática
* Vetorização manual
* Fallbacks portáveis

## 28. Arquitetura de software em Rust

### Design de APIs

* APIs orientadas a ownership
* APIs com borrowed data
* APIs genéricas
* APIs com traits
* APIs com builders
* APIs com typestate
* APIs sem alocação
* APIs thread-safe
* APIs async
* APIs compatíveis com `no_std`

### Padrões comuns

* Newtype pattern
* Builder pattern
* Type-state pattern
* Strategy pattern com traits
* Command pattern
* Visitor pattern
* Extension trait
* Sealed trait
* RAII guard
* Interior mutability pattern
* Arena pattern
* ECS pattern
* Actor model
* Pipeline pattern

### Organização

* Crate binária vs crate biblioteca
* Workspaces
* Módulos internos
* APIs públicas mínimas
* Feature flags
* Separação por domínio
* Separação por plataforma
* Testabilidade
* Documentação pública

## 29. Rust para aplicações web/backend

* HTTP server
* Rotas
* Middleware
* Extractors
* State compartilhado
* `Arc<AppState>`
* Conexão com banco
* Pool de conexões
* Migrações
* JSON
* Validação
* Autenticação
* Autorização
* Rate limiting
* Observabilidade
* Graceful shutdown
* Background jobs
* Filas
* WebSockets
* SSE
* Upload de arquivos
* Configuração
* Secrets
* Deploy

## 30. Rust para bibliotecas matemáticas/científicas

* Tipos numéricos genéricos
* Traits numéricos
* Precisão numérica
* Erros de ponto flutuante
* Comparação com tolerância
* `f32` vs `f64`
* Estabilidade numérica
* Vetores e matrizes
* Álgebra linear
* Interpolação
* Otimização
* Root-finding
* Integração numérica
* Paralelização
* SIMD
* Layout de arrays
* `Vec<T>` vs slices
* `ndarray`
* `nalgebra`
* APIs sem alocação
* APIs com buffers fornecidos pelo chamador

## 31. Rust para sistemas distribuídos

* TCP
* UDP
* QUIC
* gRPC
* Protocol Buffers
* Cap’n Proto
* MessagePack
* Kafka
* NATS
* Redis
* Postgres
* Backpressure
* Retries
* Timeouts
* Circuit breakers
* Idempotência
* Consistência
* Concorrência
* Observabilidade
* Tolerância a falhas

## 32. Segurança

* Memory safety
* Thread safety
* Evitar data races
* Validação de entrada
* Parsing seguro
* Criptografia
* `ring`
* `rustls`
* TLS
* Hashing
* Constant-time operations
* Supply chain security
* `cargo audit`
* `cargo deny`
* Secrets
* Sanitizers
* Fuzzing
* Hardening
* Unsafe review

## 33. Compilação avançada

* `rustc`
* MIR
* LLVM IR
* Assembly output
* `cargo asm`
* Linker
* LTO
* ThinLTO
* Panic strategy: unwind vs abort
* Codegen units
* Target triples
* Custom target JSON
* Cross-compilation
* Static linking
* Dynamic linking
* Musl
* MinGW
* MSVC
* Build scripts
* Conditional compilation
* `cfg`
* `cfg_attr`
* Feature flags

## 34. Edição, linting e qualidade

* `rustfmt`
* `clippy`
* Warnings como errors
* `#![deny(warnings)]`
* `#![forbid(unsafe_code)]`
* `#![deny(missing_docs)]`
* `#![deny(clippy::unwrap_used)]`
* `#![deny(clippy::expect_used)]`

## 35. Tópicos internos do compilador

Para nível expert real:

* AST
* HIR
* MIR
* Borrow checker
* Polonius
* Trait solver
* Coherence
* Monomorphization
* Drop checking
* Variance checking
* Lifetime inference
* Type inference
* Macro expansion
* Name resolution
* LLVM backend
* Incremental compilation
* Diagnostics
* Rust editions

## 36. Rust editions e linguagem moderna

* Rust 2015
* Rust 2018
* Rust 2021
* Rust 2024
* Mudanças entre editions
* Prelude
* Path clarity
* Async/await
* IntoIterator para arrays
* Resolver de features
* Novidades de pattern matching
* Novidades em lifetimes
* Compatibilidade entre editions

## 37. Tópicos avançados específicos

* `Pin`
* `Unpin`
* Self-referential structs
* `PhantomPinned`
* `Future` manual
* `Poll`
* `Waker`
* Intrusive data structures
* Arena allocation
* Slab allocation
* Generational indices
* ECS
* Lock-free programming
* Hazard pointers
* Epoch-based reclamation
* `crossbeam-epoch`
* Memory ordering avançado
* Async runtimes internos
* Executors customizados
* Allocators customizados
* Procedural macro frameworks
* Parser combinators
* Zero-copy parsing
* Serialization sem alocação
* Compile-time computation
* Const evaluation
* `const fn`
* Const generics avançado
* Typenum-like programming

## 38. Anti-patterns em Rust

* Usar `clone()` sem necessidade
* Usar `String` quando `&str` basta
* Usar `Vec<T>` quando `&[T]` basta
* Usar `Arc<Mutex<T>>` para tudo
* Usar `unwrap()` em código de produção
* Expor tipos internos na API pública
* Criar lifetimes desnecessários
* Usar `RefCell` para escapar do borrow checker sem motivo
* Usar `unsafe` prematuramente
* Overengineering com traits
* Generalizar cedo demais
* Ignorar erros
* Bloquear thread dentro de async
* Segurar lock durante `.await`
* Retornar `Box<dyn Error>` em bibliotecas sem critério
* Criar abstrações que escondem ownership importante

## 39. Projetos práticos para consolidar

### Básico

* CLI de calculadora
* Conversor de unidades
* Parser de CSV simples
* Contador de palavras
* Jogo da adivinhação
* Manipulador de arquivos

### Intermediário

* Servidor HTTP simples
* Cliente HTTP
* Cache LRU
* Thread pool
* Mini banco key-value
* Parser JSON simples
* Sistema de logs
* Biblioteca de vetores/matrizes
* Web scraper
* CLI com subcomandos

### Avançado

* Runtime async simples
* Executor de futures
* Banco key-value persistente
* Parser zero-copy
* Biblioteca FFI para C
* Servidor web com Postgres
* Sistema de atores
* Engine ECS
* Biblioteca SIMD
* Allocator simples
* Interpretador de linguagem
* Compilador pequeno
* Sistema de filas
* Biblioteca matemática sem alocação
* Algoritmos astronômicos performáticos

## 40. Ordem sugerida de estudo

1. Sintaxe, tipos primitivos e controle de fluxo
2. Funções, structs, enums e `match`
3. Ownership, borrowing e referências
4. `Option`, `Result` e tratamento de erro
5. Coleções e strings
6. Iteradores
7. Traits e generics
8. Módulos, crates e Cargo
9. Testes e documentação
10. Smart pointers: `Box`, `Rc`, `Arc`, `RefCell`, `Mutex`
11. Concorrência com threads, channels e locks
12. Async Rust com Tokio
13. Macros
14. Performance e profiling
15. Unsafe Rust
16. FFI
17. `no_std`, WASM ou backend, conforme seu objetivo
18. Internals do compilador, memory model e design avançado de bibliotecas
