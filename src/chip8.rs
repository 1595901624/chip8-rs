use crate::constant::{CHIP8_MEMORY, FONT_SET, SCREEN_HEIGHT, SCREEN_WIDTH};

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
    // 屏幕
    screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
    // 内存
    memory: [u8; CHIP8_MEMORY],
    // 一个长度为 16 的数组，表示虚拟机的通用寄存器。
    data_register: [u8; 16],
    //  一个 16 位的寄存器，可以用来存储内存地址 I
    address_register: u16,
    // 两个定时器
    // 延迟定时器
    delay_timer: u8,
    // 声音定时器
    sound_timer: u8,
    // 一个长度为 16 的布尔数组，表示虚拟机的键盘
    keyboard: [bool; 16],
    keyboard_waiting: bool,
    keyboard_register: usize,

    // 一个长度为 16 的数组，用于实现函数调用和返回
    stack: [u16; 16],
    // 一个指向栈顶的索引，用于跟踪栈的使用情况
    stack_pointer: usize,
    // 一个 16 位的寄存器，用于存储当前执行的指令地址
    program_counter: u16,
}

impl Chip8 {
    /// 创建Chip8
    pub fn new() -> Self {
        // 将字体放置在内存的前 80 个字节
        let mut memory = [0u8; CHIP8_MEMORY];
        for i in 0..FONT_SET.len() {
            memory[i] = FONT_SET[i];
        }

        return Self {
            screen: [[0; SCREEN_WIDTH]; SCREEN_HEIGHT],
            memory,
            data_register: [0; 16],
            program_counter: 0x200,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: [false; 16],
            keyboard_waiting: false,
            keyboard_register: 0,
            address_register: 0,
            stack: [0; 16],
            stack_pointer: 0,
        };
    }

    /// 读取游戏 rom
    pub fn load_rom(&mut self, rom_data: &[u8]) {
        for (i, &byte) in rom_data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr < 4096 {
                self.memory[0x200 + i] = byte;
            } else {
                break;
            }
        }
    }
}