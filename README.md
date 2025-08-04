# Shali — Encrypted CLI Notes (Work in Progress)

**Shali** is a command-line notes app built in Rust, focused on **local-first encrypted note-taking**. It's still super early in development, so bugs and missing features are expected — but the core idea is a CLI that allows the user to write and manage notes in their terminal, and encrypt or decrypt them with a single command.

---

## 🔐 Encryption First

* Uses [simple-crypt](https://crates.io/crates/simple-crypt) (AES-GCM-SIV-256 + Argon2)
* All notes live in a local `shali/` directory
* You can encrypt the entire directory into a `.dir` blob and decrypt it back when needed
* Notes are totally inaccessible unless decrypted with the correct password

---

## ✏️ Current Commands

* `add` — Create a new note and open it in `vim`
* `edit` — Open an existing note in `vim`
* `delete` — Delete a note
* `encrypt` — Encrypt the whole `shali/` notes folder
* `decrypt` — Decrypt your notes folder back into plaintext

All of these are driven by simple terminal prompts for now.

---

## 🚧 Development Notes

Shali is still in early development.

* No argument parsing yet (but [`clap`](https://crates.io/crates/clap) is planned)
* Uses [`reedline`](https://crates.io/crates/reedline) for interactive prompts — though that might get scaled back
* No graceful error handling (yet) — for example, trying to edit or delete a file that doesn’t exist might just silently fail or panic
* You can’t run it globally like `nvim` or `yazi` yet — you'll need to clone the repo and run with `cargo run` during development

---

## 🛠 How to Try It

```bash
git clone https://github.com/Ronxvier/Shali
cd shali
cargo run
```

You’ll get a prompt where you can try commands like `add`, `encrypt`, etc.

---

## 🧩 What's Next

* Major bug fixes
* Adding `clap` to support actual CLI arguments (`shali add note.txt`, etc.)
* Better error messages and file checks
* Possibly adding config options or fallback behaviors
* Global install support (`cargo install shali` and `$PATH` support)

---
