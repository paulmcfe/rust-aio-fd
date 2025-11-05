use std::path::{Component, Path};

fn main() {
    let settings = Path::new("/home/user/.config/app/settings.json");

    let parent = settings.parent().unwrap();
    println!("The parent directory is {:?}.", parent);

    let filename = settings.file_name().unwrap();
    println!("The filename is {:?}.", filename);

    let extension = settings.extension().unwrap();
    println!("The extension is {:?}.", extension);

    for component in settings.components() {
        match component {
            Component::Normal(name) => {
                println!("Path component: {:?}", name.to_string_lossy());
            }
            _ => {}
        }
    }

    for ancestor in settings.ancestors() {
        println!("Next ancestor: {:?}", ancestor);
    }
}
