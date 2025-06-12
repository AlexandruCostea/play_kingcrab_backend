# ♟️ PlayKingCrab Backend

This project provides a lightweight Rust-based **Actix Web API** wrapper around the [King Crab](https://github.com/AlexandruCostea/KingCrab) chess engine. It exposes endpoints to evaluate positions using neural network-based strategies like CNN and HalfKA, designed for integration with web frontends or other clients.

---

## 🚀 Features

- 🧠 **Evaluation Strategies**: Supports switching between CNN and HalfKA neural network evaluators.
- 📦 **REST API**:
  - `POST /get_best_move` — send an engine type string ("cnn" or "halfka") and a FEN and receive the best move from that engine's perspective
- 🔐 Thread-safe engine instances via `Mutex`
- 🧠 Uses ONNX models exported from [`kingcrab-evaluation`](https://github.com/AlexandruCostea/kingcrab-evaluation)

---

## 📦 Setup

### 🔧 Prerequisites
- Rust (stable)
- ONNX models trained from [`kingcrab-evaluation`](https://github.com/AlexandruCostea/kingcrab-evaluation)

### Importing the chess engine library

```bash
git clone https://github.com/AlexandruCostea/KingCrab.git
```

### Add the project to the dependencies

- In Cargo.toml add the following line in the \[dependencies\] section
```bash
king_crab = { path = "<path to cloned repository>" }
```

### Setting up the environment
- Create a .env file at project root level
- Fill in the following entries:
```bash
CNN_MODEL_PATH=<path to cnn onnx file>
CNN_DEPTH=<desired depth when using cnn evaluator>

HALFKA_MODEL_PATH=<path to halfka folder>
HALFKA_DEPTH=<desired depth when using halfka evaluator>

FRONTEND_URL=<url of the frontend to be allowed by cors>
```
### Build the project
```bash
cargo build --release
```

### Run the project
```bash
cargo run --release
```

