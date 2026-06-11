# Pathless

Projeto de estudo: um protótipo pequeno de ARPG top-down feito em Rust + Bevy.

O objetivo é aprender Bevy com um jogo simples, público e fácil de rodar, ler e modificar: um arquivo de jogo, controles diretos e um loop curto de arena.

## O que tem

- arena 2D top-down
- movimento em 8 direções
- mira com mouse
- ataque básico em arco
- dash curto com cooldown
- waves de inimigos melee
- XP, level up automático e aumento simples de vida/dano
- HUD com vida, wave, kills e XP
- game over com restart

## Controles

- `WASD` ou setas: mover
- mouse: mirar
- `botão esquerdo do mouse` ou `Espaço`: atacar
- `Shift esquerdo`: dash
- `R`: recomeçar depois do game over

## Rodando

Pré-requisito: Rust instalado.

```bash
cargo run
```

Para validar sem abrir a janela:

```bash
cargo check
```

## Estrutura

- `src/main.rs`: jogo inteiro
- `Cargo.toml`: dependências

Sem assets externos e sem geração de código.