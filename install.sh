#!/bin/bash

# Install Homebrew
echo "Installing Homebrew..."
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Add Homebrew to PATH (for Apple Silicon Macs)
if [ -d "/opt/homebrew/bin" ]; then
    echo 'eval "$(/opt/homebrew/bin/brew shellenv)"' >> ~/.zprofile
    eval "$(/opt/homebrew/bin/brew shellenv)"
fi

# For Intel Macs
if [ -d "/usr/local/bin" ]; then
    echo 'eval "$(/usr/local/bin/brew shellenv)"' >> ~/.zprofile
    eval "$(/usr/local/bin/brew shellenv)"
fi

# Install Docker Desktop
echo "Installing Docker Desktop..."
brew install --cask docker

echo ""
echo "✅ Installation complete!"
echo ""
echo "Next steps:"
echo "1. Open Docker Desktop from Applications"
echo "2. Wait for Docker to start (whale icon in menu bar)"
echo "3. Run: docker-compose up -d"

