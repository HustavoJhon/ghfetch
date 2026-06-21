#!/usr/bin/env bash
set -e

BOLD="\033[1m"
GREEN="\033[38;2;152;151;26m"
YELLOW="\033[38;2;215;153;33m"
AQUA="\033[38;2;104;157;106m"
ORANGE="\033[38;2;214;93;14m"
RESET="\033[0m"

echo -e "${BOLD}${GREEN}  ghfetch installer${RESET}\n"

echo -e "${AQUA}Tu usuario de GitHub:${RESET}"
read -r GITHUB_USER
GITHUB_USER="${GITHUB_USER:-hustavojhon}"

echo -e "\n${AQUA}Token de GitHub (opcional, para mas rate limit):${RESET}"
read -rs GITHUB_TOKEN
echo ""

BIN_DIR="${HOME}/.local/bin"
mkdir -p "${BIN_DIR}"

CONFIG_DIR="${HOME}/.config/ghfetch"
mkdir -p "${CONFIG_DIR}"

if [ -n "$GITHUB_TOKEN" ]; then
    cat > "${CONFIG_DIR}/config.toml" << EOF
username = "${GITHUB_USER}"
github_token = "${GITHUB_TOKEN}"

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
EOF
else
    cat > "${CONFIG_DIR}/config.toml" << EOF
username = "${GITHUB_USER}"

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
EOF
fi

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if [ -f "${SCRIPT_DIR}/ghfetch" ]; then
    cp "${SCRIPT_DIR}/ghfetch" "${BIN_DIR}/ghfetch"
elif [ -f "${SCRIPT_DIR}/target/release/ghfetch" ]; then
    cp "${SCRIPT_DIR}/target/release/ghfetch" "${BIN_DIR}/ghfetch"
elif [ -f "./target/release/ghfetch" ]; then
    cp "./target/release/ghfetch" "${BIN_DIR}/ghfetch"
else
    echo -e "\n${YELLOW}Compilando ghfetch...${RESET}"
    cargo build --release
    cp target/release/ghfetch "${BIN_DIR}/ghfetch"
fi

chmod +x "${BIN_DIR}/ghfetch"

SHELL_RC=""
if [ -f "${HOME}/.zshrc" ]; then
    SHELL_RC="${HOME}/.zshrc"
elif [ -f "${HOME}/.bashrc" ]; then
    SHELL_RC="${HOME}/.bashrc"
fi

if [ -n "$SHELL_RC" ] && ! grep -q "${BIN_DIR}" "$SHELL_RC" 2>/dev/null; then
    echo "export PATH=\"${BIN_DIR}:\$PATH\"" >> "$SHELL_RC"
fi

echo ""
echo -e "${GREEN}${BOLD}  Listo!${RESET}"
echo -e "  Usuario: ${YELLOW}${GITHUB_USER}${RESET}"
echo -e "  Config:  ${CONFIG_DIR}/config.toml"
echo -e "  Binario: ${BIN_DIR}/ghfetch"
echo ""
echo -e "  Ejecuta: ${BOLD}ghfetch${RESET}"
echo ""
