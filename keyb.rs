static SCANCODE_LOOKUP_LOWERCASE:&[u8;90] = b"\0\x1F1234567890-=\x08\tqwertyuiop[]\n\0asdfghjkl;'`\0\\zxcvbnm,./\0*\0 \0\0\0\0\0\0\0\0\0\0\0\0\07894561230.\0\0\0\0\0\0\0\0";

pub fn scandecode(c: u8) -> u8 {
    match c {
        c if c<90 => SCANCODE_LOOKUP_LOWERCASE[c as usize],
        _ => 0
    }
}