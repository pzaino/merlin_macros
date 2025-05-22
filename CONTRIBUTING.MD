# Contributing to RISC OS Merlin

Hello and welcome! 🎉  
Thank you for your interest in contributing to **RISC OS Merlin** — a modern reimagining of the RISC OS kernel, written in Rust.

We want this project to be a healthy, productive, and rewarding experience for everyone involved — new and experienced alike.

---

## 💡 Who Can Contribute?

Anyone!

Whether you're:

- a seasoned OS developer,
- a Rust expert,
- someone curious about kernel development,
- a retro computing enthusiast,
- or just want to help with documentation, tests, or tooling...

You're welcome to contribute! 🙌

We especially encourage contributions in areas like:

- Documentation & tutorials
- Platform ports
- Bug reports and testing
- HAL code
- Modules and drivers
- New syscall designs
- Performance or memory optimization
- Emulator integration or compatibility fixes

> 🧠 *For core kernel contributions, a strong understanding of Rust and operating systems is expected.*

---

## 🚧 Code Style & Guidelines

- We follow idiomatic **Rust (`#![no_std]`)** style wherever possible.
- Unsafe Rust should be avoided unless strictly necessary, and must be well-documented and justified.
- All code **must compile on stable or nightly Rust** as specified in the build section.
- Commit messages should be clear and follow the format:  
  `module: short description`, e.g. `mmu: fix page table alignment`

If you’re unsure — open a draft PR or ask via Discussion or Issues. We’ll be happy to help!

---

## 🧭 Ground Rules (aka "Be Excellent to Each Other")

We’ve had decades of RISC OS history — and some rough patches.

We want Merlin to be a positive, welcoming space for collaboration, learning, and progress.

So here’s the one rule we care deeply about:

> **Treat each other with respect, patience, and good faith.**

This means:

- No harassment, insults, trolling, or personal attacks.
- Don’t nitpick for the sake of it.
- Disagree **respectfully** and focus on the idea, not the person.
- Be helpful, especially to newcomers.

If you can’t follow this, you may be removed from discussions or blocked from the project.

We follow the our [Code of Conduct](.github/CODE_OF_CONDUCT.md), so please check it out.

---

## 🔎 Find something to work on

- Check the sources/documents for errors, open issues, or `TODO` comments in the code.
- Look for issues tagged with `good first issue` or `help wanted`.
- A fix for a bug you found is always welcome!
- Or if you have a cool idea, just open a new issue to discuss it!

## 🛠️ Submitting Changes

### 1. Fork the repository and clone your fork

```sh
git clone https://github.com/YOUR_USERNAME/merlin_macros.git
cd merlin_macros
```

### 2. Create a branch

```sh
git checkout -b my-cool-feature
```

### 3. Make your changes and commit

Use clear commit messages and split unrelated changes into separate commits.

### 4. Push to your fork and open a Pull Request

Go to [GitHub](https://github.com/pzaino/risc-os-merlin/pulls) and open a PR against the `develop` branch. Use the template provided.

## 🪄 How Branches Work in the Merlin Repository

- **`main`** — The **staging branch**. This contains the latest tested code intended for wider testing and community feedback.
- **`release`** — The **stable branch**. This contains the latest production-ready code for users. All tagged versions (`v0.x.x`) come from here.
- **`develop`** — The **active development branch**. This is where new features, bug fixes, and experimental work are merged first.

---

### 🔁 Release Cycle

1. Code is developed in **`develop`**, tested locally or in CI.
2. Once features are complete, `develop` is merged into **`main`** for integration and wider testing.
3. After approximately **one month of stability testing** on `main`, it is merged into **`release`** and a new version is tagged (e.g. `v0.1.0`).
4. A new cycle begins: `develop` is merged into `main`, and development continues.

This model ensures:

- Active work stays isolated
- Mainline testing is consistent
- Releases are stable and intentional

---

## 🐛 Reporting Bugs & Ideas

Please use [GitHub Issues](https://github.com/pzaino/risc-os-merlin/issues) to:

- Report a bug (please include platform/arch, toolchain version, logs if possible)
- Suggest new features
- Propose architectural changes

---

## 🤝 Contributor License

All contributions must be under the project's license (MPL 2.0 + additional conditions).  
By contributing, you agree to license your work under these terms.

---

## ❤️ Thank You

Whether you're fixing typos, filing bugs, sharing ideas, or building entire subsystems — thank you for being part of the journey!

Let’s build something modern, modular, and magical. 🧙

— Paolo
