/// Chip8 是单色 64 x 32 像素的显示屏
// 屏幕宽
pub(crate) const SCREEN_WIDTH: usize = 64;
// 屏幕高
pub(crate) const SCREEN_HEIGHT: usize = 32;
// 4KB 内存
pub(crate) const CHIP8_MEMORY: usize = 4096;
/// CHIP-8 程序严格基于十六进制。
///
/// 这意味着 CHIP-8 程序的格式与高级语言的基于文本的格式几乎没有相似之处。
///
/// 每条 CHIP-8 指令的长度为两个字节，并使用四个十六进制数字表示。
pub(crate) const INSTRUCTION_LENGTH: usize = 2;