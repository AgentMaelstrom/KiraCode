/// # generate.rs Module
///
/// The generation module for the KiraCode compiler, version 'test0'.

// ---

pub struct ASM;

impl ASM {
    pub fn mov(register: &str, value: i32) -> String {
        String::from("mov ") + register + ", " + value.to_string().as_str()
    }

    pub fn eax() -> &'static str {
        "eax"
    }

    pub fn ebx() -> &'static str {
        "ebx"
    }

    pub fn ecx() -> &'static str {
        "ecx"
    }

    pub fn edx() -> &'static str {
        "edx"
    }

    pub fn esi() -> &'static str {
        "esi"
    }

    pub fn edi() -> &'static str {
        "edi"
    }

    pub fn ebp() -> &'static str {
        "ebp"
    }

    pub fn rax() -> &'static str {
        "rax"
    }

    pub fn rdi() -> &'static str {
        "rdi"
    }

    pub fn rsi() -> &'static str {
        "rsi"
    }

    pub fn rdx() -> &'static str {
        "rdx"
    }

    pub fn r10() -> &'static str {
        "r10"
    }

    pub fn r8() -> &'static str {
        "r8"
    }

    pub fn r9() -> &'static str {
        "r9"
    }
}

pub struct Generator;
