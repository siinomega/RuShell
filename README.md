# 🌐 RuShell

**RuShell** This Rust-based reverse shell client establishes a connection to an attacker's server, allowing the execution of shell commands remotely. It uses the Tokio async runtime for efficient handling of asynchronous I/O operations and leverages the colored crate for user-friendly output.

## ✏️ Features

- **Asynchronous Execution:** Utilizes Rust's async capabilities for efficient command execution.
- **Command History:** Maintains a history of executed commands.
- **Colored Output:** Enhances user experience with color-coded messages for better visibility.
- **Error Handling:** Robust error handling for connection and command execution failures.

## 🛠️ Prerequisites

Before running the tool, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) (1.67+)
- [Tokio](https://tokio.rs/) (for async capabilities)
- [Colored](https://docs.rs/colored/) (for color-coded console output)

You can add these dependencies to your `Cargo.toml`:

```toml
[dependencies]
colored = "2.1.0"
tokio = { version = "1.41.0", features = ["rt", "rt-multi-thread", "macros"] }
```
## 💻 Installation

1. CloneThe Repository :

   ```bash
   git clone https://github.com/siinomega/RuShell.git
   ```
2. Navigate to the project directory :

   ```bash
   cd RuShell
   ```
---
## ⚡ Contribuer

If you would like to contribute to this project, feel free to submit a pull request or report issues. Any contribution is welcome!

---
## 📄 Licence

This project is licensed under the [MIT](LICENSE).
---

Thank you for checking out this project! Feel free to reach out to me if you have any questions or suggestions.

