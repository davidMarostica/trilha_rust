# saudacao_cliente

Programa simples em Rust para ler uma entrada do usuário e exibir uma mensagem de saudação personalizada.

## Descrição

O programa lê uma linha da entrada padrão (stdin) e espera receber exatamente dois valores separados por espaço:

1. `nome` — nome do cliente
2. `conta` — tipo ou identificação da conta

Se a entrada contiver exatamente dois itens, o programa imprime:

```text
Welcome, <nome>! Your account type is <conta>.
```

Caso contrário, exibe:

```text
Invalid input.
```

## Como executar

Na raiz do crate, rode:

```bash
cargo run
```

Em seguida, digite os dois valores e pressione Enter.

Exemplo de entrada:

```text
David premium
```

Exemplo de saída:

```text
Welcome, David! Your account type is premium.
```

## Compilar

Para compilar o binário sem executar:

```bash
cargo build
```

## Estrutura do projeto

- `Cargo.toml` — configurações do pacote Rust.
- `src/main.rs` — código-fonte do programa.
- `README.md` — documentação deste crate.
