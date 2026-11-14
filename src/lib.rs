//! This crate provides a fully written in Rust memory allocator for Vulkan, DirectX 12 and Metal.
//!
//! # Setting up the Vulkan memory allocator
//!
//! ```no_run
//! # #[cfg(feature = "vulkan")]
//! # fn main() {
//! use gpu_allocator::vulkan::*;
//! # use ash::vk;
//! # let device = todo!();
//! # let instance = todo!();
//! # let physical_device = todo!();
//!
//! let mut allocator = Allocator::new(&AllocatorCreateDesc {
//!     instance,
//!     device,
//!     physical_device,
//!     debug_settings: Default::default(),
//!     buffer_device_address: true,  // Ideally, check the BufferDeviceAddressFeatures struct.
//!     allocation_sizes: Default::default(),
//! });
//! # }
//! # #[cfg(not(feature = "vulkan"))]
//! # fn main() {}
//! ```
//!
//! # Simple Vulkan allocation example
//!
//! ```no_run
//! # #[cfg(feature = "vulkan")]
//! # fn main() {
//! use gpu_allocator::vulkan::*;
//! use gpu_allocator::MemoryLocation;
//! # use ash::vk;
//! # let device = todo!();
//! # let instance = todo!();
//! # let physical_device = todo!();
//! # let mut allocator = Allocator::new(&AllocatorCreateDesc {
//! #     instance,
//! #     device,
//! #     physical_device,
//! #     debug_settings: Default::default(),
//! #     buffer_device_address: true,  // Ideally, check the BufferDeviceAddressFeatures struct.
//! #     allocation_sizes: Default::default(),
//! # }).unwrap();
//!
//! // Setup vulkan info
//! let vk_info = vk::BufferCreateInfo::default()
//!     .size(512)
//!     .usage(vk::BufferUsageFlags::STORAGE_BUFFER);
//!
//! let buffer = unsafe { device.create_buffer(&vk_info, None) }.unwrap();
//! let requirements = unsafe { device.get_buffer_memory_requirements(buffer) };
//!
//! let allocation = allocator
//!     .allocate(&AllocationCreateDesc {
//!         name: "Example allocation",
//!         requirements,
//!         location: MemoryLocation::CpuToGpu,
//!         linear: true, // Buffers are always linear
//!         allocation_scheme: AllocationScheme::GpuAllocatorManaged,
//!     }).unwrap();
//!
//! // Bind memory to the buffer
//! unsafe { device.bind_buffer_memory(buffer, allocation.memory(), allocation.offset()).unwrap() };
//!
//! // Cleanup
//! allocator.free(allocation).unwrap();
//! unsafe { device.destroy_buffer(buffer, None) };
//! # }
//! # #[cfg(not(feature = "vulkan"))]
//! # fn main() {}
//! ```
//!
//! # Setting up the D3D12 memory allocator
//!
//! ```no_run
//! # #[cfg(feature = "d3d12")]
//! # fn main() {
//! use gpu_allocator::d3d12::*;
//! # let device = todo!();
//!
//! let mut allocator = Allocator::new(&AllocatorCreateDesc {
//!     device: ID3D12DeviceVersion::Device(device),
//!     debug_settings: Default::default(),
//!     allocation_sizes: Default::default(),
//! });
//! # }
//! # #[cfg(not(feature = "d3d12"))]
//! # fn main() {}
//! ```
//!
//! # Simple d3d12 allocation example
//!
//! ```no_run
//! # #[cfg(feature = "d3d12")]
//! # fn main() -> windows::core::Result<()> {
//! use gpu_allocator::d3d12::*;
//! use gpu_allocator::MemoryLocation;
//! # use windows::Win32::Graphics::{Dxgi, Direct3D12};
//! # let device = todo!();
//!
//! # let mut allocator = Allocator::new(&AllocatorCreateDesc {
//! #     device: ID3D12DeviceVersion::Device(device),
//! #     debug_settings: Default::default(),
//! #     allocation_sizes: Default::default(),
//! # }).unwrap();
//!
//! let buffer_desc = Direct3D12::D3D12_RESOURCE_DESC {
//!     Dimension: Direct3D12::D3D12_RESOURCE_DIMENSION_BUFFER,
//!     Alignment: 0,
//!     Width: 512,
//!     Height: 1,
//!     DepthOrArraySize: 1,
//!     MipLevels: 1,
//!     Format: Dxgi::Common::DXGI_FORMAT_UNKNOWN,
//!     SampleDesc: Dxgi::Common::DXGI_SAMPLE_DESC {
//!         Count: 1,
//!         Quality: 0,
//!     },
//!     Layout: Direct3D12::D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
//!     Flags: Direct3D12::D3D12_RESOURCE_FLAG_NONE,
//! };
//! let allocation_desc = AllocationCreateDesc::from_d3d12_resource_desc(
//!     &allocator.device(),
//!     &buffer_desc,
//!     "Example allocation",
//!     MemoryLocation::GpuOnly,
//! );
//! let allocation = allocator.allocate(&allocation_desc).unwrap();
//! let mut resource: Option<Direct3D12::ID3D12Resource> = None;
//! let hr = unsafe {
//!     device.CreatePlacedResource(
//!         allocation.heap(),
//!         allocation.offset(),
//!         &buffer_desc,
//!         Direct3D12::D3D12_RESOURCE_STATE_COMMON,
//!         None,
//!         &mut resource,
//!     )
//! }?;
//!
//! // Cleanup
//! drop(resource);
//! allocator.free(allocation).unwrap();
//! # Ok(())
//! # }
//! # #[cfg(not(feature = "d3d12"))]
//! # fn main() {}
//! ```
//!
//! # Setting up the Metal memory allocator
//!
//! ```no_run
//! # #[cfg(feature = "metal")]
//! # fn main() {
//! use gpu_allocator::metal::*;
//! # let device = objc2_metal::MTLCreateSystemDefaultDevice().expect("No MTLDevice found");
//! let mut allocator = Allocator::new(&AllocatorCreateDesc {
//!     device: device.clone(),
//!     debug_settings: Default::default(),
//!     allocation_sizes: Default::default(),
//!     create_residency_set: false,
//! });
//! # }
//! # #[cfg(not(feature = "metal"))]
//! # fn main() {}
//! ```
//!
//! # Simple Metal allocation example
//!
//! ```no_run
//! # #[cfg(feature = "metal")]
//! # fn main() {
//! use gpu_allocator::metal::*;
//! use gpu_allocator::MemoryLocation;
//! # let device = objc2_metal::MTLCreateSystemDefaultDevice().expect("No MTLDevice found");
//! # let mut allocator = Allocator::new(&AllocatorCreateDesc {
//! #     device: device.clone(),
//! #     debug_settings: Default::default(),
//! #     allocation_sizes: Default::default(),
//! #    create_residency_set: false,
//! # })
//! # .unwrap();
//! let allocation_desc = AllocationCreateDesc::buffer(
//!     &device,
//!     "Example allocation",
//!     512, // size in bytes
//!     MemoryLocation::GpuOnly,
//! );
//! let allocation = allocator.allocate(&allocation_desc).unwrap();
//! # use objc2_metal::MTLHeap;
//! let heap = unsafe { allocation.heap() };
//! let resource = unsafe {
//!     heap.newBufferWithLength_options_offset(
//!         allocation.size() as usize,
//!         heap.resourceOptions(),
//!         allocation.offset() as usize,
//!     )
//! }
//! .unwrap();
//!
//! // Cleanup
//! drop(resource);
//! allocator.free(&allocation).unwrap();
//! # }
//! # #[cfg(not(feature = "metal"))]
//! # fn main() {}
//! ```
#![deny(clippy::unimplemented, clippy::unwrap_used, clippy::ok_expect)]
#![warn(
    clippy::alloc_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core
)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "visualizer"))]
compile_error!("Cannot enable `visualizer` feature in `no_std` environment.");

#[cfg(not(any(feature = "std", feature = "hashbrown")))]
compile_error!("Either `std` or `hashbrown` feature must be enabled");

mod result;
pub use result::*;

pub(crate) mod allocator;

pub use allocator::{AllocationReport, AllocatorReport, MemoryBlockReport};

#[cfg(feature = "visualizer")]
pub mod visualizer;

#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(all(windows, feature = "d3d12"))]
pub mod d3d12;

#[cfg(all(target_vendor = "apple", feature = "metal"))]
pub mod metal;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemoryLocation {
    /// The allocated resource is stored at an unknown memory location; let the driver decide what's the best location
    Unknown,
    /// Store the allocation in GPU only accessible memory - typically this is the faster GPU resource and this should be
    /// where most of the allocations live.
    GpuOnly,
    /// Memory useful for uploading data to the GPU and potentially for constant buffers
    CpuToGpu,
    /// Memory useful for CPU readback of data
    GpuToCpu,
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub struct AllocatorDebugSettings {
    /// Logs out debugging information about the various heaps the current device has on startup
    pub log_memory_information: bool,
    /// Logs out all memory leaks on shutdown with log level Warn
    pub log_leaks_on_shutdown: bool,
    /// Stores a copy of the full backtrace for every allocation made, this makes it easier to debug leaks
    /// or other memory allocations, but storing stack traces has a RAM overhead so should be disabled
    /// in shipping applications.
    #[cfg(feature = "std")]
    pub store_stack_traces: bool,
    /// Log out every allocation as it's being made with log level Debug, rather spammy so off by default
    pub log_allocations: bool,
    /// Log out every free that is being called with log level Debug, rather spammy so off by default
    pub log_frees: bool,
    /// Log out stack traces when either `log_allocations` or `log_frees` is enabled.
    #[cfg(feature = "std")]
    pub log_stack_traces: bool,
}

impl Default for AllocatorDebugSettings {
    fn default() -> Self {
        Self {
            log_memory_information: false,
            log_leaks_on_shutdown: true,
            #[cfg(feature = "std")]
            store_stack_traces: false,
            log_allocations: false,
            log_frees: false,
            #[cfg(feature = "std")]
            log_stack_traces: false,
        }
    }
}

/// The sizes of the memory blocks that the allocator will create.
///
/// Useful for tuning the allocator to your application's needs. For example most games will be fine with the default
/// values, but eg. an app might want to use smaller block sizes to reduce the amount of memory used.
///
/// Clamped between 4MB and 256MB, and rounds up to the nearest multiple of 4MB for alignment reasons.
///
/// Note that these limits only apply to shared memory blocks that can hold multiple allocations.
/// If an allocation does not fit within the corresponding maximum block size, it will be placed
/// in a dedicated memory block holding only this allocation, without limitations other than what
/// the underlying hardware and driver are able to provide.
///
/// # Fixed or growable block size
///
/// This structure represents ranges of allowed sizes for shared memory blocks.
/// By default, if the upper bounds are not extended using `with_max_*_memblock_size`,
/// the allocator will be configured to use a fixed memory block size for shared
/// allocations.
///
/// Otherwise, the allocator will pick a memory block size within the specifed
/// range, depending on the number of existing allocations for the memory
/// type.
///
/// As a rule of thumb, the allocator will start with the minimum block size
/// and double the size with each new allocation, up to the specified maximum
/// block size. This growth is tracked independently for each memory type.
/// The block size also decreases when blocks are deallocated.
///
/// # Example
///
/// ```
/// use gpu_allocator::AllocationSizes;
/// const MB: u64 = 1024 * 1024;
/// // This configuration uses fixed memory block sizes.
/// let fixed = AllocationSizes::new(256 * MB, 64 * MB);
///
/// // This configuration starts with 8MB memory blocks
/// // and grows the block size of a given memory type each
/// // time a new allocation is needed, up to a limit of
/// // 256MB for device memory and 64MB for host memory.
/// let growing = AllocationSizes::new(8 * MB, 8 * MB)
///     .with_max_device_memblock_size(256 * MB)
///     .with_max_host_memblock_size(64 * MB);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct AllocationSizes {
    /// The initial size for device memory blocks.
    ///
    /// The size of new device memory blocks doubles each time a new block is needed, up to
    /// [`AllocationSizes::max_device_memblock_size`].
    ///
    /// Defaults to 256MB.
    min_device_memblock_size: u64,
    /// The maximum size for device memory blocks.
    ///
    /// Defaults to the value of [`AllocationSizes::min_device_memblock_size`].
    max_device_memblock_size: u64,
    /// The initial size for host memory blocks.
    ///
    /// The size of new host memory blocks doubles each time a new block is needed, up to
    /// [`AllocationSizes::max_host_memblock_size`].
    ///
    /// Defaults to 64MB.
    min_host_memblock_size: u64,
    /// The maximum size for host memory blocks.
    ///
    /// Defaults to the value of [`AllocationSizes::min_host_memblock_size`].
    max_host_memblock_size: u64,
}

impl AllocationSizes {
    /// Sets the minimum device and host memory block sizes.
    ///
    /// The maximum block sizes are initialized to the minimum sizes and
    /// can be increased using [`AllocationSizes::with_max_device_memblock_size`] and
    /// [`AllocationSizes::with_max_host_memblock_size`].
    pub fn new(device_memblock_size: u64, host_memblock_size: u64) -> Self {
        let device_memblock_size = Self::adjust_memblock_size(device_memblock_size, "Device");
        let host_memblock_size = Self::adjust_memblock_size(host_memblock_size, "Host");

        Self {
            min_device_memblock_size: device_memblock_size,
            max_device_memblock_size: device_memblock_size,
            min_host_memblock_size: host_memblock_size,
            max_host_memblock_size: host_memblock_size,
        }
    }

    /// Sets the maximum device memblock size, in bytes.
    pub fn with_max_device_memblock_size(mut self, size: u64) -> Self {
        self.max_device_memblock_size =
            Self::adjust_memblock_size(size, "Device").max(self.min_device_memblock_size);

        self
    }

    /// Sets the maximum host memblock size, in bytes.
    pub fn with_max_host_memblock_size(mut self, size: u64) -> Self {
        self.max_host_memblock_size =
            Self::adjust_memblock_size(size, "Host").max(self.min_host_memblock_size);

        self
    }

    fn adjust_memblock_size(size: u64, kind: &str) -> u64 {
        const MB: u64 = 1024 * 1024;

        let size = size.clamp(4 * MB, 256 * MB);

        if size % (4 * MB) == 0 {
            return size;
        }

        let val = size / (4 * MB) + 1;
        let new_size = val * 4 * MB;
        log::warn!(
            "{kind} memory block size must be a multiple of 4MB, clamping to {}MB",
            new_size / MB
        );

        new_size
    }

    /// Used internally to decide the size of a shared memory block
    /// based within the allowed range, based on the number of
    /// existing allocations. The more blocks there already are
    /// (where the requested allocation didn't fit), the larger
    /// the returned memory block size is going to be (up to
    /// `max_*_memblock_size`).
    pub(crate) fn get_memblock_size(&self, is_host: bool, count: usize) -> u64 {
        let (min_size, max_size) = if is_host {
            (self.min_host_memblock_size, self.max_host_memblock_size)
        } else {
            (self.min_device_memblock_size, self.max_device_memblock_size)
        };

        // The ranges are clamped to 4MB..256MB so we never need to
        // shift by more than 7 bits. Clamping here to avoid having
        // to worry about overflows.
        let shift = count.min(7) as u64;
        (min_size << shift).min(max_size)
    }

    /// The largest allocation a small block holds: a sixteenth of the
    /// smallest block of its kind of memory.
    pub(crate) fn small_allocation_limit(&self, is_host: bool) -> u64 {
        self.small_tier_block_size(is_host, 0) / SMALL_ALLOCATIONS_PER_BLOCK
    }

    /// Which blocks an allocation of `size` bytes is placed in.
    ///
    /// A small allocation goes to the smallest tier whose blocks hold at
    /// least [`SMALL_ALLOCATIONS_PER_BLOCK`] allocations of its size.
    pub(crate) fn block_class(&self, is_host: bool, size: u64) -> BlockClass {
        if size > self.small_allocation_limit(is_host) {
            return BlockClass::General;
        }
        let mut tier = 0;
        while tier + 1 < SMALL_BLOCK_TIERS
            && size * SMALL_ALLOCATIONS_PER_BLOCK <= self.small_tier_block_size(is_host, tier + 1)
        {
            tier += 1;
        }
        BlockClass::Small(tier)
    }

    /// The size of a block of small tier `tier`: the smallest block of its
    /// kind of memory divided by the tier's [`SMALL_TIER_DIVISORS`] entry.
    pub(crate) fn small_tier_block_size(&self, is_host: bool, tier: u8) -> u64 {
        let smallest = if is_host {
            self.min_host_memblock_size
        } else {
            self.min_device_memblock_size
        };
        smallest / SMALL_TIER_DIVISORS[usize::from(tier)]
    }

    /// The size a shared block of `class` starts from, given how many blocks
    /// of that class the memory type holds: a small tier's own block size, or
    /// the general block size for `count` blocks. An allocation larger than
    /// this takes a block of its own.
    pub(crate) fn memblock_size_for(&self, class: BlockClass, is_host: bool, count: usize) -> u64 {
        match class {
            BlockClass::Small(tier) => self.small_tier_block_size(is_host, tier),
            BlockClass::General => self.get_memblock_size(is_host, count),
        }
    }

    /// The size of the next shared block of `class` taken from the device,
    /// given how many blocks of that class the memory type holds and the bytes
    /// allocated in them (`live`).
    ///
    /// A small tier's next block is the largest power-of-two multiple of the
    /// tier's own block size that is at most `live`, and at most the smallest
    /// block size: a tier holding little opens blocks of its own size, and a
    /// tier filling up opens blocks as large as what it already holds, so a
    /// burst of small allocations opens a number of blocks that grows with
    /// the logarithm of its bytes up to the smallest block size, and in
    /// proportion to them past it. A tier's blocks therefore reserve at most
    /// about twice what is allocated in them when the last one opened, and
    /// the tier's own block size more. General blocks take
    /// [`AllocationSizes::memblock_size_for`].
    pub(crate) fn next_memblock_size(
        &self,
        class: BlockClass,
        is_host: bool,
        count: usize,
        live: u64,
    ) -> u64 {
        match class {
            BlockClass::Small(tier) => {
                let cap = self.small_tier_block_size(is_host, 0);
                let mut size = self.small_tier_block_size(is_host, tier);
                while size * 2 <= cap && size * 2 <= live {
                    size *= 2;
                }
                size
            }
            BlockClass::General => self.get_memblock_size(is_host, count),
        }
    }

    /// Whether an emptied shared block of `class` stays reserved as the class's
    /// last block: a general block always does, and a small tier's only at
    /// the tier's own block size, so a larger block a burst opened is returned
    /// to the device once it empties.
    pub(crate) fn keeps_last_empty_block(
        &self,
        class: BlockClass,
        is_host: bool,
        size: u64,
    ) -> bool {
        match class {
            BlockClass::Small(tier) => size == self.small_tier_block_size(is_host, tier),
            BlockClass::General => true,
        }
    }
}

/// How many allocations at the size limit of a small block fit in one: an
/// allocation is small when it is at most the smallest block size divided by
/// this, and it is placed in the smallest tier whose blocks hold this many of
/// its size.
const SMALL_ALLOCATIONS_PER_BLOCK: u64 = 16;

/// The sizes of small block, as divisors of the smallest block size: the
/// smallest block itself, and an eighth of it (8 and 1 MB for an 8 MB smallest
/// block). An allocation of up to a sixteenth of the eighth (64 KB there)
/// shares the eighth tier's blocks, which are the eighth size while the tier
/// holds little, so a long-lived allocation of a few bytes to a few kilobytes
/// made among few others of its size keeps a block of that size reserved
/// ([`AllocationSizes::next_memblock_size`]). Each tier a memory type uses
/// costs it at least one partly filled block, which is why the tiers are these
/// two rather than every halving between them.
const SMALL_TIER_DIVISORS: [u64; 2] = [1, 8];

/// How many sizes of small block there are.
pub(crate) const SMALL_BLOCK_TIERS: u8 = SMALL_TIER_DIVISORS.len() as u8;

/// The blocks an allocation shares.
///
/// A small allocation is placed only in blocks of its own small tier, and
/// every other allocation only in general blocks. A block is returned to the
/// device once nothing is allocated in it, so an allocation that lives long
/// keeps its block alive: a small one holds a block of its tier, among
/// allocations of its own tier's sizes, and the blocks that larger
/// allocations came and went in empty out and are returned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BlockClass {
    /// Allocations of at most [`AllocationSizes::small_allocation_limit`],
    /// by tier: tier `t` blocks are the smallest block size divided by
    /// [`SMALL_TIER_DIVISORS`]`[t]`.
    Small(u8),
    /// Every larger allocation.
    General,
}

/// How many shared blocks of each class a memory type holds.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct ActiveBlocks {
    general: usize,
    small: [usize; SMALL_BLOCK_TIERS as usize],
}

impl ActiveBlocks {
    /// The number of shared blocks of `class`.
    pub(crate) fn of(&self, class: BlockClass) -> usize {
        match class {
            BlockClass::Small(tier) => self.small[usize::from(tier)],
            BlockClass::General => self.general,
        }
    }

    /// Counts a shared block of `class` taken from the device.
    pub(crate) fn opened(&mut self, class: BlockClass) {
        match class {
            BlockClass::Small(tier) => self.small[usize::from(tier)] += 1,
            BlockClass::General => self.general += 1,
        }
    }

    /// Counts a shared block of `class` returned to the device.
    pub(crate) fn closed(&mut self, class: BlockClass) {
        match class {
            BlockClass::Small(tier) => self.small[usize::from(tier)] -= 1,
            BlockClass::General => self.general -= 1,
        }
    }
}

impl Default for AllocationSizes {
    fn default() -> Self {
        const MB: u64 = 1024 * 1024;
        Self {
            min_device_memblock_size: 256 * MB,
            max_device_memblock_size: 256 * MB,
            min_host_memblock_size: 64 * MB,
            max_host_memblock_size: 64 * MB,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AllocationSizes, BlockClass};

    const MB: u64 = 1024 * 1024;

    /// An allocation of at most a sixteenth of the smallest block is small,
    /// and its block is the smallest tier holding sixteen of its size, the
    /// same size however many there are; every larger allocation takes the
    /// growing general blocks.
    #[test]
    fn small_allocations_take_small_blocks_of_their_own() {
        let sizes = AllocationSizes::new(8 * MB, 4 * MB)
            .with_max_device_memblock_size(64 * MB)
            .with_max_host_memblock_size(32 * MB);

        assert_eq!(sizes.block_class(false, 512 * 1024), BlockClass::Small(0));
        assert_eq!(
            sizes.block_class(false, 512 * 1024 + 1),
            BlockClass::General
        );
        assert_eq!(sizes.block_class(true, 256 * 1024), BlockClass::Small(0));
        assert_eq!(sizes.block_class(true, 256 * 1024 + 1), BlockClass::General);

        for count in [0, 1, 5, 40] {
            assert_eq!(
                sizes.memblock_size_for(BlockClass::Small(0), false, count),
                8 * MB
            );
            assert_eq!(
                sizes.memblock_size_for(BlockClass::Small(0), true, count),
                4 * MB
            );
        }
        assert_eq!(
            sizes.memblock_size_for(BlockClass::General, false, 0),
            8 * MB
        );
        assert_eq!(
            sizes.memblock_size_for(BlockClass::General, false, 2),
            32 * MB
        );
        assert_eq!(
            sizes.memblock_size_for(BlockClass::General, false, 5),
            64 * MB
        );
    }

    /// A small allocation of up to a sixteenth of an eighth of the smallest
    /// block shares the eighth-size blocks, and a larger small allocation the
    /// smallest blocks. A long-lived allocation of a few bytes therefore keeps
    /// at most a 1 MB block reserved where the smallest block is 8 MB.
    #[test]
    fn a_small_allocation_takes_the_tier_sized_to_it() {
        let sizes = AllocationSizes::new(8 * MB, 4 * MB);

        assert_eq!(sizes.block_class(false, 192), BlockClass::Small(1));
        assert_eq!(sizes.block_class(false, 64 * 1024), BlockClass::Small(1));
        assert_eq!(
            sizes.block_class(false, 64 * 1024 + 1),
            BlockClass::Small(0)
        );
        assert_eq!(sizes.block_class(false, 512 * 1024), BlockClass::Small(0));
        assert_eq!(sizes.block_class(true, 32 * 1024), BlockClass::Small(1));
        assert_eq!(sizes.block_class(true, 32 * 1024 + 1), BlockClass::Small(0));

        assert_eq!(sizes.memblock_size_for(BlockClass::Small(1), false, 0), MB);
        assert_eq!(
            sizes.memblock_size_for(BlockClass::Small(1), true, 0),
            MB / 2
        );
        assert_eq!(
            sizes.memblock_size_for(BlockClass::Small(0), false, 3),
            8 * MB
        );

        for size in [1, 192, 4096, 64 * 1024, 100_000, 300_000, 512 * 1024] {
            let class = sizes.block_class(false, size);
            let block = sizes.memblock_size_for(class, false, 0);
            assert!(
                block >= size * 16,
                "{size} B sits sixteen to a {block} B block"
            );
        }
    }

    /// A small tier's next block is the largest power-of-two multiple of its
    /// own size that the bytes already allocated in the tier reach, up to the
    /// smallest block size; the smallest tier's blocks stay that size.
    #[test]
    fn a_small_tier_filling_up_opens_blocks_as_large_as_it_holds() {
        let sizes = AllocationSizes::new(8 * MB, 4 * MB);
        let next = |tier, live| sizes.next_memblock_size(BlockClass::Small(tier), false, 0, live);

        assert_eq!(next(1, 0), MB);
        assert_eq!(next(1, MB - 1), MB);
        assert_eq!(next(1, 2 * MB - 1), MB);
        assert_eq!(next(1, 2 * MB), 2 * MB);
        assert_eq!(next(1, 5 * MB), 4 * MB);
        assert_eq!(next(1, 8 * MB), 8 * MB);
        assert_eq!(next(1, 300 * MB), 8 * MB);
        assert_eq!(next(0, 0), 8 * MB);
        assert_eq!(next(0, 300 * MB), 8 * MB);
        assert_eq!(
            sizes.next_memblock_size(BlockClass::Small(1), true, 0, 64 * MB),
            4 * MB
        );
        assert_eq!(
            sizes.next_memblock_size(BlockClass::General, false, 0, 64 * MB),
            8 * MB
        );

        // A burst of 15 MB of allocations of up to 64 KB into an empty tier
        // opens 1, 1, 2, 4 and 8 MB blocks: five, against fifteen of 1 MB.
        let mut live = 0;
        let mut blocks = Vec::new();
        while live < 15 * MB {
            let block = next(1, live);
            blocks.push(block / MB);
            live += block;
        }
        assert_eq!(blocks, [1, 1, 2, 4, 8]);
    }

    /// An emptied block stays reserved as its class's last only at the
    /// size its class starts from.
    #[test]
    fn only_a_block_of_its_tier_size_stays_as_the_last() {
        let sizes = AllocationSizes::new(8 * MB, 4 * MB);
        assert!(sizes.keeps_last_empty_block(BlockClass::Small(1), false, MB));
        assert!(!sizes.keeps_last_empty_block(BlockClass::Small(1), false, 2 * MB));
        assert!(sizes.keeps_last_empty_block(BlockClass::Small(1), true, MB / 2));
        assert!(!sizes.keeps_last_empty_block(BlockClass::Small(1), true, MB));
        assert!(sizes.keeps_last_empty_block(BlockClass::Small(0), false, 8 * MB));
        assert!(sizes.keeps_last_empty_block(BlockClass::General, false, 64 * MB));
    }

    /// Each class counts its own shared blocks, small tiers apart.
    #[test]
    fn each_class_counts_its_own_blocks() {
        let mut active = super::ActiveBlocks::default();
        active.opened(BlockClass::Small(1));
        active.opened(BlockClass::Small(1));
        active.opened(BlockClass::General);
        assert_eq!(active.of(BlockClass::Small(1)), 2);
        assert_eq!(active.of(BlockClass::Small(0)), 0);
        assert_eq!(active.of(BlockClass::General), 1);
        active.closed(BlockClass::Small(1));
        assert_eq!(active.of(BlockClass::Small(1)), 1);
    }
}
