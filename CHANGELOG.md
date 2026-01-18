# Changelog

Historique des modifications du projet.

## [1.0.0] - 2025

### Ajoute

- Structure `Slab` pour gerer une page memoire decoupee en blocs
- Structure `SlabAllocator` avec support de plusieurs tailles
- Tailles supportees : 64, 128, 256, 512, 1024 octets
- Liste chainee pour gestion des blocs libres
- Methode `contains()` pour verifier si un pointeur appartient a une slab
- Tests unitaires complets
- Documentation sur le SLUB de Linux (writeup)
- Integration conceptuelle avec le projet FAT32

### Caracteristiques techniques

- Compatible no_std
- Allocation en O(1)
- Liberation en O(1)
- Pas de fragmentation pour objets de meme taille

### Tests

- `test_slab_alloc_dealloc` : allocation et liberation basique
- `test_slab_contains` : verification d'appartenance des pointeurs
- `test_allocator_different_sizes` : test avec differentes tailles

## Auteurs

- Noella IKIREZI
- Siham BOUDJAIDI
