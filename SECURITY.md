# Security Policy

## Supported Versions

We release patches for security vulnerabilities in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of our chess application seriously. If you discover a security vulnerability, please follow these steps:

### How to Report

1. **Do NOT** open a public issue for security vulnerabilities
2. Report via GitHub Security Advisories at https://github.com/ZuhaadRathore/chess-engine/security/advisories/new
3. Include as much information as possible:
   - Type of vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if available)

### What to Expect

- **Acknowledgment**: We'll acknowledge receipt within 48 hours
- **Initial Assessment**: We'll provide an initial assessment within 5 business days
- **Updates**: We'll keep you informed of our progress
- **Resolution**: We'll work to release a fix as quickly as possible
- **Credit**: We'll credit you in the release notes (if you wish)

### Security Best Practices

When using this application:

- Keep your dependencies up to date
- Only download from official sources
- Verify checksums of released binaries
- Report any suspicious behavior

## Known Security Considerations

This is a local chess application with no network features or user data collection. However, we still take security seriously:

- **Input Validation**: All chess moves are validated before execution
- **Memory Safety**: Built with Rust for memory safety guarantees
- **Sandboxing**: Tauri provides OS-level sandboxing for web content
- **No Data Collection**: The app does not collect, store, or transmit user data

## Security Updates

Security updates will be released as soon as possible and announced via:
- GitHub Security Advisories
- Release notes
- README updates

Thank you for helping keep this project secure!
