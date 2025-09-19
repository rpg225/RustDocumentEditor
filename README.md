---

# Rust Document Editor

A simple, graphical desktop document editor built with Rust, primarily serving as a learning and testing ground for Rust's GUI capabilities using the `druid` UI toolkit. This project demonstrates fundamental concepts of desktop application development in Rust, providing a basic text editing experience.

## ✨ Features

*   **Basic Text Editing:** Type, delete, and modify text within the editor window.
*   **Graphical User Interface (GUI):** An intuitive user interface powered by the `druid` UI toolkit.
*   **Built with Rust:** Leverages the performance and safety features of the Rust programming language.
*   **Cross-Platform Potential:** As a Rust and Druid application, it can be compiled and run on various operating systems.

## 💻 Technologies Used

*   **Rust:** The programming language providing memory safety and performance.
*   **Druid:** A data-oriented UI toolkit for Rust, used for constructing the graphical interface.

## ⚙️ Local Setup

To get this document editor up and running on your local machine, follow these steps:

### Prerequisites

*   **Rust Toolchain:** You need to have the Rust programming language and its package manager (`cargo`) installed. If you don't, you can install it via `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    Follow the on-screen instructions.

### 1. Clone the Repository

```bash
git clone https://github.com/rpg225/RustDocumentEditor.git
cd RustDocumentEditor
```

### 2. Run the Application

Once inside the project directory, you can compile and run the application using Cargo:

```bash
cargo run
```

This command will download dependencies, compile the project, and then launch the graphical editor window.

## 🚀 How to Use

After launching the application:

1.  **Start Typing:** Begin typing directly into the editor area.
2.  **Edit Text:** Use your keyboard to add, delete, or modify characters.
3.  *(Further features like "Open" or "Save" would be listed here if implemented.)*
