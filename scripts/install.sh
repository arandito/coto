#!/bin/sh
set -e

# ANSI color codes and special formatting
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
YELLOW='\033[1;33m'
PURPLE='\033[0;35m'
ORANGE='\033[38;5;208m'
BOLD='\033[1m'
DIM='\033[2m'
ITALIC='\033[3m'
UNDERLINE='\033[4m'
BLINK='\033[5m'
REVERSE='\033[7m'
RESET='\033[0m'

# GitHub repo information
REPO="arandito/coto"
BINARY_NAME="coto"

# Check if running with root privileges (sudo)
if [ "$(id -u)" = "0" ]; then
    SUDO=""
else
    SUDO="sudo"
fi

# Print fancy ASCII art logo
print_logo() {
    echo ""
    echo "${MAGENTA}  ██████╗ ██████╗ ████████╗ ██████╗ ${RESET}"
    echo "${MAGENTA} ██╔════╝██╔═══██╗╚══██╔══╝██╔═══██╗${RESET}"
    echo "${MAGENTA} ██║     ██║   ██║   ██║   ██║   ██║${RESET}"
    echo "${MAGENTA} ██║     ██║   ██║   ██║   ██║   ██║${RESET}"
    echo "${MAGENTA} ╚██████╗╚██████╔╝   ██║   ╚██████╔╝${RESET}"
    echo "${MAGENTA}  ╚═════╝ ╚═════╝    ╚═╝    ╚═════╝ ${RESET}"
    echo ""
    echo "${MAGENTA}${BOLD}⚡ ${RESET}${ITALIC}${BOLD}Unleash the power of boto3${RESET}${MAGENTA}${BOLD} ⚡${RESET}"
    echo ""
}

# Animated progress spinner
spinner() {
    local pid=$1
    local delay=0.1
    local spinstr='|/-\'
    while [ "$(ps a | awk '{print $1}' | grep $pid)" ]; do
        local temp=${spinstr#?}
        printf " [%c]  " "$spinstr"
        local spinstr=$temp${spinstr%"$temp"}
        sleep $delay
        printf "\b\b\b\b\b\b"
    done
    printf "    \b\b\b\b"
}

# Display progress bar
progress_bar() {
    local duration=$1
    local prefix=$2
    local barsize=40
    local barchar="▓"
    local emptychar="░"
    
    local i
    for i in $(seq 1 $barsize); do
        sleep $(echo "$duration/$barsize" | bc -l)
        local progressbar=""
        local j=0
        while [ $j -lt $i ]; do
            progressbar="${progressbar}${barchar}"
            j=$((j+1))
        done
        while [ $j -lt $barsize ]; do
            progressbar="${progressbar}${emptychar}"
            j=$((j+1))
        done
        
        percent=$((i*100/barsize))
        printf "\r${prefix} [${BLUE}${progressbar}${RESET}] ${BOLD}${percent}%%${RESET}"
    done
    printf "\n"
}

# Print error message and exit
error() {
    echo "${RED}${BOLD}✗ ERROR:${RESET}${RED} $1${RESET}" >&2
    exit 1
}

# Print success message
success_step() {
    echo "${GREEN}${BOLD}✓${RESET} ${GREEN}$1${RESET}"
}

# Print info message
info() {
    echo "${BLUE}${BOLD}ℹ${RESET} ${BLUE}$1${RESET}"
}

# Print warning message
warning() {
    echo "${YELLOW}${BOLD}⚠${RESET} ${YELLOW}$1${RESET}"
}

# Detect operating system and architecture
detect_os_arch() {
    info "Detecting system configuration..."
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    ARCH="$(uname -m)"
    
    case "$OS" in
        darwin)
            OS="apple-darwin"
            OS_NAME="macOS"
            ;;
        linux)
            OS="unknown-linux-gnu"
            OS_NAME="Linux"
            ;;
        *)
            error "Unsupported operating system: $OS"
            ;;
    esac

    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ARCH_NAME="64-bit Intel/AMD"
            ;;
        arm64|aarch64)
            ARCH="aarch64"
            ARCH_NAME="64-bit ARM"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac

    TARGET="$ARCH-$OS"
    success_step "Detected ${BOLD}$OS_NAME${RESET} on ${BOLD}$ARCH_NAME${RESET} architecture"
}

# Get the latest release version
get_latest_version() {
    info "Fetching the latest release..."
    
    VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    
    if [ -z "$VERSION" ]; then
        error "Failed to fetch the latest version. Please check your internet connection."
    fi
    
    success_step "Found latest version: ${BOLD}${VERSION}${RESET}"
}

# Download the binary
download_binary() {
    info "Preparing to download $BINARY_NAME..."
    TEMP_DIR=$(mktemp -d)
    ARCHIVE="$BINARY_NAME-$TARGET.tar.gz"
    URL="https://github.com/$REPO/releases/download/$VERSION/$ARCHIVE"
    
    echo "${CYAN}Downloading ${BOLD}$BINARY_NAME${RESET}${CYAN} for ${BOLD}$TARGET${RESET}"
    printf "${DIM}URL: $URL${RESET}\n"
    
    # Start download with progress bar
    curl -L --progress-bar -o "$TEMP_DIR/$ARCHIVE" "$URL" || error "Failed to download $ARCHIVE"
    
    success_step "Download complete!"
    
    info "Extracting archive..."
    progress_bar 1 "Extracting"
    tar -xzf "$TEMP_DIR/$ARCHIVE" -C "$TEMP_DIR" || error "Failed to extract archive"
    
    # Create bin directory if it doesn't exist
    mkdir -p "$HOME/.local/bin"
    
    info "Installing $BINARY_NAME..."
    # Move the binary to the installation directory
    if mv "$TEMP_DIR/$BINARY_NAME" "$HOME/.local/bin/$BINARY_NAME" 2>/dev/null; then
        INSTALL_PATH="$HOME/.local/bin/$BINARY_NAME"
    else
        printf "${YELLOW}Attempting to install system-wide (requires sudo)${RESET}\n"
        $SUDO mv "$TEMP_DIR/$BINARY_NAME" "/usr/local/bin/$BINARY_NAME" || error "Failed to install binary"
        INSTALL_PATH="/usr/local/bin/$BINARY_NAME"
    fi
    
    # Make the binary executable
    if [ "$INSTALL_PATH" = "$HOME/.local/bin/$BINARY_NAME" ]; then
        chmod +x "$INSTALL_PATH" || error "Failed to make binary executable"
    else
        $SUDO chmod +x "$INSTALL_PATH" || error "Failed to make binary executable"
    fi
    
    # Clean up
    rm -rf "$TEMP_DIR"
    success_step "Installation complete at ${BOLD}$INSTALL_PATH${RESET}"
}

# Update PATH if needed
update_path() {
    if [ -d "$HOME/.local/bin" ] && ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
        info "Updating your PATH configuration..."
        
        SHELL_NAME=$(basename "$SHELL")
        case "$SHELL_NAME" in
            bash)
                PROFILE="$HOME/.bashrc"
                ;;
            zsh)
                PROFILE="$HOME/.zshrc"
                ;;
            *)
                PROFILE="$HOME/.profile"
                ;;
        esac
        
        echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> "$PROFILE"
        success_step "Added $HOME/.local/bin to PATH in $PROFILE"
        warning "Please restart your shell or run: ${BOLD}source $PROFILE${RESET}"
    fi
}

# Display success message
display_success() {
    echo ""
    echo "${GREEN}${BOLD}╔═════════════════════════════════════════════╗${RESET}"
    echo "${GREEN}${BOLD}║                                             ║${RESET}"
    echo "${GREEN}${BOLD}║       🚀 INSTALLATION SUCCESSFUL! 🚀        ║${RESET}"
    echo "${GREEN}${BOLD}║                                             ║${RESET}"
    echo "${GREEN}${BOLD}╚═════════════════════════════════════════════╝${RESET}"
    echo ""
    echo "${PURPLE}${BOLD}✨ ${RESET}${CYAN}${BINARY_NAME} ${VERSION}${RESET}${PURPLE}${BOLD} ✨${RESET} has been installed on your system!"
    echo ""
    
    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        echo "${ORANGE}${BOLD}┌─ NEXT STEP ─────────────────────────────────┐${RESET}"
        echo "${ORANGE}${BOLD}│${RESET}                                             ${ORANGE}${BOLD}│${RESET}"
        echo "${ORANGE}${BOLD}│${RESET}  ${REVERSE}${BOLD} IMPORTANT ${RESET} First, run: ${CYAN}${BOLD}$BINARY_NAME setup${RESET}         ${ORANGE}${BOLD}│${RESET}"
        echo "${ORANGE}${BOLD}│${RESET}                                             ${ORANGE}${BOLD}│${RESET}"
        echo "${ORANGE}${BOLD}└─────────────────────────────────────────────┘${RESET}"
        echo ""
        echo "After setup is complete, you can use $BINARY_NAME commands."
        echo "Try: ${CYAN}${BOLD}$BINARY_NAME --help${RESET} to see available options."
    else
        warning "You may need to add the installation directory to your PATH or restart your terminal."
    fi
    
    echo ""
    echo "┌─ ${BOLD}RESOURCES${RESET} ─────────────────────────────────┐"
    echo "│                                             │"
    echo "│  Documentation and Support:${RESET}                 │"
    echo "│  ${BLUE}https://github.com/$REPO${RESET}           │"
    echo "│                                             │"
    echo "└─────────────────────────────────────────────┘"
    echo ""
    echo "${PURPLE}${BOLD}✨ ${RESET}${DIM}Thank you for installing $BINARY_NAME!${RESET}${PURPLE}${BOLD} ✨${RESET}"
    echo ""
}

# Check terminal capabilities
check_terminal() {
    # Check if the terminal supports colors
    if [ -t 1 ] && command -v tput >/dev/null 2>&1 && [ "$(tput colors 2>/dev/null || echo 0)" -ge 8 ]; then
        HAS_COLOR=true
    else
        # Disable colors
        RED=''
        GREEN=''
        BLUE=''
        CYAN=''
        YELLOW=''
        BOLD=''
        DIM=''
        ITALIC=''
        UNDERLINE=''
        BLINK=''
        RESET=''
        
        warning "Your terminal doesn't support colors, continuing with plain output."
    fi
}

main() {
    check_terminal
    print_logo
    
    echo "${BOLD}Welcome to the ${CYAN}$BINARY_NAME${RESET}${BOLD} installer!${RESET}"
    echo "This script will install the latest version of ${CYAN}$BINARY_NAME${RESET} on your system."
    echo ""
    
    detect_os_arch
    get_latest_version
    download_binary
    update_path
    display_success
}

main
