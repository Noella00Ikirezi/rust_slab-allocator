# Architecture du Slab Allocator

## Vue d'ensemble

Notre slab allocator est compose de deux structures principales :
- **Slab** : une page memoire decoupee en blocs
- **SlabAllocator** : gestionnaire de plusieurs slabs

## Schema de l'architecture

```
                    SlabAllocator
                         |
    +--------+--------+--------+--------+--------+
    |        |        |        |        |        |
    v        v        v        v        v        v
  Slab     Slab     Slab     Slab     Slab     Slab
  64B      64B      128B     256B     512B    1024B
```

## Structure d'une Slab

Une slab est une page de 4096 octets decoupee en blocs :

```
+--------------------------------------------------+
|                    Slab (4096 octets)            |
+--------------------------------------------------+
| Block 0 | Block 1 | Block 2 | ... | Block N      |
+--------------------------------------------------+
    |          |
    v          v
 [next] --> [next] --> [next] --> NULL
    ^
    |
 free_list (tete de liste)
```

### Exemple pour une slab de 64 octets

```
Taille slab: 4096 octets
Taille bloc: 64 octets
Nombre de blocs: 4096 / 64 = 64 blocs

+----+----+----+----+----+----+----+----+
| B0 | B1 | B2 | B3 | B4 | B5 | .. | B63|
+----+----+----+----+----+----+----+----+
  64   64   64   64   64   64        64  octets
```

## Liste chainee des blocs libres

Les blocs libres sont relies entre eux :

```
Etat initial (tous libres):
free_list -> B0 -> B1 -> B2 -> B3 -> ... -> B63 -> NULL

Apres allocation de B0:
free_list -> B1 -> B2 -> B3 -> ... -> B63 -> NULL

Apres liberation de B0:
free_list -> B0 -> B1 -> B2 -> B3 -> ... -> B63 -> NULL
```

## Les differentes tailles

Notre allocateur gere 5 tailles de blocs :

| Slab    | Taille bloc | Blocs par slab | Usage                    |
|---------|-------------|----------------|--------------------------|
| slab_64 | 64 octets   | 64 blocs       | Petites structures       |
| slab_128| 128 octets  | 32 blocs       | Structures moyennes      |
| slab_256| 256 octets  | 16 blocs       | Buffers                  |
| slab_512| 512 octets  | 8 blocs        | Gros objets              |
| slab_1024| 1024 octets| 4 blocs        | Tres gros objets         |

## Algorithme d'allocation

```
fonction alloc(taille):
    1. Trouver la slab avec la bonne taille de bloc
       (plus petite taille >= taille demandee)

    2. Si free_list n'est pas vide:
       - Prendre le premier bloc
       - Mettre a jour free_list
       - Retourner le pointeur

    3. Sinon:
       - Retourner NULL (plus de memoire)
```

## Algorithme de liberation

```
fonction dealloc(pointeur):
    1. Trouver quelle slab contient ce pointeur

    2. Ajouter le bloc en tete de free_list:
       - bloc.next = free_list
       - free_list = bloc
```

## Complexite

| Operation   | Complexite |
|-------------|------------|
| Allocation  | O(1)       |
| Liberation  | O(1)       |
| Recherche   | O(n) slabs |

## Comparaison avec malloc

```
malloc classique:
- Recherche d'un bloc libre de bonne taille
- Peut fragmenter la memoire
- Overhead de metadata par allocation

Slab allocator:
- Allocation immediate (premier bloc libre)
- Pas de fragmentation (taille fixe)
- Metadata minimale
```

## Integration dans un projet

Pour utiliser le slab allocator dans un projet no_std :

```rust
use slab_allocator::SlabAllocator;

static mut ALLOCATOR: SlabAllocator = SlabAllocator::new();

// Allocation
let ptr = ALLOCATOR.alloc(100); // utilise slab_128

// Liberation
ALLOCATOR.dealloc(ptr, 100);
```
