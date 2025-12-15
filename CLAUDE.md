# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Vitals is a modern cross-platform desktop system monitoring tool built with Tauri 2.0 and Vue 3. It's designed as a contemporary rewrite of Conky, providing real-time system metrics visualization with a modern web-based UI. The app features a transparent background and operates as a macOS-style desktop widget.

### Technology Stack
- **Backend**: Rust with Tauri 2.0 framework
- **Frontend**: Vue 3 + TypeScript + TailwindCSS 4.x + PrimeVue 4.x
- **Build Tool**: Vite 6.x
- **Package Manager**: pnpm
- **Chart Library**: Chart.js with vue-chartjs
- **UI Components**: PrimeVue (Card, Button, ProgressBar, Chart, etc.)
- **Icons**: PrimeIcons

## Development Commands

### Core Development
```bash
# Start development server with hot reload
pnpm tauri dev

# Install dependencies
pnpm install

# Build frontend (TypeScript check + Vite build)
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
cargo test             # Run tests (no tests currently)
cargo clippy           # Linting
```

### Type Checking
TypeScript checking is integrated into the build command:
```bash
# This is included in `pnpm build`
vue-tsc --noEmit
```

## Architecture

### Frontend Structure (`src/`)
- **main.ts**: Vue app entry point with PrimeVue configuration
- **App.vue**: Root component with drag functionality, pin-to-desktop, and keyboard shortcuts
- **vite-env.d.ts**: Vite environment type definitions
- **types/system.ts**: TypeScript interfaces for system metrics
- **utils/format.ts**: Utility functions for formatting bytes, percentages, frequencies
- **assets/widget.css**: Custom widget styling with transparent background
- **components/**:
  - `CpuMonitor.vue`: CPU usage, frequency, core count display
  - `MemoryMonitor.vue`: RAM usage with progress bars
  - `NetworkMonitor.vue`: Network speed charts with real-time line graphs

### Backend Structure (`src-tauri/`)
- **src/main.rs**: Tauri application entry point
- **src/lib.rs**: Core Tauri commands and system monitoring logic using `sysinfo` crate
- **Cargo.toml**: Rust dependencies including sysinfo for system monitoring
- **tauri.conf.json**: Window configuration for desktop widget mode

### Key Configuration Files
- **vite.config.ts**: Vite bundler configuration with TailwindCSS plugin (port 1420 fixed for Tauri)
- **tsconfig.json**: TypeScript compiler options with strict mode enabled
- **package.json**: Node.js dependencies and scripts
- **pnpm-lock.yaml**: Locked dependency versions

## Development Workflow

1. **Initial Setup**: Run `pnpm install` to install all dependencies
2. **Development**: Use `pnpm tauri dev` for full-stack development with hot reload
3. **Type Checking**: Run `pnpm build` (includes `vue-tsc --noEmit`)
4. **Building**: Use `pnpm tauri build` to create distributable desktop app

## Tauri-Specific Notes

- Development server runs on fixed port 1420 (required by Tauri)
- Frontend_dist is configured to `../dist` for Tauri builds
- Window configuration in `src-tauri/tauri.conf.json`:
  - Size: 400x700px (min 300x500px)
  - Transparent background
  - No decorations (borderless)
  - Skip taskbar (desktop widget mode)
  - Centered on screen
- macOS Private API enabled for enhanced window management
- Icons: Comprehensive icon set for all platforms

## Key Features Implementation

### System Monitoring
- **CPU**: Real-time usage per core, frequency display
- **Memory**: RAM usage with progress bars, swap monitoring
- **Network**: Real-time speed charts with download/upload tracking
- **Data polling**: Every 1 second interval

### Desktop Widget Features
- **Pin to desktop**: Toggle always-on-top mode
- **Drag functionality**: When not pinned
- **Keyboard shortcuts**:
  - Cmd+Q: Exit app
  - Cmd+P: Toggle pin on top
  - Cmd+R: Toggle monitoring
  - F12/Cmd+Option+I: Developer tools (debug mode only)
- **Transparent background**: Blends with desktop wallpaper

### UI/UX Design
- **PrimeVue components**: Card, Button, ProgressBar, Chart
- **TailwindCSS**: Utility-first styling
- **Glass morphism**: Backdrop blur effects
- **Chinese language support**: UI text in Chinese with English technical terms

## Important Implementation Notes

### Rust Backend
- Uses `sysinfo` crate for system monitoring
- Implements thread-safe state management with Arc<Mutex>
- Network speed calculation based on delta between measurements
- No GPU monitoring currently implemented (temperature field None)

### Frontend Architecture
- Reactive data binding with Vue 3 Composition API
- Chart.js integration for real-time network graphs
- Component-based structure with TypeScript props
- Custom formatters for human-readable data display

## Required Tools
- Node.js (with pnpm)
- Rust and Cargo
- Tauri CLI (installed via pnpm)

## VS Code Extensions (Recommended)
- Vue.volar (Vue 3 language support)
- tauri-apps.tauri-vscode (Tauri integration)
- rust-lang.rust-analyzer (Rust language support)

## PrimeVue Framework Documentation
The project uses PrimeVue 4.x as the primary UI component library. For detailed information about components, APIs, and usage examples, refer to:
- **PrimeVue.md**: Complete PrimeVue documentation generated on 2025-12-10 (located in project root)

### PrimeVue Components Currently Used
- **Card**: Container component for content sections
- **Button**: Interactive button component with various styles
- **ProgressBar**: Progress visualization component
- **Tag**: Label and categorization component
- **MeterGroup**: Group of meter/progress indicators
- **Knob**: Circular dial/gauge component
- **Chart**: Chart component integrated with Chart.js

### Key PrimeVue Features
- **Pass-Through API**: Access to internal DOM elements for custom attributes
- **Theming System**: Styled mode (pre-skinned) and unstyled mode (custom CSS)
- **Accessibility**: WCAG 2.1 AA level compliance
- **Input Variants**: Outlined (default) or filled input styles

## Project Goals
According to the README, the project aims to implement:
- Real-time CPU, memory, GPU, storage, and network monitoring
- Desktop widget functionality with "pin to desktop" feature
- Process management capabilities
- Customizable themes via CSS
- Cross-platform support (Linux focus, with Wayland considerations)