# 💼 Rust Risk Management Tool

This project is a simple CLI-based tool developed in Rust for managing trading risk in both **Futures** and **Spot** markets. It helps calculate trade size, potential loss, potential profit, and risk-to-reward (R:R) ratio based on user input.

## 📊 Features

- Two options:
  - [1] Futures Case Management
  - [2] Spot Case Management
- Calculates:
  - Trade amount based on risk percentage and stop-loss
  - Estimated profit based on target price
  - R:R (Risk to Reward) ratio
  - Final balance if trade is successful
- Basic risk validation:
  - Warnings for high risk percentages
  - Error if input values are invalid (e.g., negative balance)

## 🚀 How to Run

1. Make sure Rust is installed on your system. If not:  
   [https://rustup.rs/](https://rustup.rs/)

2. Clone or download the project files.

3. Compile and run the project using:

```bash
cargo run
