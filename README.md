# Slab Allocator Implementation

## PROJET 2: Slab Allocator (Cours Rust 4A)

**2 personnes max**

---

## Objectifs

1. Implementer un slab allocator en Rust
2. Rediger un writeup sur le SLUB allocator de Linux

---

## Contraintes

- `no_std` obligatoire
- `alloc` crate autorise
- Correction sur Linux
- Soumission via **git bundle** sur myges
- **Commits reguliers obligatoires**
- Code emprunte non credite = 0
- Tests obligatoires
- Fichier `Authors.md` obligatoire

---

## Structure du projet

```
slab-allocator/
├── Cargo.toml
├── Authors.md
├── README.md
├── src/
│   ├── lib.rs
│   └── slab.rs
└── writeup/
    └── SLUB_Linux.md
```

---

## Build & Test

```bash
# Lancer les tests
cargo test

# Build no_std
cargo build --target x86_64-unknown-none
```

---

## Creer le git bundle pour soumission

```bash
git bundle create slab-allocator.bundle --all
git bundle verify slab-allocator.bundle
```

---

## Auteur

**Noella IKIREZI** - ESGI 4A
