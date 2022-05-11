pub mod mapping_record;
pub mod page_alloc;
pub mod translation_table;

use core::{fmt, num::NonZeroUsize};
use crate::{types::paging::{MemoryRegion, AttributeFields, MMIODescriptor, MemAttributes, AccessPermissions, PageAddress}, drivers::pi4b::memory};
use super::{Address, Physical, Virtual};

#[derive(Debug)]
pub enum MMUEnableError {
    AlreadyEnabled,
    Other(&'static str),
}

pub trait MMU {
    unsafe fn enable_mmu_and_caching(
        &self,
        phys_tables_base_addr: Address<Physical>,
    ) -> Result<(), MMUEnableError>;

    fn is_enabled(&self) -> bool;
}

pub struct TranslationGranule<const GRANULE_SIZE: usize>;

pub struct AddressSpace<const AS_SIZE: usize>;

pub trait AssociatedTranslationTable {
    type TableStartFromTop;
    type TableStartFromBottom;
}

// TODO: I think some of these need pi 4b / coupled to it. Again if its on aarch64/pi/rockpi, use it here instead

fn kernel_init_mmio_va_allocator() {
    let region = memory::virt_mmio_remap_region();

    page_alloc::kernel_mmio_va_allocator().lock(|allocator| allocator.initialize(region));
}

unsafe fn kernel_map_at_unchecked(
    name: &'static str,
    virt_region: &MemoryRegion<Virtual>,
    phys_region: &MemoryRegion<Physical>,
    attr: &AttributeFields,
) -> Result<(), &'static str> {
    memory::kernel_translation_tables()
        .write(|tables| tables.map_at(virt_region, phys_region, attr))?;

    kernel_add_mapping_record(name, virt_region, phys_region, attr);

    Ok(())
}

fn try_kernel_virt_addr_to_phys_addr(
    virt_addr: Address<Virtual>,
) -> Result<Address<Physical>, &'static str> {
    memory::kernel_translation_tables()
        .read(|tables| tables.try_virt_addr_to_phys_addr(virt_addr))
}

impl fmt::Display for MMUEnableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MMUEnableError::AlreadyEnabled => write!(f, "MMU is already enabled"),
            MMUEnableError::Other(x) => write!(f, "{}", x),
        }
    }
}

impl<const GRANULE_SIZE: usize> TranslationGranule<GRANULE_SIZE> {
    pub const SIZE: usize = Self::size_checked();
    pub const MASK: usize = Self::SIZE - 1;
    pub const SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    const fn size_checked() -> usize {
        assert!(GRANULE_SIZE.is_power_of_two());

        GRANULE_SIZE
    }
}

impl<const AS_SIZE: usize> AddressSpace<AS_SIZE> {
    pub const SIZE: usize = Self::size_checked();
    pub const SIZE_SHIFT: usize = Self::SIZE.trailing_zeros() as usize;

    const fn size_checked() -> usize {
        assert!(AS_SIZE.is_power_of_two());

        Self::arch_address_space_size_sanity_checks();

        AS_SIZE
    }
}

pub fn kernel_add_mapping_record(
    name: &'static str,
    virt_region: &MemoryRegion<Virtual>,
    phys_region: &MemoryRegion<Physical>,
    attr: &AttributeFields,
) {
    if let Err(x) = mapping_record::kernel_add(name, virt_region, phys_region, attr) {
        // warn!("{}", x);
    }
}

pub unsafe fn kernel_map_mmio(
    name: &'static str,
    mmio_descriptor: &MMIODescriptor,
) -> Result<Address<Virtual>, &'static str> {
    let phys_region = MemoryRegion::from(*mmio_descriptor);
    let offset_into_start_page = mmio_descriptor.start_addr().offset_into_page();

    // Check if an identical region has been mapped for another driver. If so, reuse it.
    let virt_addr = if let Some(addr) =
        mapping_record::kernel_find_and_insert_mmio_duplicate(mmio_descriptor, name)
    {
        addr
    // Otherwise, allocate a new region and map it.
    } else {
        let num_pages = match NonZeroUsize::new(phys_region.num_pages()) {
            None => return Err("Requested 0 pages"),
            Some(x) => x,
        };

        let virt_region =
            page_alloc::kernel_mmio_va_allocator().lock(|allocator| allocator.alloc(num_pages))?;

        kernel_map_at_unchecked(
            name,
            &virt_region,
            &phys_region,
            &AttributeFields {
                mem_attributes: MemAttributes::Device,
                acc_perms: AccessPermissions::ReadWrite,
                execute_never: true,
            },
        )?;

        virt_region.start_addr()
    };

    Ok(virt_addr + offset_into_start_page)
}

pub fn try_kernel_virt_page_addr_to_phys_page_addr(
    virt_page_addr: PageAddress<Virtual>,
) -> Result<PageAddress<Physical>, &'static str> {
    memory::kernel_translation_tables()
        .read(|tables| tables.try_virt_page_addr_to_phys_page_addr(virt_page_addr))
}

pub fn try_kernel_page_attributes(
    virt_page_addr: PageAddress<Virtual>,
) -> Result<AttributeFields, &'static str> {
    memory::kernel_translation_tables()
        .read(|tables| tables.try_page_attributes(virt_page_addr))
}

#[inline(always)]
pub unsafe fn enable_mmu_and_caching(
    phys_tables_base_addr: Address<Physical>,
) -> Result<(), MMUEnableError> {
    // returns a temp object (static mmu)
    mmu().enable_mmu_and_caching(phys_tables_base_addr)
}

pub fn post_enable_init() {
    kernel_init_mmio_va_allocator();
}