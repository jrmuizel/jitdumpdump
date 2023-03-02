use scroll::{Pread, LE};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

#[derive(Debug, Pread)]
struct Header {
    magic: u32,
    version: u32,
    total_size: u32,
    elf_mach: u32,
    pad1: u32,
    pid: u32,
    timestamp: u64,
    flags: u64
}

#[derive(Debug, Pread)]
struct Record {
    id: u32,
    total_size: u32,
    timestamp: u64,
}

#[derive(Debug, TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
enum RecordType {
    CodeLoad = 0,
    CodeMove = 1,
    CodeDebugInfo = 2,
    CodeClose = 3,
    CodeUnwindingInfo = 4
}

#[derive(Debug, Pread)]
struct CodeLoadHeader {
    pid: u32,
    tid: u32,
    vma: u64,
    code_addr: u64,
    code_size: u64,
    code_index: u64,
}


fn main() {
    let dump = std::fs::read(std::env::args().nth(1).unwrap()).unwrap();

    let mut offset = 0;
    let header: Header = dump.gread(&mut offset).unwrap();
    dbg!(&header);
    assert_eq!(header.magic, 0x4a695444);
    loop {
        let mut record_offset = offset;
        let record = dump.gread::<Record>(&mut record_offset);
        if let Ok(record) = record {
            dbg!(&record);
            let id = RecordType::try_from(record.id).unwrap();
            dbg!(id);
            

            match id {
                RecordType::CodeLoad => {
                    let code_load: CodeLoadHeader = dump.pread(record_offset).unwrap();
                    dbg!(code_load);
                }
                _ => {}
            }
            offset += record.total_size as usize;

        } else {
            break;
        }
    }


}
