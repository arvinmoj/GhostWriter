<div align="center">

# GhostWriter

**No more copy-pasting to browsers — just type, trigger, and transform.**

![Version](https://img.shields.io/github/v/release/arvinmoj/GhostWriter)
![License](https://img.shields.io/github/license/arvinmoj/GhostWriter)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)
</div>

## ✨ Highlights

- **System-wide** — Works in every app: email, code editors, chat, documents
- **Zero UI** — Pure keyboard shortcuts, no context switching
- **Custom instructions** — Drop a `.md` file to define how AI transforms your text
- **Multi-provider** — GPT-4, Claude, Llama via OpenRouter; Big Pickle (free) via OpenCode Zen
- **Privacy-first** — Your API key stays encrypted locally, zero telemetry
- **Lightweight** — Built with Tauri + Rust, minimal memory footprint

## Table of Contents

- [About GhostWriter](#about-ghostwriter)
- [Quick Start](#quick-start)
- [Features](#features)
- [Screenshots](#screenshots)
- [Usage Examples](#usage-examples)
- [Configuration](#configuration)
- [Building from Source](#building-from-source)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## About GhostWriter

GhostWriter is a **background utility** that lets you refine, rewrite, or translate text **anywhere** on your computer — without ever leaving the app you're working in.

Highlight text, press a hotkey, and AI-powered instructions transform it on the spot. No browser tabs. No copy-paste. No interruption.

Whether you're fixing grammar in an email, translating code comments, or making your writing more concise, GhostWriter puts AI-powered text transformation at your fingertips — exactly where you need it.

## 🚀 Quick Start

### Installation

**macOS:**
```bash
brew tap arvinmoj/ghostwriter
brew install --cask ghostwriter
```

**Windows:**
Download the `.msi` installer from [Releases](https://github.com/arvinmoj/GhostWriter/releases)

**Linux:**
```bash
chmod +x GhostWriter-*.AppImage && ./GhostWriter-*.AppImage
```

### Setup (30 seconds)

**Option 1 — OpenRouter** (GPT-4, Claude, Llama, etc.):
1. **Get an API key** → [OpenRouter.ai](https://openrouter.ai) (one key for 200+ models)
2. **Launch GhostWriter** → Configure your API key, instruction file path, and hotkey
3. **Go** → Highlight text anywhere and press your hotkey

**Option 2 — OpenCode Zen** (Big Pickle is **free**):
1. **Get an API key** → [opencode.ai/zen](https://opencode.ai/zen) (free tier available)
2. **Configure** — set `api_base_url` and `model` in your config (see Configuration section)
3. **Launch GhostWriter** and start transforming text instantly — no API costs

Create an instruction file `~/ghost-instructions.md`:
```markdown
Fix grammar and spelling mistakes. Improve clarity. Keep the original tone.
```

### Usage

| Step | Action |
|------|--------|
| **1** | Select any text in any application |
| **2** | Press `Cmd + Shift + R` (macOS) / `Ctrl + Shift + R` (Windows/Linux) |
| **3** | GhostWriter reads your selection and sends it to AI with your instructions |
| **4** | Transformed text replaces your original selection — instantly |

## 🎯 Features

### System-Wide Text Transformation
Works in **every text field** across your OS — email clients, IDEs, browsers, chat apps, documents, terminal editors.

### Custom Instruction Files
Your instruction file is a plain Markdown (`.md`) file that tells the AI **how** to process your text. Create as many as you want and switch between them.

| File | What it does |
|------|-------------|
| `grammar.md` | "Fix grammar and spelling while preserving tone." |
| `translate.md` | "Translate this to French. Keep technical terms in English." |
| `friendly.md` | "Rewrite this in a warm, conversational tone." |
| `concise.md` | "Shorten this to half the length without losing key points." |
| `code-review.md` | "Review for bugs, performance issues, and security vulnerabilities." |

### Multi-Provider Support

**OpenRouter** — 200+ models, one API key:
- OpenAI: GPT-4, GPT-5 series
- Anthropic: Claude Opus 4, Sonnet 4
- Google: Gemini 3 Pro, Gemini 3 Flash
- Meta: Llama 3, 4
- And many more...

**OpenCode Zen** — Curated provider with free models:
- [Big Pickle](https://opencode.ai/docs/zen/) — **Free** (limited time), 200K context
- DeepSeek V4 Flash Free — Free
- MiMo-V2.5 Free, Nemotron 3 Ultra Free
- Plus paid models: Claude, GPT, Gemini, Qwen, and more

## 💡 Usage Examples

### Improving Writing Quality
Highlight a paragraph in your document and press your hotkey to instantly:
- Fix grammar and spelling
- Improve sentence structure
- Enhance vocabulary while maintaining your voice

### Code Documentation
Select technical comments in your IDE and transform them to:
- Be more clear and concise
- Follow documentation standards
- Explain complex logic in simple terms

### Communication Enhancement
Before sending a message in any chat app:
- Make your tone more professional or friendly
- Ensure your message is clear and concise
- Translate to another language while preserving meaning

## ⚙️ Configuration

On first launch, GhostWriter will ask for:

| Setting | Description | Default |
|---------|-------------|---------|
| **API Key** | Your API key (OpenRouter or OpenCode Zen) | Required |
| **Instruction File** | Path to your `.md` prompt file | `~/ghost-instructions.md` |
| **Model** | Preferred AI model | `openai/gpt-4o-mini` |
| **Hotkey** | Keyboard shortcut | `Cmd/Ctrl + Shift + R` |
| **API Base URL** | Custom API endpoint for OpenAI-compatible providers | OpenRouter default |
| **Proxy URL** | Optional proxy for API calls | None |

### Configuration File Template

Create the config file in the appropriate location for your OS:

<details>
<summary><b>Linux</b> — <code>~/.config/ghostwriter/config.json</code></summary>

```json
{
  "api_key_encrypted": "",
  "model": "openai/gpt-4o-mini",
  "instruction_file": "/Users/$(whoami)/.config/ghostwriter/instructions/default.md",
  "hotkey": {
    "modifiers": ["cmd"],
    "key": "r"
  },
  "proxy_url": null,
  "api_base_url": null,
  "first_run": false
}
```
</details>

<details>
<summary><b>Windows</b> — <code>%APPDATA%\ghostwriter\config.json</code></summary>

```json
{
  "api_key_encrypted": "",
  "model": "openai/gpt-4o-mini",
  "instruction_file": "%APPDATA%\\ghostwriter\\instructions\\default.md",
  "hotkey": {
    "modifiers": ["ctrl"],
    "key": "r"
  },
  "proxy_url": null,
  "api_base_url": null,
  "first_run": true
}
```
</details>

<details>
<summary><b>macOS</b> — <code>~/.config/ghostwriter/config.json</code></summary>

```json
{
  "api_key_encrypted": "",
  "model": "openai/gpt-4o-mini",
  "instruction_file": "$HOME/.config/ghostwriter/instructions/default.md",
  "hotkey": {
    "modifiers": ["cmd"],
    "key": "r"
  },
  "proxy_url": null,
  "api_base_url": null,
  "first_run": false
}
```
</details>

## 📦 Building from Source

<details>
<summary>Click to expand build instructions</summary>

### Prerequisites
- [Rust](https://rustup.rs) (latest stable)
- [Node.js](https://nodejs.org) (v18+)
- Platform-specific dependencies (see [Tauri docs](https://v2.tauri.app/start/prerequisites/))

### Build
```bash
git clone https://github.com/arvinmoj/GhostWriter.git
cd GhostWriter
npm install
npm run tauri build
```

The compiled binary will be in `src-tauri/target/release/`.

### Development
```bash
npm run tauri dev
```

</details>

## 🤝 Contributing

Contributions are welcome! Whether it's:
- 🐛 Bug reports
- 💡 Feature suggestions
- 📝 Documentation improvements
- 🔧 Code contributions

We welcome contributions from everyone. Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## 📄 License

[MIT](LICENSE) — (c) Arvin

## 🙏 Acknowledgments

- [Rust](https://www.rust-lang.org) — For performance and safety
- [Tauri](https://tauri.app) — For the amazing cross-platform framework
- [OpenRouter](https://openrouter.ai) — For unified AI model access
- [OpenCode Zen](https://opencode.ai/docs/zen/) — For free AI model access (Big Pickle)

---

<div align="center">

**⭐ If GhostWriter saves you time, consider starring the repo!**

[Report Bug](https://github.com/arvinmoj/GhostWriter/issues) · [Request Feature](https://github.com/arvinmoj/GhostWriter/issues) · [Download](https://github.com/arvinmoj/GhostWriter/releases)

</div>