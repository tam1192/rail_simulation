---
name: awesome-cursorrules
description: Installs and maintains Cursor Project Rules from PatrickJS/awesome-cursorrules. Use when adding, updating, or choosing .cursor/rules/*.mdc files, when the user mentions awesome-cursorrules, cursorrules, or project rules, or when the stack (Vue, Rust, Tailwind, Git) needs catalog guidance.
---

# Awesome Cursor Rules

Source catalog: https://github.com/PatrickJS/awesome-cursorrules

That repo is a list of `.mdc` Project Rules, not a single always-on prompt. Do not copy the whole catalog. Pick files that match this repo, put them in `.cursor/rules/`, and keep them scoped with `globs`.

## This repository

Stack: Vite + Vue 3 (Composition API) + TypeScript + Tailwind v4 + Rust WASM.

Already installed under `.cursor/rules/` (see [SOURCES.md](SOURCES.md)):

| File | Role |
|---|---|
| `project-stack.mdc` | Local overlay. Wins over generic catalog advice. |
| `vue.mdc` | Vue 3 patterns |
| `vue3-composition-api.mdc` | Composition API / Vite |
| `rust-general.mdc` | Idiomatic Rust (not Solana) |
| `tailwind.mdc` | Utility CSS |
| `clean-code.mdc` | Naming, SRP, DRY |
| `git-conventional-commits.mdc` | Conventional Commits |

## How to add another rule

1. Browse https://github.com/PatrickJS/awesome-cursorrules/tree/main/rules
2. Download one `.mdc` that matches the files you are editing
3. Save it as `.cursor/rules/<short-name>.mdc`
4. Keep YAML frontmatter. Prefer `alwaysApply: false` and tight `globs`
5. Skip rules for stacks this repo does not use (Next.js, Nuxt, Pinia, DaisyUI, shadcn, Solana, Angular, …)
6. Append the source URL to [SOURCES.md](SOURCES.md)

If a catalog rule conflicts with `project-stack.mdc`, follow `project-stack.mdc`.

## Do not

- Vendor `rules/rust.mdc` from the catalog (it is Anchor/Solana)
- Vendor empty stubs such as `typescript-vuejs-cursorrules-prompt-file.mdc`
- Set `alwaysApply: true` on catalog files
- Duplicate a rule that is already in `.cursor/rules/`
