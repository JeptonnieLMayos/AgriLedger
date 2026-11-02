# AgriLedger 🌱

**A simple, offline-first farm recordkeeping application built to help farmers manage their agricultural activities and keep track of their farm operations.**

AgriLedger is a free farm management application designed with small-scale farmers in mind, particularly Filipino farmers who need a practical way to organize farm records without depending on a constant internet connection.

The project aims to provide a lightweight, accessible, and reliable tool for recording farm information, monitoring activities, and understanding farm operations.

---

## 🎯 Purpose and Goals

Many farmers still rely on notebooks, memory, or scattered records to manage their farms. This can make it difficult to track expenses, monitor activities, and review previous planting cycles.

AgriLedger aims to address this by providing:

- **Accessible farm management:** Make digital farm recordkeeping practical and easy to use.
- **Offline-first operation:** Allow farmers to access and manage their records without an internet connection.
- **Simplicity:** Keep the interface straightforward and avoid unnecessary complexity.
- **Data ownership:** Store farm records locally rather than requiring a cloud account.
- **Lightweight performance:** Minimize unnecessary dependencies and resource consumption.
- **Practical usefulness:** Focus on features that address actual farm management needs.

The long-term goal is to give farmers a useful tool for understanding and managing their own agricultural operations.

---

## ✨ Features

### Implemented / In Development

- [x] Crop record creation, editing, and deletion
- [x] Crop information storage using SQLite
- [x] Local database initialization
- [x] Current date integration
- [x] Cross-platform Rust application structure
- [ ] Field management
- [ ] Activity scheduling and tracking
- [ ] Material usage tracking
- [ ] Harvest records
- [ ] Farm expense and cost tracking
- [ ] Soil test records
- [ ] Temperature records

*Features marked as incomplete are planned or under development.*

---

## 🛠️ Technology Stack

AgriLedger uses a Rust-first technology stack.

| Technology | Purpose                                |
| ---------- | -------------------------------------- |
| Rust       | Application logic and backend          |
| Slint      | Declarative, code-based user interface |
| SQLite     | Local database                         |
| rusqlite   | Rust interface for SQLite              |
| Cargo      | Build system and dependency management |

### Design Principles

- Offline-first architecture
- Local SQLite database
- Code-defined user interface
- Minimal dependencies
- Shared application logic across platforms
- Platform-specific integration where necessary

---

## 📱 Platform Support

The project is being developed with cross-platform support in mind.

| Platform | Status                  |
| -------- | ----------------------- |
| Linux    | Development             |
| Android  | Development and testing |
| Windows  | Planned                 |
| macOS    | Planned                 |
| iOS      | Planned                 |

Platform availability will depend on development and testing.

---

## 📦 Installation and Development

### Requirements

- Rust toolchain
- Cargo
- Git
- SQLite development support as required by your build environment
- Android SDK and NDK for Android development

### Clone the repository

```bash
git clone <repository-url>
cd AgriLedger
```

### Build for desktop

```bash
cargo build
```

### Run on desktop

```bash
cargo run
```

### Build for Android

Install and configure the Android SDK and NDK, then configure the appropriate Rust target.

For an ARM64 Android target:

```bash
rustup target add aarch64-linux-android
```

Run the Android application using Cargo APK:

```bash
cargo apk run --target aarch64-linux-android --lib
```

*Android build instructions may require additional environment configuration depending on your development environment.*

---

## 💾 Data Storage

AgriLedger uses SQLite for local data storage.

Farm records are stored in a local application database. The application is designed to work without requiring an internet connection.

The database structure is being developed around separate records for crops, activities, materials, and other farm information.

---

## 🗺️ Development Roadmap

AgriLedger is actively under development.

Planned development stages:

1. **Core foundation**

   - Application structure
   - Database initialization
   - Basic crop management

2. **Farm management**

   - Field records
   - Crop cycles
   - Activity scheduling
   - Material usage

3. **Farm records and analysis**

   - Harvest tracking
   - Expense tracking
   - Soil and environmental records
   - Production summaries

4. **Cross-platform development**

   - Android improvements
   - Windows support
   - macOS and iOS support

5. **Future improvements**

   - Data export and backup
   - Reports and statistics
   - Additional farmer-focused tools

---

## 🤝 Project Philosophy

AgriLedger is being developed with the belief that useful agricultural technology should be accessible to farmers, not another source of unnecessary complexity or expense.

The application prioritizes practical functionality, local data storage, and independence from constant internet connectivity.

---

## 📜 License

License: To be determined.

---

**AgriLedger — Practical farm records. Better-informed farming.**
