fn main() {
    let version = std::env::args().nth(1).expect("no version provided");
    let mut segments = version.split('.');
    let major: u16 = segments
        .next()
        .expect("no major version provided")
        .parse()
        .expect("invalid major version");
    let minor: u16 = segments
        .next()
        .expect("no minor version provided")
        .parse()
        .expect("invalid minor version");
    let patch: u16 = segments
        .next()
        .expect("no patch version provided")
        .parse()
        .expect("invalid patch version");
    assert!(segments.next().is_none(), "too many version segments");
    let version = format!("{major}.{minor}.{}-alpha.1", patch + 1);
    println!("{version}");
}
