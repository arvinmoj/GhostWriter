# GhostWriter - Help & Usage Guide

## Getting Started

GhostWriter is a system-wide AI text refiner that works in any application. Once configured, simply select text and press your hotkey to transform it using AI.

## Step 1: Get an OpenRouter API Key

1. Visit [https://openrouter.ai](https://openrouter.ai)
2. Sign up for a free account
3. Navigate to **Account → API Keys**
4. Create a new API key (it will start with `sk-or-...`)
5. Copy the key - you'll need it for the next step

## Step 2: Configure GhostWriter

### Option A: Interactive Setup (Recommended)
1. Launch GhostWriter (it will appear in your menu bar)
2. Select any text in any application
3. Press `Cmd+Shift+R` (macOS) / `Ctrl+Shift+R` (Windows/Linux)
4. A prompt will appear asking for your OpenRouter API key
5. Enter your key and press Enter
6. The app will save it securely and restart

### Option B: Manual Configuration
Create the config file manually:

```bash
mkdir -p ~/.config/ghostwriter
cat > ~/.config/ghostwriter/config.json << EOF
{
  "api_key_encrypted": "",
  "model": "openai/gpt-4o-mini",
  "instruction_file": "/Users/$(whoami)/.config/ghostwriter/instructions/default.md",
  "hotkey": {
    "modifiers": ["cmd"],
    "key": "r"
  },
  "first_run": false
}
EOF
```

Then restart GhostWriter - it will prompt for your API key.