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
- XP com escolhas de upgrade estilo Vampire Survivors
- barra de vida nos inimigos
- efeitos sonoros simples de ataque, hit e level up
- HUD com vida, wave, kills, XP, upgrade e dash
- game over com restart

## Controles

- `WASD` ou setas: mover
- mouse: mirar
- `botão esquerdo do mouse` ou `Espaço`: atacar
- `Shift esquerdo`: dash
- `R`: recomeçar depois do game over

## Rodando

Pré-requisito: Rust instalado.

Clone e rode pela raiz do projeto:

```bash
git clone https://github.com/Yorizel/pathless.git
cd pathless
cargo run
```

Na primeira vez o Rust baixa e compila o Bevy, então pode demorar alguns minutos antes da janela abrir.

Se quiser só validar que o projeto compila:

```bash
cargo check
```

Se `cargo run` terminar sem abrir janela, rode:

```bash
cargo run --release
```

Se ainda não abrir, copie a saída completa do terminal. Em Linux, Bevy também precisa de driver gráfico/Vulkan funcionando.

## Estrutura

- `src/main.rs`: bootstrap do Bevy
- `src/game.rs`: estado da run, ordem dos sistemas e restart
- `src/shared.rs`: constantes e helpers compartilhados da arena
- `src/world.rs`: câmera e cenário
- `src/player.rs`: componente e sistemas do jogador
- `src/enemy.rs`: componente e sistemas dos inimigos
- `src/encounter.rs`: waves e spawn de inimigos
- `src/combat.rs`: ataque, mortes e VFX simples
- `src/sfx.rs`: carregamento e disparo dos efeitos sonoros
- `src/presentation.rs`: HUD
- `assets/sounds/`: efeitos sonoros OGG
- `Cargo.toml`: dependências

Assets sonoros ficam em OGG/Vorbis para usar o decoder padrão do Bevy.