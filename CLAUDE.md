# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Vitals is a modern cross-platform desktop system monitoring tool built with Tauri 2.0 and Vue 3. It's designed as a contemporary rewrite of Conky, providing real-time system metrics visualization with a modern web-based UI.

### Technology Stack
- **Backend**: Rust with Tauri 2.0 framework
- **Frontend**: Vue 3 + TypeScript + TailwindCSS 4.x
- **Build Tool**: Vite 6.x
- **Package Manager**: pnpm

## Development Commands

### Core Development
```bash
# Start development server with hot reload
pnpm tauri dev

# Install dependencies
pnpm install

# Type checking (Vue/TypeScript)
vue-tsc --noEmit

# Build for production
pnpm build

# Tauri-specific operations
pnpm tauri build    # Build desktop app
pnpm tauri dev      # Development mode
```

### Frontend Only
```bash
# Frontend dev server (without Tauri)
pnpm dev

# Preview built frontend
pnpm preview
```

### Backend (Rust)
```bash
# From src-tauri directory
cargo build            # Build Rust backend
cargo run              # Run backend
cargo test             # Run tests
cargo clippy           # Linting
```

## Architecture

### Frontend Structure (`src/`)
- **main.ts**: Vue app entry point
- **App.vue**: Root component with default template
- **vite-env.d.ts**: Vite environment type definitions

### Backend Structure (`src-tauri/`)
- **src/main.rs**: Tauri application entry point
- **src/lib.rs**: Core Tauri commands and application setup
- **Cargo.toml**: Rust dependencies and project metadata
- **tauri.conf.json**: Tauri application configuration

### Key Configuration Files
- **vite.config.ts**: Vite bundler configuration with TailwindCSS plugin
- **tsconfig.json**: TypeScript compiler options
- **package.json**: Node.js dependencies and scripts
- **pnpm-lock.yaml**: Locked dependency versions

## Development Workflow

1. **Initial Setup**: Run `pnpm install` to install all dependencies
2. **Development**: Use `pnpm tauri dev` for full-stack development with hot reload
3. **Type Checking**: Run `vue-tsc --noEmit` before commits
4. **Building**: Use `pnpm tauri build` to create distributable desktop app

## Tauri-Specific Notes

- Development server runs on fixed port 1420 (required by Tauri)
- Frontend_dist is configured to `../dist` for Tauri builds
- Window configuration is in `src-tauri/tauri.conf.json`
- Currently set to 800x600px but can be adjusted for system monitoring widget needs

## Required Tools
- Node.js (with pnpm)
- Rust and Cargo
- Tauri CLI (installed via pnpm)

## VS Code Extensions (Recommended)
- Vue.volar (Vue 3 language support)
- tauri-apps.tauri-vscode (Tauri integration)
- rust-lang.rust-analyzer (Rust language support)

## Project Goals
According to the README, the project aims to implement:
- Real-time CPU, memory, GPU, storage, and network monitoring
- Desktop widget functionality with "pin to desktop" feature
- Process management capabilities
- Customizable themes via CSS
- Cross-platform support (Linux focus, with Wayland considerations)