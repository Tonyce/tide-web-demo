fn main() {
    // println!("build...");
    println!("cargo:rustc-env=VAR=VALUE");
    println!(
        "cargo:rustc-env=DATABASE_URL=postgres://ttang@localhost/learn_db"
    );
    // TODO：一些配置项？
}
