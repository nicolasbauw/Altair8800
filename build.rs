use directories::UserDirs;
use fs_extra::{copy_items, dir::create};

fn main() {
    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let mut dest = home_dir.to_path_buf();
        let options = fs_extra::dir::CopyOptions::new();

        // Create .config directory in user's home directory
        dest.push(".config/");
        create(&dest, false).unwrap_or_default(); // Create error ? directory exists, so we don't care

        // Create teletype's config directory in user's home directory
        dest.push("teletype/");
        if create(&dest, false).is_err() {
            println!("teletype directory already exists");
        }

        // Copy teletype/config.toml to home_dir/.config/teletype/config.toml
        let mut from_paths = Vec::new();
        from_paths.push("config/config.toml");
        println!("Copying {:#?} to {:#?}", from_paths, dest);
        if copy_items(&from_paths, dest, &options).is_err() {
            println!("teletype configuration file already exists");
        }
    }
}
