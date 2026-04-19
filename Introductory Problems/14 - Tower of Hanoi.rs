fn hanoi(
    out: &mut cp::tools::FastOutput<std::io::StdoutLock<'static>>,
    count: u8,
    src: u8,
    dst: u8,
) {
    if count == 1 {
        out.u8(src as u8);
        out.byte(b' ');
        out.u8(dst as u8);
        out.byte(b'\n');
    } else {
        let md = 6 - src - dst;
        hanoi(out, count - 1, src, md);
        hanoi(out, 1, src, dst);
        hanoi(out, count - 1, md, dst);
    }
}

pub fn solve() {
    cp::prepare!();

    let mut out = cp::tools::FastOutput::stdout();
    sc!(n: u8);
    out.u64((1 << n as u64) - 1);
    out.byte(b'\n');
    hanoi(&mut out, n, 1, 3);

    out.flush();
}

cp::main!(large_stack);
