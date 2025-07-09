# 🌍 Hello World - Anchor PDA Example

This is a simple Solana smart contract written using the [Anchor framework](https://book.anchor-lang.com/). It demonstrates:

✅ Storing a greeting string on-chain  
✅ Updating that greeting  
✅ Using PDAs (Program Derived Addresses) with seeds and bumps

---

## 🏗️ What It Does

- Creates an account (PDA) for storing a greeting (e.g., `"Hello"`).
- Allows updating the greeting to a new value.

---

## 📁 Program Structure

### Instructions

- `initialize(greeting: String)`: Initializes a new greeting account (PDA).
- `update(new_greeting: String)`: Updates the greeting in the PDA.

---

## 🧠 PDA Design

The PDA is derived using:

```rust
seeds = [b"greeting", user.key().as_ref()]
