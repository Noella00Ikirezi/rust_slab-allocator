# Writeup : Le SLUB Allocator de Linux

## Introduction

Le SLUB (Simple List Based) est l'allocateur memoire utilise par defaut dans le noyau Linux depuis la version 2.6.23. Il remplace l'ancien SLAB allocator.

## Pourquoi un slab allocator ?

Dans un systeme d'exploitation, on alloue souvent des objets de meme taille (structures, buffers, etc). Malloc classique est pas optimal pour ca car :
- Fragmentation de la memoire
- Overhead pour chaque allocation
- Lent pour les petites allocations frequentes

Le slab allocator resout ces problemes en pre-allouant des blocs de taille fixe.

## Comment ca marche ?

### Principe de base

1. On decoupe la memoire en "slabs" (pages de 4Ko en general)
2. Chaque slab contient des objets de meme taille
3. Les objets libres sont lies dans une liste chainee
4. Allocation = prendre le premier element de la liste
5. Liberation = remettre l'element dans la liste

### Structure du SLUB

```
+------------------+
|   kmem_cache     |  <- cache pour un type d'objet
+------------------+
        |
        v
+------------------+
|      Slab        |  <- page de memoire
| +----+ +----+    |
| |obj | |obj | ...|  <- objets de taille fixe
| +----+ +----+    |
+------------------+
```

### Les caches

Le SLUB utilise des "caches" pour chaque taille d'objet :
- kmalloc-64 pour les objets <= 64 octets
- kmalloc-128 pour les objets <= 128 octets
- kmalloc-256 pour les objets <= 256 octets
- etc.

## Differences entre SLAB et SLUB

| SLAB | SLUB |
|------|------|
| 3 listes (full, partial, empty) | 1 seule liste (partial) |
| Metadata separee | Metadata dans la page |
| Plus de code | Code plus simple |
| Plus de memoire utilisee | Moins de memoire |

## Notre implementation

Dans notre projet, on a fait une version simplifiee :

```rust
pub struct Slab {
    block_size: usize,        // taille des blocs
    free_list: Option<...>,   // liste des blocs libres
    memory: [u8; 4096],       // memoire de la slab
    free_count: usize,        // nombre de blocs libres
}
```

On a 5 slabs pour differentes tailles :
- slabs_64 (blocs de 64 octets)
- slabs_128 (blocs de 128 octets)
- slabs_256 (blocs de 256 octets)
- slabs_512 (blocs de 512 octets)
- slabs_1024 (blocs de 1024 octets)

## Avantages du SLUB

1. **Rapidite** : O(1) pour alloc et free
2. **Pas de fragmentation** : objets de meme taille
3. **Cache friendly** : objets contigus en memoire
4. **Simple** : moins de code = moins de bugs

## Utilisation dans Linux

On peut voir les caches SLUB avec :
```bash
cat /proc/slabinfo
```

Ou avec plus de details :
```bash
slabtop
```

## Conclusion

Le SLUB est un allocateur efficace pour les objets de taille fixe. Il est utilise partout dans le noyau Linux pour les structures internes (inodes, dentries, task_struct, etc).

Notre implementation est basique mais montre les concepts principaux : liste chainee de blocs libres et plusieurs caches pour differentes tailles.

## References

- Documentation Linux : Documentation/vm/slub.rst
- Code source : mm/slub.c
- LWN article : https://lwn.net/Articles/229984/
