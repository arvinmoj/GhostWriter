<div align="center">

# GhostWriter

**No more copy-pasting to browsers -- just type, trigger, and transform.**

![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![License](https://img.shields.io/badge/license-MIT-blue)
![Tauri](https://img.shields.io/badge/built%20with-Tauri-ffc131)

---

</div>

## What is GhostWriter?

GhostWriter is a **background utility** that lets you refine, rewrite, or translate text **anywhere** on your computer -- emails, code editors, chat apps, forms, documents -- without ever leaving the app you're working in.

Highlight text, press a hotkey, and AI-powered instructions transform it on the spot.

## How It Works

| Step | Action |
|------|--------|
| **1** | Select any text in any application |
| **2** | Press `Cmd + Shift + R` (macOS) / `Ctrl + Shift + R` (Windows/Linux) |
| **3** | GhostWriter reads your selection and sends it to an AI model with your custom instructions |
| **4** | The refined text replaces your original selection -- instantly |

That's it. No switching windows. No copy-paste. No interruption.

## Features

| Feature | Description |
|---------|-------------|
| **System-Wide** | Works in every text field across your OS |
| **Custom Instructions** | Drop a `.md` file with your prompt -- switch modes by swapping files |
| **Multi-Model** | Powered by OpenRouter -- use GPT-4, Claude, Llama, and more |
| **Zero UI** | All interaction happens through keyboard shortcuts |
| **Privacy-First** | Your API key stays encrypted locally. No data stored. |
| **Lightweight** | Built with Tauri + Rust -- small footprint, low memory usage |

### Example Instruction Files

| File | What it does |
|------|-------------|
| `grammar.md` | "Fix grammar and spelling while preserving tone." |
| `translate.md` | "Translate this to French. Keep technical terms in English." |
| `friendly.md` | "Rewrite this in a warm, conversational tone." |
| `concise.md` | "Shorten this to half the length without losing key points." |

## Installation

### macOS

1. Download the latest `.dmg` from [Releases](https://github.com/arvinmoj/GhostWriter/releases)
2. Drag GhostWriter to Applications
3. Open **System Settings -> Privacy & Security -> Accessibility** and grant permission
4. Launch GhostWriter -- it lives in your menu bar

### Windows

1. Download the latest `.msi` from [Releases](https://github.com/arvinmoj/GhostWriter/releases)
2. Run the installer
3. Launch GhostWriter -- it lives in your system tray

### Linux

1. Download the latest `.AppImage` from [Releases](https://github.com/arvinmoj/GhostWriter/releases)
2. `chmod +x GhostWriter-*.AppImage && ./GhostWriter-*.AppImage`
3. It will run in your system tray

## Setup

1. **Create an instruction file** -- for example, `~/ghost-instructions.md`:
   ```markdown
   Fix grammar and spelling mistakes. Improve clarity. Keep the original tone.
   ```

2. **Get an API key** -- sign up at [OpenRouter.ai](https://openrouter.ai) (one key gives you access to GPT-4, Claude, Llama, and more)

3. **Configure** -- on first launch, GhostWriter will ask for:
   - Your API key (stored encrypted)
   - Path to your instruction file
   - Preferred model
   - Hotkey combination

4. **Go** -- highlight text in any app and press your hotkey

## Instruction Files (The "Engine")

Your instruction file is a plain Markdown (`.md`) file that tells the AI **how** to process your text. You can create as many as you want and switch between them.

**Example -- `grammar.md`:**
```markdown
You are a professional editor. Correct grammar, spelling, and punctuation.
Preserve the original tone and style. Do not add or remove information.
```

**Example -- `code-review.md`:**
```markdown
You are a senior software engineer. Review the code for bugs, performance
issues, and security vulnerabilities. Suggest improvements concisely.
```

> **Pro tip:** Keep multiple instruction files handy and rename the active one, or configure quick-switching in settings.
