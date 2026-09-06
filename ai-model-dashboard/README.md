# AI Model Intelligence Dashboard

A blazing-fast, GitHub-native dashboard for tracking AI model releases, free tiers, and provider updates. Built with Rust + Dioxus, hosted on GitHub Pages.

## Features

- 🚀 **Real-time Intelligence**: Automated daily scans of Hugging Face, GitHub Releases, and official blogs
- 💰 **Free Tier Tracker**: Clear visibility into free models and their usage limits
- 🔍 **Advanced Search & Filter**: Find models by provider, modality, license, and more
- ✅ **AI-Validated Data**: Multi-step validation using OpenCode AI and rule-based checks
- 🛡️ **Security First**: OWASP Top 10 mitigation, CSP headers, atomic operations
- 📊 **Benchmark Insights**: Community-verified performance metrics
- 🎨 **Carbon Design System**: Professional UI with rounded corners and IBM Plex fonts

## Architecture

```
GitHub Actions (Daily) → Data Scrapers → AI Validation → JSON Build → Dioxus Frontend → GitHub Pages
```

## Quick Start

### Local Development

```bash
# Install pnpm
curl -fsSL https://get.pnpm.io/install.sh | sh -

# Install dependencies
pnpm install

# Run local CI simulation
./scripts/ci-local.sh

# Build frontend
cd frontend
dx serve --platform web
```

### Configuration

Edit `frontend/content/config.toml` to customize:
- AI providers and models
- Validation prompts (zen/go modes)
- Data sources and scraping rules

## GitHub Actions Pipeline

The pipeline runs automatically once daily (6 AM UTC) with manual trigger option:

1. **Security Audit**: Dependency scanning with cargo-audit
2. **Code Quality**: Clippy linting, formatting checks
3. **Testing**: Unit and integration tests (100% coverage required)
4. **Data Update**: Scrape sources, validate with AI, generate JSON
5. **Build**: Compile Dioxus frontend to WASM
6. **Deploy**: Push to GitHub Pages with security headers

**Cost Optimization**: 
- Runs only 1-2x daily (vs 4x in reference implementation)
- Local testing recommended before deployment
- Efficient caching reduces runtime to ~20 minutes

## Project Structure

```
ai-model-dashboard/
├── frontend/              # Dioxus web application
│   ├── src/
│   │   ├── components/    # Reusable UI components
│   │   ├── pages/         # Route handlers
│   │   ├── config/        # TOML configuration loaders
│   │   └── utils/         # Helpers and API clients
│   ├── content/           # TOML configs (providers, prompts)
│   ├── tests/             # Frontend test suite
│   └── Dioxus.toml        # Dioxus build configuration
├── shared/                # Shared types and utilities
├── scripts/               # CI/CD automation scripts
├── .github/workflows/     # GitHub Actions definitions
└── Cargo.toml             # Workspace configuration
```

## Security Features

- ✅ Content Security Policy (CSP)
- ✅ X-Frame-Options: DENY
- ✅ HSTS with preload
- ✅ No credential persistence
- ✅ Atomic file operations
- ✅ Schema validation for all data
- ✅ OWASP Top 10 protection

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Run `./scripts/ci-local.sh` to validate changes
4. Submit a pull request

For detailed contribution guidelines, see [CONTRIBUTING.md](CONTRIBUTING.md).
