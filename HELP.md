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
  "proxy_url": null,
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

## Step 4: Using the Hotkey

### Default Hotkey
- **macOS**: `Cmd + Shift + R`
- **Windows/Linux**: `Ctrl + Shift + R`

### Usage Flow
1. Select text in any application (Notes, Word, Slack, IDE, browser, etc.)
2. Press the hotkey
3. GhostWriter will:
   - Simulate `Cmd+A`/`Ctrl+A` (Select All)
   - Simulate `Cmd+C`/`Ctrl+C` (Copy)
   - Send your text + instruction to OpenRouter AI
   - Wait for the AI response
   - Simulate `Cmd+V`/`Ctrl+V` (Paste) to replace original text
4. The transformed text appears instantly!

### Hotkey Customization
To change the hotkey, edit your config file:
```json
{
  "hotkey": {
    "modifiers": ["cmd", "option"],
    "key": "t"
  }
}
```
Valid modifier keys: `cmd`, `ctrl`, `alt`, `option`, `shift`
Valid key: any single letter or number (a-z, 0-9)

## Troubleshooting

### App quit unexpectedly on macOS
This usually means missing accessibility permissions:
1. Go to **System Settings → Privacy & Security → Accessibility**
2. Find GhostWriter in the list and enable it
3. You may also need to enable **Input Monitoring**
4. Restart GhostWriter

### No text transformation happening
Check these:
1. Is GhostWriter running? (Look for the icon in your menu bar)
2. Did you press the correct hotkey?
3. Is text actually selected when you press the hotkey?
4. Do you have an internet connection? (Required for API calls)
5. Check the app logs: Run GhostWriter from Terminal to see output

### API Key Issues
- Verify your OpenRouter API key is correct
- Check you have credits in your OpenRouter account
- The key is stored encrypted in `~/.config/ghostwriter/config.json`
- To reset: delete the config file and restart the app

### Performance
- First request may be slower (API initialization)
- Subsequent requests are faster
- Typical response time: 1-3 seconds depending on model and text length

## Privacy & Security

- **Your API key stays local**: Encrypted and stored only on your machine
- **No data collection**: GhostWriter doesn't track usage or collect analytics
- **OpenRouter only**: Text is sent only to OpenRouter (you control which models)
- **No logging**: Your text and API key are never logged or stored beyond memory
- **Open source**: Inspect the code at https://github.com/arvinmoj/GhostWriter

## File Locations

### Configuration
- **Config file**: `~/.config/ghostwriter/config.json`
- **Instructions directory**: `~/.config/ghostwriter/instructions/`
- **Default instruction**: `~/.config/ghostwriter/instructions/default.md`

### Application
- **Binary**: `/Applications/GhostWriter.app/Contents/MacOS/ghostwriter` (if installed via DMG)
- **Logs**: Run from Terminal to see real-time output

## Advanced Usage

### Reloading Configuration
GhostWriter automatically watches for changes to:
- Config file
- Instruction file
No restart needed when you edit these files.

### Multiple Instruction Files
Create multiple `.md` files and switch between them by updating the `instruction_file` path in your config.

### Custom Models
Change the model in your config:
```json
{
  "model": "anthropic/claude-3.5-sonnet"
}
```
Popular options:
- `openai/gpt-4o` (most capable)
- `openai/gpt-4o-mini` (fast & cheap, default)
- `anthropic/claude-3.5-sonnet` (excellent for writing)
- `meta-llama/llama-3.1-70b-instruct` (open source option)

### Proxy Configuration
GhostWriter can route OpenRouter API requests through an HTTP or SOCKS proxy. Set the `proxy_url` field in your config:
```json
{
  "proxy_url": "http://127.0.0.1:8080"
}
```
Supported protocols:
- `http://` - HTTP proxy
- `https://` - HTTPS proxy
- `socks5://` - SOCKS5 proxy

Omit or set to `null` to connect directly without a proxy.

## Frequently Asked Questions

**Q: Does GhostWriter work offline?**
A: No, it requires an internet connection to reach the OpenRouter API.

**Q: Is there a usage cost?**
A: Yes, you pay OpenRouter for API usage based on the model and tokens used. Check openrouter.ai for pricing.

**Q: Can I use local models instead?**
A: Not in this version - it requires an API connection to OpenRouter.

**Q: What happens if I select a lot of text?**
A: There's a practical limit based on the model's context window (typically 8K-32K tokens). Very large selections may be truncated.

**Q: Does it work in password fields or secure inputs?**
A: No, for security reasons it cannot access text in secure input fields.

**Q: How do I quit GhostWriter?**
A: Right-click the menu bar icon and select "Quit", or run `pkill -f ghostwriter` in Terminal.

## Getting Help

If you encounter issues:
1. Check the troubleshooting section above
2. Run GhostWriter from Terminal to see error output
3. Visit the GitHub repository: https://github.com/arvinmoj/GhostWriter
4. Ensure you have the latest version

Happy writing!