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

- `src/main.rs`: bootstrap do Bevy
- `src/game.rs`: estado da run, ordem dos sistemas e restart
- `src/shared.rs`: constantes e helpers compartilhados da arena
- `src/world.rs`: câmera e cenário
- `src/player.rs`: componente e sistemas do jogador
- `src/enemy.rs`: componente e sistemas dos inimigos
- `src/encounter.rs`: waves e spawn de inimigos
- `src/combat.rs`: ataque, mortes e VFX simples
- `src/presentation.rs`: HUD
- `Cargo.toml`: dependências

Sem assets externos e sem geração de código.