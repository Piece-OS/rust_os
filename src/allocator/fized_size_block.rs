/// The block sizes to use.
///
/// The sizes must each be a power of 2 because the are also used as
/// the block alignment (alignments must be always be powers of 2).
const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];
struct ListNode { next: Option<&'static mut ListNode> }
pub struct FixedSizeAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}
