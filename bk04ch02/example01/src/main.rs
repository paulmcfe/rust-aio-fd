use std::path::Path;

fn main() {
    // base is a &Path
    let base = Path::new("/etc");

    // join() creates a brand new PathBuf
    let nginx_conf = base.join("nginx/nginx.conf");

    // base is unchanged
    println!("Base path: {}", base.display());

    // nginx_conf is an owned PathBuf, so you can use it freely
    println!("Joined path: {}", nginx_conf.display());
}
