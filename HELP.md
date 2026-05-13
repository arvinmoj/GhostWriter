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

## Step 3: Create Instruction Files

Instruction files tell GhostWriter how to process your text. They're simple Markdown files stored in `~/.config/ghostwriter/instructions/`.

### Default Instruction File
GhostWriter automatically creates `~/.config/ghostwriter/instructions/default.md` with:
```
You are a helpful AI assistant. Improve the user's text for clarity, grammar, and style while preserving the original meaning and tone.
```

### Custom Instruction Examples

**Grammar Fix (`grammar.md`):**
```
You are a professional editor. Correct all grammar, spelling, and punctuation errors.
Preserve the original tone and style. Do not add or remove information.
```

**Translation to French (`translate_french.md`):**
```
Translate the following text to French.
Keep technical terms and proper nouns in their original English form.
Maintain the original tone and formatting.
```

**Friendly Tone (`friendly.md`):**
```
Rewrite the following text in a warm, friendly, and conversational tone.
Make it feel natural and approachable while preserving the core message.
```

**Code Review (`code_review.md`):**
```
You are a senior software engineer. Review the code for:
- Bugs and logic errors
- Performance issues
- Security vulnerabilities
- Code style and best practices
Provide specific, actionable suggestions for improvement.
```

**Concise Summary (`concise.md`):**
```
Shorten the following text to half its length without losing essential information.
Be direct and concise. Remove filler words and redundant phrases.
```

To use a custom instruction file, update your config:
```json
{
  "instruction_file": "/Users/yourname/.config/ghostwriter/instructions/grammar.md"
}
```