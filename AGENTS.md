# AGENTS.md

## Objetivo do repositório

`Pathless` é um protótipo pequeno de ARPG top-down em Bevy.
O foco é jogabilidade simples, leitura fácil do código e progressão curta.

## Regras de escopo

- Manter o projeto pequeno e jogável.
- Preferir mecânicas claras a sistemas complexos.
- Não transformar o protótipo em MMO, roguelite gigante ou clone de PoE.
- Toda mudança nova deve preservar a facilidade de rodar e entender o projeto.

## Stack

- Rust
- Bevy 0.18

## Comandos principais

```bash
cargo run
cargo check
cargo fmt
```

## Estrutura atual

- `src/main.rs`: protótipo inteiro atual
- `README.md`: visão geral e controles

## Convenções de implementação

- Preferir código simples antes de abstrações.
- Se um sistema ainda é pequeno, manter junto em vez de fragmentar em muitos módulos.
- Quando o arquivo crescer demais, quebrar por domínio:
  - `player`
  - `enemy`
  - `combat`
  - `progression`
  - `ui`
  - `state`
- Reusar constantes para números centrais de balanceamento.
- Qualquer nova mecânica deve ter feedback visível em jogo.

## Prioridades de design

1. game feel
2. clareza da progressão
3. legibilidade do código
4. expansão futura sem complicar o presente

## Mudanças aceitáveis agora

- novos inimigos simples
- mais upgrades
- skill secundária
- boss
- salas ou waves
- menu inicial

## Mudanças a evitar por enquanto

- multiplayer
- inventário complexo
- árvore de skills grande
- crafting
- backend
- economia
- dezenas de telas e sistemas paralelos

## Verificação mínima antes de concluir uma mudança

```bash
cargo check
```

Se mexer em comportamento central, também rodar o jogo localmente com `cargo run` e validar os controles afetados.
