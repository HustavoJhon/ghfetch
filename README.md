# ghfetch

<p align="center">
  <img src="https://img.shields.io/badge/rust-1.95+-orange.svg" alt="Rust">
  <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/theme-Gruvbox-green.svg" alt="Theme">
</p>

**ghfetch** es una herramienta CLI inspirada en Neofetch y Fastfetch que muestra tu perfil de GitHub junto con informacion del sistema, todo con una estetica retro-terminal en tema **Gruvbox**.

Muestra tu avatar en ASCII, estadisticas del sistema, y un grafico de contribuciones real — todo en una sola pantalla responsive.

```
   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░     👻 OS  ·············· Arch Linux
   ░░▒▒▓▓▓▓▓▓▓▒▒░░░░░░░░░░░░░░░     👻 Kernel ········· 6.12.10-arch1-1
   ░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒░░░░░░░░░     👻 CPU ············ AMD Ryzen 5 3600
   ░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░░░░░░░     👻 RAM ············ 8.2 GiB / 16 GiB
   ░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░░░░░░     👻 Disk ··········· 120 GiB / 445 GiB
   ░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒░░░░░░░     👻 Shell ·········· zsh
   ░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▒▒░░░░░░░     👻 Terminal ······· kitty
   ░░░░░▒▓▓▓▓▓▓▓▓▓▓▓▓▒▒░░░░░░░░     👻 Res ············ 1920x1080
   ░░░░░░░░░▒▒▓▓▓▒▒▒░░░░░░░░░░░░     👻 Uptime ········· 3h 42m
   ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░     👻 Arch ··········· x86_64

                                      73 contribuciones  Menos █ █ █ █  Mas

                                        ░░░░░░░░░░░░░░░▒░░░░░░░░░▒░░░░▓▓░░
                                        ░░░░░░░░░░░░░▒░░░░░░░░░▒▓░░░░░▓▓░░
                                        ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▓▓░░
                                        ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░▒░

   fetched in 110ms
```

---

## Caracteristicas

- **Avatar en ASCII** — Descarga tu foto de GitHub y la convierte en arte ASCII con bloques de color Gruvbox
- **Deteccion automatica** — Lee tu usuario desde `git config github.user` o `gh auth status`
- **Info del sistema** — SO, kernel, CPU, GPU, RAM, disco, shell, terminal, resolucion, uptime, arquitectura
- **Grafico de contribuciones** — Calendario ASCII del ano actual con datos reales de GitHub
- **Layout responsive** — Avatar a la izquierda, info y contribuciones apiladas a la derecha
- **Tema Gruvbox Dark** — Paleta de colores cohesiva con iconos fantasma por categoria
- **Rapido** — ~100ms cacheado, ~800ms primera ejecucion
- **Multiplataforma** — Linux, macOS, Windows
- **Configurable** — Archivo TOML con todas las opciones

---

## Instalacion

### Instalador automatico (recomendado)

```bash
git clone https://github.com/hustavojhon/ghfetch.git
cd ghfetch
bash install.sh
```

El instalador detecta tu usuario de GitHub automaticamente, te pide un token (opcional) y configura todo.

### Desde codigo fuente

```bash
git clone https://github.com/hustavojhon/ghfetch.git
cd ghfetch
cargo build --release
cp target/release/ghfetch ~/.local/bin/
```

### Requisitos

- Rust 1.95+
- Git (para deteccion automatica de usuario)

---

## Uso

```bash
ghfetch                          # Tu perfil (detectado automaticamente)
ghfetch -u torvalds              # Perfil de otro usuario
ghfetch -w 50                    # Avatar mas grande
ghfetch --no-avatar              # Solo info del sistema
ghfetch --no-contributions       # Ocultar grafico
ghfetch --nerd-font-icons        # Iconos Nerd Font en vez de emojis
```

### Opciones

| Flag | Corta | Descripcion |
|------|-------|-------------|
| `--username` | `-u` | Usuario de GitHub (auto-detectado si no se especifica) |
| `--config` | `-c` | Ruta al archivo de configuracion |
| `--no-avatar` | | Oculta el avatar ASCII |
| `--no-system` | | Oculta info del sistema |
| `--no-contributions` | | Oculta grafico de contribuciones |
| `--avatar-width` | `-w` | Ancho del avatar en caracteres |
| `--nerd-font-icons` | | Usa glifos Nerd Font |

---

## Deteccion de usuario

ghfetch intenta detectar tu usuario de GitHub en este orden:

1. Flag `-u` (maxima prioridad)
2. Variable de entorno `GHFETCH_USERNAME`
3. `git config --global github.user`
4. `gh auth status` (GitHub CLI)
5. Archivo de configuracion `~/.config/ghfetch/config.toml`

Si nada funciona, ghfetch te pide que configures uno.

Para guardar tu usuario permanentemente:

```bash
git config --global github.user TU_USUARIO
```

---

## Configuracion

Archivo `~/.config/ghfetch/config.toml`:

```toml
username = ""

[display]
show_avatar = true
show_system_info = true
show_contributions = true
nerd_font_icons = false
max_contrib_weeks = 53

[cache]
avatar_ttl_hours = 24
profile_ttl_hours = 1
contributions_ttl_hours = 2

[icons]
ghost = "👻"

[icons.overrides]
# os = "󰻀"
# cpu = "󰻠"
# ram = "󰘚"
```

### Variables de entorno

| Variable | Proposito |
|----------|-----------|
| `GITHUB_TOKEN` | Token de acceso personal (5000 req/h) |
| `GHFETCH_USERNAME` | Usuario por defecto |
| `GHFETCH_CONFIG` | Ruta al archivo de configuracion |

---

## Arquitectura

```
src/
├── main.rs           # Punto de entrada, CLI, orquestacion
├── theme.rs          # Paleta Gruvbox, utilidades ANSI, iconos
├── config.rs         # Carga de archivo TOML, defaults
├── cache.rs          # Cache en disco con TTL por clave
├── system.rs         # Informacion del sistema multiplataforma
├── github.rs         # Cliente REST + GraphQL para GitHub
├── avatar.rs         # Descarga, resize y conversion a ASCII
├── greetings.rs      # Saludo dinamico (desactivado por defecto)
├── contributions.rs  # Renderizado del grafico de contribuciones
└── renderer.rs       # Motor de layout responsive
```

### Dependencias principales

| Crate | Uso |
|-------|-----|
| `clap` | Parseo de argumentos CLI |
| `toml` + `serde` | Archivos de configuracion |
| `ureq` | Cliente HTTP para API de GitHub |
| `image` | Procesamiento de avatar |
| `sysinfo` | Informacion del sistema |
| `chrono` | Fechas y horas |
| `terminal_size` | Deteccion de tamano de terminal |
| `dirs` | Directorios XDG |

---

## Rendimiento

- **Primera ejecucion**: ~800ms (descarga avatar + API)
- **Ejecucion cacheada**: ~110ms
- **Binario**: ~3.6 MB

Estrategia de cache:
- Avatar: 24h
- Perfil: 1h
- Contribuciones: 2h
- Directorio: `~/.cache/ghfetch/`

---

## Roadmap

- [ ] Temas adicionales (Nord, Catppuccin, Tokyo Night)
- [ ] Modo `--watch` con actualizacion en vivo
- [ ] Salida JSON para scripting
- [ ] Soporte de imagenes inline (iTerm2/Kitty)
- [ ] Paquetes para AUR, Homebrew, Scoop
- [ ] Estadisticas de lenguajes desde GitHub

---

## Contribuir

1. Hace un fork
2. Crea una rama (`git checkout -b feature/nueva-cosa`)
3. Hace tus cambios
4. Ejecuta `cargo build` y `cargo clippy`
5. Envia un pull request

---

## Licencia

MIT © [Gustavo Jhon](https://github.com/hustavojhon)
