type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Half = u16;
type Elf64Word = u32;
type Elf64Sword = i32;
type Elf64Xword = u64;
type Elf64Sxword = i64;

const ELF_EI_MAG0: u8 = 0;
const ELF_EI_MAG1: u8 = 1;
const ELF_EI_MAG2: u8 = 2;
const ELF_EI_MAG3: u8 = 3;
const ELF_EI_CLASS: u8 = 4;
const ELF_EI_DATA: u8 = 5;
const ELF_EI_VERSION: u8 = 6;
const ELF_EI_OSABI: u8 = 7;
const ELF_EI_ABIVERSION: u8 = 8;
const ELF_EI_PAD: u8 = 9;
const ELF_EI_NIDENT: u8 = 16;

const ELF_CLASS32: u8 = 1;
const ELF_CLASS64: u8 = 2;

const ELF_DATA2LSB: u8 = 1;
const ELF_DATA2MSB: u8 = 2;

const ELF_EV_CURRENT: u8 = 1;

const ELF_OSABI_SYSV: u8 = 0;
const ELF_OSABI_HPUX: u8 = 1;
const ELF_OSABI_STANDALONE: u8 = 255;

const ELF_ET_NONE: u8 = 0;
const ELF_ET_REL: u8 = 1;
const ELF_ET_EXEC: u8 = 2;
const ELF_ET_DYN: u8 = 3;
const ELF_ET_CORE: u8 = 4;

#[repr(packed)]
struct ElfEhdr {
    e_ident: [u8; 16],
    e_type: Elf64Half,
    e_machine: Elf64Half,
    e_version: Elf64Word,
    e_entry: Elf64Addr,
    e_phoff: Elf64Off,
    e_shoff: Elf64Off,
    e_flags: Elf64Word,
    e_ehsize: Elf64Half,
    e_phentsize: Elf64Half,
    e_phnum: Elf64Half,
    e_shentsize: Elf64Half,
    e_shnum: Elf64Half,
    e_shstrndx: Elf64Half,
}

#[derive(Clone, PartialEq)]
pub enum RangeTypes {
    None,
    End,
    FileHeader,
}

impl RangeTypes {
    pub fn class(&self) -> &str {
        match self {
            RangeTypes::FileHeader => "ehdr",
            _ => "",
        }
    }
}

pub struct ParsedElf {
    pub filename: String,
    pub identification: Vec<(String, String)>,
    pub contents: Vec<u8>,
    pub ranges: Vec<RangeTypes>,
}

impl ParsedElf {
    pub fn from_bytes(filename: &String, buf: Vec<u8>) -> Result<ParsedElf, &str> {
        if buf.len() < ELF_EI_NIDENT as usize {
            return Err("file smaller than ELF file header");
        }

        if buf[0..=ELF_EI_MAG3 as usize] != [0x7f, 'E' as u8, 'L' as u8, 'F' as u8] {
            return Err("mismatched magic: not an ELF file");
        }

        let mut identification = vec![];

        identification.push((
            String::from("Class"),
            match buf[ELF_EI_CLASS as usize] {
                ELF_CLASS32 => String::from("32-bit objects"),
                ELF_CLASS64 => String::from("64-bit objects"),
                x => format!("Unknown: {}", x),
            },
        ));

        identification.push((
            String::from("Data encoding"),
            match buf[ELF_EI_DATA as usize] {
                ELF_DATA2LSB => String::from("Little endian"),
                ELF_DATA2MSB => String::from("Big endian"),
                x => format!("Unknown: {}", x),
            },
        ));

        let ver = buf[ELF_EI_VERSION as usize];
        if ver != ELF_EV_CURRENT {
            identification.push((String::from("Uncommon version(!)"), format!("{}", ver)));
        }

        let abi = buf[ELF_EI_OSABI as usize];
        identification.push((
            String::from("ABI"),
            match abi {
                ELF_OSABI_SYSV => String::from("SysV"),
                ELF_OSABI_HPUX => String::from("HP-UX"),
                ELF_OSABI_STANDALONE => String::from("Standalone"),
                x => format!("Unknown: {}", x),
            },
        ));

        let abi_ver = buf[ELF_EI_ABIVERSION as usize];
        if !(abi == ELF_OSABI_SYSV && abi_ver == 0) {
            identification.push((
                String::from("Uncommon ABI version(!)"),
                format!("{}", abi_ver),
            ));
        }

        let ehdr_size = std::mem::size_of::<ElfEhdr>();

        if buf.len() < ehdr_size {
            return Err("file smaller than ELF file header");
        }

        let mut ranges = vec![RangeTypes::None; buf.len()];

        ranges[0] = RangeTypes::FileHeader;
        ranges[ehdr_size] = RangeTypes::End;

        Ok(ParsedElf {
            filename: filename.clone(),
            identification,
            contents: buf,
            ranges,
        })
    }
}