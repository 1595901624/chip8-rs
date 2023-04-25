use crate::constant::CHIP8_MEMORY;

/// Chip8 解释器
///
/// Chip-8语言能够访问 4KB（4,096字节）的RAM
///
/// 从0x000（0）到 0xFFF（4095）位置。
///
/// 前512字节，从 0x000 到 0x1FF，是原始解释器的位置，不应该被程序使用。
///
/// 大多数Chip-8程序从位置 0x200（512）开始。
///
/// 例外：但有些程序从 0x600（1536）开始。以 0x600开始的程序是为 ETI 660计算机准备的，我们不考虑这种情况。
struct Chip8 {
    // 内存
    ram: [u8; CHIP8_MEMORY],
}