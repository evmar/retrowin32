use memory::{Extensions, Mem};
use win32_winapi::calling_convention::VarArgs;

/// An implementation of the C `printf` function, used in DLLs that need one;
/// both user32 and msvcrt expose variations.
pub fn printf(
    out: &mut dyn std::io::Write,
    fmt: &str,
    mut args: VarArgs,
    mem: Mem,
) -> anyhow::Result<()> {
    fn read_number(c: u8) -> usize {
        // TODO: multiple digits, error handling, etc.
        assert!(c >= b'0' && c <= b'9');
        (c - b'0') as usize
    }

    let mut i = fmt.bytes();
    while let Some(c) = i.next() {
        if c == b'%' {
            let mut c = i.next().unwrap();

            let mut width = 0;
            if c >= b'0' && c <= b'9' {
                width = read_number(c);
                c = i.next().unwrap();
            }

            let mut precision = 0;
            if c == b'.' {
                c = i.next().unwrap();
                precision = read_number(c);
                c = i.next().unwrap();
            }

            let mut long = false;
            if c == b'l' {
                long = true;
                c = i.next().unwrap();
            }
            _ = long; // currently ignored

            match c {
                b'u' => write!(
                    out,
                    "{:width$.precision$}",
                    args.pop::<u32>(mem),
                    width = width,
                    precision = precision
                )?,
                b'd' => write!(
                    out,
                    "{:width$.precision$}",
                    args.pop::<i32>(mem),
                    width = width,
                    precision = precision
                )?,
                b's' => {
                    let addr = args.pop::<u32>(mem);
                    let str = mem.slicez(addr);
                    write!(out, "{}", std::str::from_utf8(str)?)?;
                }
                b'x' => write!(
                    out,
                    "{:width$.precision$x}",
                    args.pop::<u32>(mem),
                    width = width,
                    precision = precision
                )?,
                _ => todo!("format string character {:?}", c as char),
            }
        } else {
            out.write(&[c])?;
        }
    }
    Ok(())
}
