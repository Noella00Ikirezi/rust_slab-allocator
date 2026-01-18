# Slab Allocator

Projet Rust 4A - ESGI

## Auteurs

- Noella IKIREZI
- Siham BOUDJAIDI

## Description

Implementation d'un slab allocator en Rust pour environnement no_std.
Le slab allocator est un allocateur memoire optimise pour les objets de taille fixe.

## Structure du projet

```
slab-allocator/
├── src/
│   ├── lib.rs          # Point d'entree
│   └── slab.rs         # Implementation du slab allocator
├── writeup/
│   └── SLUB_Linux.md   # Writeup sur le SLUB de Linux
├── Authors.md
├── Cargo.toml
└── README.md
```

## Fonctionnalites

- **Slab** : structure qui decoupe une page memoire en blocs de taille fixe
- **SlabAllocator** : allocateur avec plusieurs slabs pour differentes tailles (64, 128, 256, 512, 1024 octets)
- **Liste chainee** : gestion des blocs libres en O(1)
- **Compatible no_std** : peut etre utilise dans un environnement sans bibliotheque standard

## Comment ca marche

1. La memoire est decoupee en "slabs" de 4Ko
2. Chaque slab contient des blocs de taille fixe
3. Les blocs libres sont lies dans une liste chainee
4. Allocation = prendre le premier bloc libre
5. Liberation = remettre le bloc dans la liste

## Build et Tests

```bash
# Lancer les tests
cargo test

# Build
cargo build
```

## Tests disponibles

| Test | Description |
|------|-------------|
| test_slab_alloc_dealloc | Verifie alloc et dealloc basique |
| test_slab_contains | Verifie si un pointeur est dans la slab |
| test_allocator_different_sizes | Test avec differentes tailles |

## Integration avec FAT32

Le slab allocator a ete integre dans le projet FAT32 (projet precedent).
Voir `fat32-exam/src/allocator.rs` pour l'integration.

## Writeup SLUB

Le dossier `writeup/` contient une documentation sur le SLUB allocator de Linux,
qui est l'allocateur utilise par defaut dans le noyau Linux.

## Git Bundle

Pour la soumission :
```bash
git bundle create slab-allocator.bundle --all
```
