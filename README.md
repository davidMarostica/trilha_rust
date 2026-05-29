# trilha_rust

Repositório de aprendizado em Rust com exercícios, desafios e projetos didáticos organizados em crates independentes.

**Autor:** David Aparecido da Silva

**Contato:** davidmarosticasilvasilva25@gmail.com

## Visão geral

Este workspace contém dois conjuntos principais de projetos:

- `VARIAVEL/` — exercícios básicos de Rust sobre variáveis, controle de fluxo, funções e estruturas de dados.
- `DESAFIOS/` — desafios extras para praticar entrada de dados e lógica de programa.

Cada subdiretório é um crate Rust separado com seu próprio `Cargo.toml`.

## Estrutura do projeto

- `README.md` — documentação geral do workspace.
- `VARIAVEL/` — exercícios básicos em Rust.
- `DESAFIOS/` — projetos de desafio e exemplos adicionais.
- `.gitignore` — arquivos e diretórios ignorados pelo Git.
- `.vscode/` — configurações do VS Code.

## Exercícios em `VARIAVEL/`

- `condicionais` — controle de fluxo e condicionais.
- `dados` — manipulação de dados e estruturas básicas.
- `exercicio1` — primeiro exercício introdutório.
- `exercicio_2` — segundo exercício de prática.
- `exercicio_3` — terceiro exercício de prática.
- `exercicio_04` — quarto exercício.
- `funcoes` — exemplos de funções em Rust.
- `heap` — conceitos de heap e memória dinâmica.
- `shadowing` — demonstração de shadowing de variáveis.
- `stack` — exemplos sobre pilha e escopo de variáveis.
- `variavel` — exemplos básicos de declaração e uso de variáveis.

## Desafios em `DESAFIOS/`

- `bytebank_saldo` — desafio de controle de saldo.
- `saudacao_cliente` — leitura de entrada e saudação personalizada.

## Como executar qualquer projeto

Navegue até a pasta do crate desejado e execute:

```bash
cd VARIAVEL/condicionais
cargo run
```

ou, para um desafio:

```bash
cd DESAFIOS/saudacao_cliente
cargo run
```

## Executar a partir do diretório raiz

Também é possível executar qualquer crate diretamente do root usando `--manifest-path`:

```bash
cargo run --manifest-path VARIAVEL/condicionais/Cargo.toml
cargo run --manifest-path DESAFIOS/saudacao_cliente/Cargo.toml
```

## Compilar um projeto específico

Para compilar um crate sem executá-lo:

```bash
cargo build --manifest-path VARIAVEL/condicionais/Cargo.toml
```

ou

```bash
cargo build --manifest-path DESAFIOS/saudacao_cliente/Cargo.toml
```

## Observações

Este workspace é destinado ao estudo de Rust e à prática de conceitos básicos e intermediários. Cada crate é independente, permitindo explorar um exemplo por vez.

 