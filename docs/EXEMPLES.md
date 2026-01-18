# Exemples d'utilisation

## Exemple 1 : Allocation basique

```rust
use slab_allocator::{Slab, SlabAllocator};

fn main() {
    // Creer une slab avec des blocs de 64 octets
    let mut slab = Slab::new(64);

    // Allouer un bloc
    let ptr = slab.alloc();

    if ptr.is_null() {
        println!("Plus de memoire !");
    } else {
        println!("Bloc alloue a l'adresse: {:p}", ptr);

        // Utiliser le bloc...

        // Liberer le bloc
        slab.dealloc(ptr);
    }
}
```

## Exemple 2 : Utiliser SlabAllocator

```rust
use slab_allocator::SlabAllocator;

fn main() {
    let mut allocator = SlabAllocator::new();

    // Demander 100 octets -> utilise slab_128
    let ptr = allocator.alloc(100);

    // Demander 50 octets -> utilise slab_64
    let ptr2 = allocator.alloc(50);

    // Demander 500 octets -> utilise slab_512
    let ptr3 = allocator.alloc(500);

    // Liberer
    allocator.dealloc(ptr, 100);
    allocator.dealloc(ptr2, 50);
    allocator.dealloc(ptr3, 500);
}
```

## Exemple 3 : Stocker des structures

```rust
use slab_allocator::Slab;
use core::ptr;

// Structure de 24 octets
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

fn main() {
    // Creer slab pour des blocs de 64 octets (assez pour Point3D)
    let mut slab = Slab::new(64);

    // Allouer
    let ptr = slab.alloc() as *mut Point3D;

    if !ptr.is_null() {
        // Ecrire dans le bloc
        unsafe {
            ptr::write(ptr, Point3D { x: 1.0, y: 2.0, z: 3.0 });

            // Lire
            let point = ptr::read(ptr);
            println!("Point: ({}, {}, {})", point.x, point.y, point.z);
        }

        // Liberer
        slab.dealloc(ptr as *mut u8);
    }
}
```

## Exemple 4 : Verifier si un pointeur appartient a la slab

```rust
use slab_allocator::Slab;

fn main() {
    let mut slab = Slab::new(128);

    let ptr = slab.alloc();

    // Verifier si le pointeur est dans cette slab
    if slab.contains(ptr) {
        println!("Le pointeur appartient a cette slab");
    }

    // Un pointeur externe
    let externe: *mut u8 = 0x1234 as *mut u8;
    if !slab.contains(externe) {
        println!("Le pointeur externe n'appartient pas a la slab");
    }
}
```

## Exemple 5 : Plusieurs allocations

```rust
use slab_allocator::Slab;

fn main() {
    let mut slab = Slab::new(64);
    let mut pointeurs = Vec::new();

    // Allouer plusieurs blocs
    for i in 0..10 {
        let ptr = slab.alloc();
        if !ptr.is_null() {
            println!("Allocation {}: {:p}", i, ptr);
            pointeurs.push(ptr);
        }
    }

    println!("Blocs alloues: {}", pointeurs.len());

    // Liberer tous les blocs
    for ptr in pointeurs {
        slab.dealloc(ptr);
    }
}
```

## Exemple 6 : Utilisation en no_std

```rust
#![no_std]
#![no_main]

use slab_allocator::SlabAllocator;

static mut ALLOCATOR: SlabAllocator = SlabAllocator::new();

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Allouer de la memoire
        let ptr = ALLOCATOR.alloc(256);

        if !ptr.is_null() {
            // Utiliser la memoire...

            // Liberer
            ALLOCATOR.dealloc(ptr, 256);
        }
    }

    loop {}
}
```

## Notes importantes

1. **Thread safety** : Notre implementation n'est pas thread-safe.
   Pour du multi-thread, il faudrait ajouter des locks.

2. **Tailles supportees** : 64, 128, 256, 512, 1024 octets.
   Les tailles plus grandes ne sont pas gerees.

3. **Fragmentation** : Le slab allocator evite la fragmentation
   car tous les blocs ont la meme taille.

4. **Performance** : Allocation et liberation en O(1).
