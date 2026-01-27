# Multi-Downloader NX - Dioxus Edition

A complete rebuild of the multi-downloader library as a **Dioxus 0.7.1 fullstack application**.

## Overview

Multi-Downloader NX is a powerful anime downloader supporting Crunchyroll, Hidive, and AnimationDigitalNetwork, now rebuilt with Rust and Dioxus for improved performance, safety, and modern features.

## Features

### Core Functionality
- ✅ Download from Crunchyroll, Hidive, and AnimationDigitalNetwork
- ✅ Queue management with priority support
- ✅ DRM decryption (L3 CDM)
- ✅ Multiple quality options

### New Features
- 🎨 **Theme System**: Light, Dark, OLED Black, Solarized, Dracula, and custom themes
- 🌍 **Internationalization**: Support for 16 languages with RTL support
- ⏰ **Scheduled Downloads**: Plan downloads with recurring schedules
- 🔔 **Discord Webhooks**: Customizable notifications with template support
- 📊 **DRM Status Dashboard**: Real-time CDM validation and setup status
- 📱 **Responsive Design**: Mobile-first with touch-friendly controls

## Prerequisites

- Rust 1.70 or higher
- Node.js 18+ (for Tailwind CSS processing)
- ffmpeg >= 4.0.0
- MKVToolNix >= 60.0.0

## Installation

### 1. Install Dependencies

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js dependencies for Tailwind
npm install -D tailwindcss postcss autoprefixer
```

### 2. Build the Project

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

### 3. Run the Application

```bash
# Development mode with hot reload
dx serve

# Or with cargo
cargo run
```

The application will be available at `http://localhost:8080`

## Project Structure

```
src/
├── main.rs                 # Application entry point
├── lib.rs                  # Shared library code
├── app.rs                  # Root Dioxus component
├── router.rs               # Application routing
├── components/             # Reusable UI components
├── pages/                  # Page components
├── server/                 # Server-side logic
├── services/               # Business logic
├── models/                 # Data structures
├── state/                  # Application state management
└── hooks/                  # Custom Dioxus hooks

locales/                    # i18n translation files
assets/                     # Static assets
```

## Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

### Linting

```bash
cargo clippy
```

### Formatting

```bash
cargo fmt
```

## Configuration

Edit the `Dioxus.toml` file to configure the application settings.

## Internationalization

The application supports 16 languages:
- English (en)
- German (de)
- Spanish (es)
- French (fr)
- Italian (it)
- Portuguese (pt, pt-BR)
- Russian (ru)
- Chinese (zh-CN, zh-TW)
- Japanese (ja)
- Korean (ko)
- Polish (pl)
- Dutch (nl)
- Turkish (tr)
- Arabic (ar) with RTL support

## Themes

Available themes:
- Light (default)
- Dark
- OLED Black
- Solarized Light
- Solarized Dark
- Dracula
- Custom themes (create your own)

## License

MIT License - see LICENSE.md for details

## Legal Warning

This application is not endorsed by or affiliated with Crunchyroll, Hidive, or AnimationDigitalNetwork. This application enables you to download videos for offline viewing which may be forbidden by law in your country. The usage of this application may also cause a violation of the Terms of Service between you and the stream provider. This tool is not responsible for your actions; please make an informed decision before using this application.
