<div align="center">

# GhostWriter

**No more copy-pasting to browsers — just type, trigger, and transform.**

![Version](https://img.shields.io/github/v/release/arvinmoj/GhostWriter)
![License](https://img.shields.io/github/license/arvinmoj/GhostWriter)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)

## ✨ Highlights

- **System-wide** — Works in every app: email, code editors, chat, documents
- **Zero UI** — Pure keyboard shortcuts, no context switching
- **Custom instructions** — Drop a `.md` file to define how AI transforms your text
- **Multi-model** — GPT-4, Claude, Llama, and more via OpenRouter
- **Privacy-first** — Your API key stays encrypted locally, zero telemetry
- **Lightweight** — Built with Tauri + Rust, minimal memory footprint

---

## 🎯 What is GhostWriter?

GhostWriter is a **background utility** that lets you refine, rewrite, or translate text **anywhere** on your computer — without ever leaving the app you're working in.

Highlight text, press a hotkey, and AI-powered instructions transform it on the spot. No browser tabs. No copy-paste. No interruption.

---

## 🚀 Quick Start

### Installation

**macOS:**
```bash
# Download from Releases or use Homebrew (coming soon)
brew install --cask ghostwriter  # placeholder
```

**Windows:**
Download the `.msi` installer from [Releases](https://github.com/arvinmoj/GhostWriter/releases)

**Linux:**
```bash
chmod +x GhostWriter-*.AppImage && ./GhostWriter-*.AppImage
```

### Setup (30 seconds)

1. **Get an API key** → [OpenRouter.ai](https://openrouter.ai) (one key for GPT-4, Claude, Llama, etc.)
2. **Create an instruction file** `~/ghost-instructions.md`:
   ```markdown
   Fix grammar and spelling mistakes. Improve clarity. Keep the original tone.
   ```
3. **Launch GhostWriter** → Configure your API key, instruction file path, and hotkey
4. **Go** → Highlight text anywhere and press your hotkey

### Usage

| Step | Action |
|------|--------|
| **1** | Select any text in any application |
| **2** | Press `Cmd + Shift + R` (macOS) / `Ctrl + Shift + R` (Windows/Linux) |
| **3** | GhostWriter reads your selection and sends it to AI with your instructions |
| **4** | Transformed text replaces your original selection — instantly |

---


## 🛠️ Features

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

### Multi-Model Support
Powered by [OpenRouter](https://openrouter.ai) — use any model available:
- OpenAI: GPT-4, GPT-3.5
- Anthropic: Claude 3 Opus, Sonnet, Haiku
- Meta: Llama 3
- And many more...

---

## 🔒 Security & Privacy

- **Your API key** — Stored encrypted on your local machine
- **No proxy servers** — AI processing uses your key directly, nothing routed through us
- **Zero telemetry** — No analytics, no usage data, no tracking
- **Open source** — Entire codebase is auditable

---

## ⚙️ Configuration

On first launch, GhostWriter will ask for:

| Setting | Description | Default |
|---------|-------------|---------|
| **API Key** | Your OpenRouter API key | Required |
| **Instruction File** | Path to your `.md` prompt file | `~/ghost-instructions.md` |
| **Model** | Preferred AI model | `openai/gpt-4` |
| **Hotkey** | Keyboard shortcut | `Cmd/Ctrl + Shift + R` |
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
  "first_run": true
}
```
</details>

<details>
<summary><b>macOS</b> — <code>~/Library/Application Support/ghostwriter/config.json</code></summary>

```json
{
  "api_key_encrypted": "",
  "model": "openai/gpt-4o-mini",
  "instruction_file": "$HOME/Library/Application Support/ghostwriter/instructions/default.md",
  "hotkey": {
    "modifiers": ["cmd"],
    "key": "r"
  },
  "proxy_url": null,
  "first_run": false
}
```
</details>

---

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

---

## 🤝 Contributing

Contributions are welcome! Whether it's:
- 🐛 Bug reports
- 💡 Feature suggestions
- 📝 Documentation improvements
- 🔧 Code contributions

Please open an [issue](https://github.com/arvinmoj/GhostWriter/issues) or submit a PR.

---

## 📄 License

[MIT](LICENSE) — (c) Arvin Moj

---

## 🙏 Acknowledgments

- [Tauri](https://tauri.app) — For the amazing cross-platform framework
- [OpenRouter](https://openrouter.ai) — For unified AI model access
- [Rust](https://www.rust-lang.org) — For performance and safety

---

<div align="center">

**⭐ If GhostWriter saves you time, consider starring the repo!**

[Report Bug](https://github.com/arvinmoj/GhostWriter/issues) · [Request Feature](https://github.com/arvinmoj/GhostWriter/issues) · [Download](https://github.com/arvinmoj/GhostWriter/releases)

</div>
