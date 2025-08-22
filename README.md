# doxx 📄

> `.docx` files in your terminal — no Microsoft Word required

[![CI](https://github.com/bgreenwell/doxx/workflows/CI/badge.svg)](https://github.com/bgreenwell/doxx/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

A fast, terminal-native document viewer for Word files. View, search, and export `.docx` documents without leaving your command line.

## Screenshots

<div align="center">
  <table>
    <tr>
      <td align="center">
        <img src="assets/screenshot1-images.png" alt="Terminal image display" width="400">
        <br><em>Terminal image display</em>
      </td>
      <td align="center">
        <img src="assets/screenshot2-colors.png" alt="Color support" width="400">
        <br><em>Color support</em>
      </td>
    </tr>
    <tr>
      <td align="center">
        <img src="assets/screenshot3-tables.png" alt="Smart tables" width="400">
        <br><em>Smart tables with alignment</em>
      </td>
      <td align="center">
        <img src="assets/screenshot4-lists.png" alt="Lists and formatting" width="400">
        <br><em>Lists and formatting</em>
      </td>
    </tr>
  </table>
</div>

## ✨ Features

- **Beautiful terminal rendering** with formatting, tables, and lists
- **Fast search** with highlighting 🔍
- **Smart tables** with proper alignment and Unicode borders
- **Copy to clipboard** — grab content directly from the terminal
- **Export formats** — Markdown, CSV, JSON, plain text
- **Terminal images** for Kitty, iTerm2, WezTerm 🖼️
- **Color support** — see Word document colors in your terminal

## 🚀 Installation

### Pre-built binaries

Download from [GitHub releases](https://github.com/bgreenwell/doxx/releases):

```bash
# macOS/Linux
curl -L https://github.com/bgreenwell/doxx/releases/latest/download/doxx-$(uname -s)-$(uname -m).tar.gz | tar xz
sudo mv doxx /usr/local/bin/

# Verify
doxx --version
```

### Build from source

```bash
git clone https://github.com/bgreenwell/doxx.git
cd doxx
cargo install --path .
```

## 🎯 Usage

```bash
# View a document
doxx report.docx

# Search for content
doxx contract.docx --search "payment"

# Start with outline view
doxx document.docx --outline

# Export to different formats
doxx data.docx --export csv > data.csv
doxx report.docx --export markdown > report.md

# View with images (supported terminals)
doxx presentation.docx --images --export text

# Enable color rendering
doxx slides.docx --color
```

## 📋 Command Line Options

### Basic Options
```bash
doxx [OPTIONS] <FILE>
```

| Option | Description |
|--------|-------------|
| `<FILE>` | Input document file (.docx) |
| `-h, --help` | Show help information |
| `-V, --version` | Show version information |

### Viewing Options
| Option | Description |
|--------|-------------|
| `-o, --outline` | Start with outline view for quick navigation |
| `-p, --page <PAGE>` | Jump to specific page number on startup |
| `-s, --search <TERM>` | Search and highlight term immediately |
| `--force-ui` | Force interactive UI mode (bypass TTY detection) |
| `--color` | Enable color support for text rendering |

### Export Options
| Option | Values | Description |
|--------|--------|-------------|
| `--export <FORMAT>` | `markdown`, `text`, `csv`, `json` | Export document instead of viewing |

**Export Examples:**
```bash
doxx report.docx --export markdown  # Convert to Markdown
doxx data.docx --export csv         # Extract tables as CSV  
doxx document.docx --export text    # Plain text output
doxx structure.docx --export json   # Document metadata as JSON
```

### Image Options
| Option | Description |
|--------|-------------|
| `--images` | Display images inline in terminal (auto-detect capabilities) |
| `--extract-images <DIR>` | Extract images to specified directory |
| `--image-width <COLS>` | Maximum image width in terminal columns (default: auto-detect) |
| `--image-height <ROWS>` | Maximum image height in terminal rows (default: auto-detect) |
| `--image-scale <SCALE>` | Image scaling factor (0.1 to 2.0, default: 1.0) |

**Image Examples:**
```bash
doxx presentation.docx --images                    # Show images inline
doxx document.docx --images --image-width 80       # Limit image width
doxx slides.docx --extract-images ./images/        # Save images to folder
```

**⚠️ Image Display Notes:**
- `--images` currently works with `--export text` mode and shows placeholders in TUI
- Supports iTerm2, Kitty, and WezTerm terminals


## ⌨️ Navigation

| Key | Action |
|-----|--------|
| `↑`/`k` | Scroll up |
| `↓`/`j` | Scroll down |
| `o` | Toggle outline |
| `s` | Search |
| `c` | Copy to clipboard |
| `h` | Help |
| `q` | Quit |

## 🔧 Why doxx?

Current terminal tools for Word documents:
- **docx2txt** → Loses all formatting, mangled tables
- **pandoc** → Complex chain, formatting lost
- **antiword** → Only handles old `.doc` files

**doxx** gives you:
- ✅ Rich formatting preserved (bold, italic, headers)
- ✅ Professional table rendering with alignment
- ✅ Interactive navigation and search
- ✅ Multiple export formats for workflows
- ✅ Terminal image display for modern terminals
- ✅ Fast startup (50ms vs Word's 8+ seconds)

Perfect for developers, sysadmins, and anyone who prefers the terminal.

## 📊 Examples

### Quick document analysis
```bash
# Get overview and search
doxx quarterly-report.docx
doxx --search "revenue"

# Extract tables for analysis
doxx financial-data.docx --export csv | python analyze.py
```

### Copy workflows
```bash
# Review and copy sections
doxx meeting-notes.docx
# Press 'c' to copy current view to clipboard

# Copy search results
doxx specs.docx --search "requirements"
# Press F2 to copy results with context
```

### Pipeline integration
```bash
# Extract text for processing
doxx notes.docx --export text | grep "action items"

# Get document structure
doxx report.docx --export json | jq '.metadata'
```

## 🏗️ Architecture

Built with Rust for performance:
- **[docx-rs](https://crates.io/crates/docx-rs)** — Document parsing
- **[ratatui](https://crates.io/crates/ratatui)** — Terminal UI
- **[viuer](https://crates.io/crates/viuer)** — Image rendering
- **[unicode-segmentation](https://crates.io/crates/unicode-segmentation)** — Proper Unicode handling

## 🛠️ Development

```bash
# Build and test
cargo build --release
cargo test

# Run with sample document
cargo run -- tests/fixtures/sample.docx
```

## 📋 Roadmap

- Image support in TUI via ratatui-image crate
- Enhanced table support (merged cells, complex layouts)
- Performance improvements for large documents
- Hyperlink navigation
- Custom themes

## 📝 License

MIT License — see [LICENSE](LICENSE) file for details.

---

**Made for developers who live in the terminal** 🚀