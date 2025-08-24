# AI Training Desktop

A cross-platform desktop application for AI model training marketplace built with Tauri v2, React, and Rust.

## Features

- **Cross-Platform**: Windows, macOS, and Linux support
- **Modern UI**: React + TypeScript frontend with responsive design
- **High Performance**: Rust backend for native system integration
- **ML Framework Integration**: Candle (Rust-native) and PyTorch support planned
- **Distributed Training**: Flower federation client integration (planned)
- **System Monitoring**: Real-time hardware and resource monitoring
- **Secure Architecture**: Tauri's security model with controlled API access

## Tech Stack

### Frontend
- **React 18** - Modern UI framework
- **TypeScript** - Type safety and developer experience
- **Vite** - Fast build tool and development server
- **Tauri API** - Native desktop integration

### Backend
- **Rust** - Systems programming for performance and safety
- **Tauri v2** - Desktop application framework
- **Tokio** - Async runtime for concurrent operations
- **Candle** - Rust-native machine learning framework
- **sysinfo** - System information and monitoring

## Quick Start

### Prerequisites
- [Node.js](https://nodejs.org/) (v16 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for your platform

### Development

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd ai-training-desktop
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Start development servers**
   
   Terminal 1 - Frontend development server:
   ```bash
   npm run dev
   ```
   
   Terminal 2 - Tauri development (after frontend is running):
   ```bash
   npm run tauri:dev
   ```

### Building for Production

```bash
# Build frontend
npm run build

# Build complete desktop application
npm run tauri:build
```

## Project Structure

```
├── src/                    # React frontend
│   ├── components/         # Reusable UI components
│   ├── pages/             # Application pages
│   ├── services/          # API communication
│   ├── types/             # TypeScript definitions
│   └── utils/             # Frontend utilities
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── commands/      # Tauri command handlers
│   │   ├── core/          # Application core logic
│   │   ├── ml/            # ML framework integration
│   │   ├── network/       # Distributed training
│   │   ├── security/      # Authentication & encryption
│   │   └── system/        # Hardware detection
│   └── Cargo.toml         # Rust dependencies
└── .github/
    └── copilot-instructions.md  # AI agent guidance
```

## Available Commands

| Command | Description |
|---------|-------------|
| `npm run dev` | Start Vite development server |
| `npm run build` | Build frontend for production |
| `npm run preview` | Preview production build |
| `npm run tauri:dev` | Start Tauri development mode |
| `npm run tauri:build` | Build desktop application |

## Contributing

1. Read the [Copilot Instructions](.github/copilot-instructions.md) for AI agent guidance
2. Follow the established patterns for React components and Rust modules
3. Ensure type safety across the Rust-TypeScript boundary
4. Test both frontend and backend changes

## Architecture

This application uses a **hybrid architecture**:

- **Frontend (React)**: Modern web UI with TypeScript for type safety
- **Backend (Rust)**: Native system integration, ML frameworks, and security
- **Communication**: Tauri commands provide type-safe bridge between layers
- **Security**: Tauri's capability system controls frontend access to native APIs

## Roadmap

- [x] Basic Tauri + React setup
- [x] System information monitoring
- [x] ML model management interface
- [ ] Candle ML framework integration
- [ ] Distributed training with Flower
- [ ] User authentication and security
- [ ] Model marketplace integration
- [ ] GPU acceleration support
- [ ] Cross-platform deployment automation

## License

[License information]
