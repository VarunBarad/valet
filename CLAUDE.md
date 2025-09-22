# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Valet is a Rust-based file management utility that automatically processes and organizes files from an inbox directory. It operates like a real-world valet, scanning the inbox and "parking" various files into their appropriate destinations.

The primary function is processing credit card statements:
- Decrypts password-protected PDF statements using `qpdf`
- Extracts statement dates using `pdftotext`
- Renames and organizes files into year-based directory structures
- Supports HDFC and ICICI bank credit card statements

## Architecture

The codebase is structured as a simple binary with modular components:

- `main.rs`: Entry point handling configuration, network mount setup/cleanup, and orchestration
- `credit_cards.rs`: Core business logic for processing different bank statements
- `build.rs`: Build-time configuration for embedding environment variables

### Key Components

**Configuration (`Config` struct in main.rs:11-16)**:
- Network storage paths (local mount and remote SMB share)
- Temporary directory for processing
- Inbox directory for scanning

**Network Storage Integration**:
- Automatically mounts SMB share from "delphinus" server
- Uses macOS Keychain integration for credential retrieval
- Unmounts after processing

**Statement Processing Pipeline**:
1. Scan inbox for files matching bank-specific patterns (regex-based)
2. Decrypt PDFs using bank-specific passwords
3. Parse statement dates (filename-based for HDFC, content-based for ICICI)
4. Organize into year-based directory structure
5. Clean up temporary and inbox files

## Development Commands

### Building
```bash
cargo build
cargo build --release
```

### Running
Requires environment variables:
```bash
export CREDIT_CARD_HDFC_PASSWORD="your_hdfc_password"
export CREDIT_CARD_ICICI_PASSWORD="your_icici_password"
cargo run
```

### Code Formatting
Uses custom rustfmt configuration (rustfmt.toml):
```bash
cargo fmt
```

Key formatting rules:
- Hard tabs (tab_spaces = 4)
- Max width = 120
- Reorder imports and modules enabled

### Linting
```bash
cargo clippy
cargo clippy -- -D warnings
```

## External Dependencies

The application requires these system tools to be installed and available in PATH:
- **qpdf**: Install via `brew install qpdf` (for PDF decryption)
- **pdftotext**: Install via `brew install xpdf` (for text extraction)

## File Processing Patterns

### HDFC Bank
- Pattern: Files containing "5589" in filename
- Date extraction: From filename using format "DD-MM-YYYY"
- Destination: `Bank Accounts/Varun - HDFC/Credit Card statement/{YEAR}/`

### ICICI Bank
- Pattern: Files containing "5241" in filename
- Date extraction: From PDF content using "pdftotext" and regex matching "Statement Date"
- Date format: "Month DD, YYYY" (e.g., "January 15, 2024")
- Destination: `Bank Accounts/Varun - ICICI/Credit Card statement/{YEAR}/`

## Security Considerations

- Passwords are embedded at build time via environment variables
- Network credentials retrieved from macOS Keychain
- Temporary files are cleaned up after processing
- Error handling includes status code validation for qpdf operations (codes 0, 2, 3 are acceptable for HDFC; 0, 3 for ICICI)