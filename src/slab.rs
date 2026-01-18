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
