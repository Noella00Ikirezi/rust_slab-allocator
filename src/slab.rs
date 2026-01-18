//! Slab Allocator - Allocateur de blocs de taille fixe

use core::ptr::NonNull;
use core::mem;

const SLAB_SIZE: usize = 4096; // taille dune slab (4Ko)

/// Bloc libre dans une slab (liste chainee)
struct FreeBlock {
    next: Option<NonNull<FreeBlock>>,
}

/// Une slab contient des blocs de taille fixe
pub struct Slab {
    block_size: usize,
    free_list: Option<NonNull<FreeBlock>>,
    memory: [u8; SLAB_SIZE],
    free_count: usize,
}

impl Slab {
    /// cree une nouvelle slab pour des blocs de taille donnee
    pub const fn new(block_size: usize) -> Self {
        Slab {
            block_size,
            free_list: None,
            memory: [0; SLAB_SIZE],
            free_count: 0,
        }
    }

    /// Initialise la slab en decoupant la memoire en blocs
    pub fn init(&mut self) {
        // on calcule la taille minimum (au moins la taille dun FreeBlock)
        let block_size = self.block_size;
        let min_size = mem::size_of::<FreeBlock>();
        let final_block_size;
        if block_size > min_size {
            final_block_size = block_size;
        } else {
            final_block_size = min_size;
        }

        // combien de blocs on peut mettre
        let num_blocks = SLAB_SIZE / final_block_size;

        self.free_list = None;
        self.free_count = num_blocks;

        // on cree la liste chainee des blocs libres
        let mut i = 0;
        while i < num_blocks {
            let ptr = unsafe {
                self.memory.as_mut_ptr().add(i * final_block_size) as *mut FreeBlock
            };
            unsafe {
                (*ptr).next = self.free_list;
                self.free_list = Some(NonNull::new_unchecked(ptr));
            }
            i = i + 1;
        }
    }

    /// Alloue un bloc depuis cette slab
    pub fn alloc(&mut self) -> Option<NonNull<u8>> {
        // on verifie si il y a un bloc libre
        if self.free_list.is_none() {
            return None;
        }

        let block = self.free_list.unwrap();
        unsafe {
            self.free_list = (*block.as_ptr()).next;
            self.free_count = self.free_count - 1;
            Some(NonNull::new_unchecked(block.as_ptr() as *mut u8))
        }
    }

    /// Libere un bloc dans cette slab
    pub fn dealloc(&mut self, ptr: NonNull<u8>) {
        let block = ptr.as_ptr() as *mut FreeBlock;
        unsafe {
            (*block).next = self.free_list;
            self.free_list = Some(NonNull::new_unchecked(block));
            self.free_count = self.free_count + 1;
        }
    }

    /// Verifie si un pointeur appartient a cette slab
    pub fn contains(&self, ptr: *const u8) -> bool {
        let start = self.memory.as_ptr();
        let end = unsafe { start.add(SLAB_SIZE) };
        if ptr >= start && ptr < end {
            return true;
        }
        return false;
    }

    /// Retourne le nombre de blocs libres
    pub fn free_count(&self) -> usize {
        self.free_count
    }
}

use core::alloc::Layout;

/// Allocateur slab avec plusieurs tailles de blocs
pub struct SlabAllocator {
    slabs_64: Slab,    // pour blocs <= 64 octets
    slabs_128: Slab,   // pour blocs <= 128 octets
    slabs_256: Slab,   // pour blocs <= 256 octets
    slabs_512: Slab,   // pour blocs <= 512 octets
    slabs_1024: Slab,  // pour blocs <= 1024 octets
    initialized: bool,
}

impl SlabAllocator {
    /// Cree un nouvel allocateur slab
    pub const fn new() -> Self {
        SlabAllocator {
            slabs_64: Slab::new(64),
            slabs_128: Slab::new(128),
            slabs_256: Slab::new(256),
            slabs_512: Slab::new(512),
            slabs_1024: Slab::new(1024),
            initialized: false,
        }
    }

    /// Initialise toutes les slabs
    pub fn init(&mut self) {
        if self.initialized == false {
            self.slabs_64.init();
            self.slabs_128.init();
            self.slabs_256.init();
            self.slabs_512.init();
            self.slabs_1024.init();
            self.initialized = true;
        }
    }

    /// Trouve la slab appropriee pour une taille donnee
    fn get_slab_for_size(&mut self, size: usize) -> Option<&mut Slab> {
        if size <= 64 {
            return Some(&mut self.slabs_64);
        } else if size <= 128 {
            return Some(&mut self.slabs_128);
        } else if size <= 256 {
            return Some(&mut self.slabs_256);
        } else if size <= 512 {
            return Some(&mut self.slabs_512);
        } else if size <= 1024 {
            return Some(&mut self.slabs_1024);
        } else {
            return None; // trop grand pour nos slabs
        }
    }

    /// Trouve la slab qui contient un pointeur
    fn find_slab_for_ptr(&mut self, ptr: *const u8) -> Option<&mut Slab> {
        if self.slabs_64.contains(ptr) {
            return Some(&mut self.slabs_64);
        } else if self.slabs_128.contains(ptr) {
            return Some(&mut self.slabs_128);
        } else if self.slabs_256.contains(ptr) {
            return Some(&mut self.slabs_256);
        } else if self.slabs_512.contains(ptr) {
            return Some(&mut self.slabs_512);
        } else if self.slabs_1024.contains(ptr) {
            return Some(&mut self.slabs_1024);
        } else {
            return None;
        }
    }

    /// Alloue de la memoire
    pub fn allocate(&mut self, layout: Layout) -> Option<NonNull<u8>> {
        // init si pas encore fait
        if self.initialized == false {
            self.init();
        }

        // on prend le max entre size et align
        let size = layout.size();
        let align = layout.align();
        let final_size;
        if size > align {
            final_size = size;
        } else {
            final_size = align;
        }

        // on cherche la bonne slab
        let slab = self.get_slab_for_size(final_size);
        if slab.is_none() {
            return None;
        }
        return slab.unwrap().alloc();
    }

    /// Libere de la memoire
    pub fn deallocate(&mut self, ptr: NonNull<u8>, _layout: Layout) {
        // on trouve quelle slab contient ce pointeur
        let slab = self.find_slab_for_ptr(ptr.as_ptr());
        if slab.is_some() {
            slab.unwrap().dealloc(ptr);
        }
        // sinon on fait rien (cest pas notre memoire)
    }
}

/// Allocateur global statique
pub static mut ALLOCATOR: SlabAllocator = SlabAllocator::new();
