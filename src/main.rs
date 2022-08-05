use std::fs::{copy, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut need_inject = false;
    let mpackage = Path::new("package_modified.json");
    let opackage = Path::new("package.json");
    let bpackage = Path::new("package_backup.json");
    if mpackage.exists() {
        copy(opackage, bpackage).unwrap();
        copy(mpackage, opackage).unwrap();
        need_inject = true;
    }
    let mindex = Path::new("index_modified.html");
    let oindex = Path::new("index.html");
    let bindex = Path::new("index_backup.html");
    let tindex = Path::new("index_temp.html");
    let index = if mindex.exists() {
        need_inject = true;
        mindex
    } else {
        oindex
    };
    if need_inject {
        copy(oindex, bindex).unwrap();
        let js = format!("<script>{}</script>", include_str!("index.js"));
        let mut inf = File::open(index).unwrap();
        let mut ouf = File::create(tindex).unwrap();
        ouf.write_all(js.as_bytes()).unwrap();
        std::io::copy(&mut inf, &mut ouf).unwrap();
        copy(tindex, oindex).unwrap();
    }
    let mut cmd = Command::new("nwo");
    cmd.spawn().expect("Failed to start.");
}
