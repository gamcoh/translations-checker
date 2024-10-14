# 🛠️ Translation Checker Pre-commit Hook

![GitHub License](https://img.shields.io/github/license/gamcoh/translations-checker) ![GitHub Last Commit](https://img.shields.io/github/last-commit/gamcoh/translations-checker)

> **A lightweight pre-commit hook to check for missing translations in TypeScript projects**  
> Automatically scan your staged files for translation keys (`t()` function) and update the translation files if necessary. This tool is designed to help developers keep their localization files up-to-date during the commit process.

## ✨ Features

- Automatically detects `t()` calls in TSX/TS/JS/JSX files
- Compares the detected keys with your localization JSON files (e.g., `fr.json`, `he.json`)
- Adds missing keys with empty strings and alerts the developer
- Simple configuration via the `.pre-commit-config.yaml` file
- Customizable via command-line arguments

## 🚀 Quick Start

1. **Install Pre-commit**
   Make sure you have `pre-commit` installed on your machine:

   ```bash
   pip install pre-commit
   ```

2. **Add the Hook to Your Project**

   In your project's root directory, create (or update) the `.pre-commit-config.yaml` file and add the following entry:

   ```yaml
   - repo: https://github.com/gamcoh/translations-checker
     rev: v0.1.0  # Use the latest release
     hooks:
     - id: translation-checker
       args: ["--json-file", "localization/fr.json"]
   ```

3. **Install the Hook**

   Run the following command to install the pre-commit hook:

   ```bash
   pre-commit install
   ```

4. **Commit as Usual**

   Every time you stage and commit TypeScript files (for example), the hook will run and ensure that all translation keys are properly checked and updated in the specified localization JSON file.

## 🛠 Usage

By default, the hook scans all staged `.ts,.tsx,.js,.jsx` files for calls to the `t()` function and compares the keys found with the entries in your localization file (e.g., `fr.json`).

### Command-Line Arguments

You can customize the behavior of the hook using command-line arguments specified in `.pre-commit-config.yaml`:

- `--json-file`: Path to the JSON file where translation keys are stored.

Example:

```yaml
- repo: https://github.com/gamcoh/translations-checker
  rev: v0.1.0
  hooks:
  - id: translation-checker
    args: ["--json-file", "path/to/fr.json"]
```

## 📂 Project Structure

```
translation-checker-precommit/
├── src/                     # Rust source code
├── target/                  # Compiled binaries
├── run_translation_checker.sh # Shell script to invoke the Rust binary
├── .pre-commit-hooks.yaml   # Pre-commit hook configuration
└── README.md                # Project documentation
```

## 📦 Installation for Development

1. Clone the repository:

   ```bash
   git clone https://github.com/gamcoh/translations-checker.git
   ```

2. Build the Rust project:

   ```bash
   cargo build --release
   ```

3. Run the binary directly to test it locally:

   ```bash
   ./target/release/translation_checker --json-file localization/fr.json
   ```

## 🛡️ Security

The `translation-checker-precommit` uses `git2` to access your staged files and check for translation keys. It only runs on files that are staged for commit, and no data is transmitted or stored externally.

## 🤝 Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

### Steps to Contribute:

1. Fork the repository
2. Create a new branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💬 Questions or Feedback?

Feel free to reach out via GitHub Issues or submit any questions. We appreciate feedback and contributions from the community.

---

### 📢 Spread the Word

If you find this tool helpful, please give it a ⭐️ on GitHub and share it with others in your community!

---

**Happy Coding! 💻**

---

### Badges and Links

- **Repository**: [translation-checker-precommit](https://github.com/gamcoh/translations-checker)
- **License**: [MIT](https://opensource.org/licenses/MIT)
