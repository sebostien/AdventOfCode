pub fn pprint_bool(map: &[Vec<bool>]) {
    let mut out = String::new();
    for c in map {
        for &c in c {
            if c {
                out.push('.');
            } else {
                out.push('#');
            }
        }
        out.push('\n');
    }
    println!("{out}");
}

