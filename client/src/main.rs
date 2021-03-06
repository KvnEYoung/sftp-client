extern crate client;

use client::connection::Connection;
use client::menu::{Menu, MenuItem};
use client::{local, remote};

fn main() {
    let connection = Connection::from_prompt();

    let mut menu = Menu::new("SFTP Client", connection);

    insert_local_menu_items(&mut menu);
    insert_remote_menu_items(&mut menu);
    menu.insert(MenuItem::new(
        "View Conenction Info",
        Connection::view_connection_info,
    ));
    menu.insert(MenuItem::new("Log off", |_| std::process::exit(0)));
    loop {
        menu.show();
        println!("");
    }
}

fn insert_local_menu_items(menu: &mut Menu) {
    menu.insert(MenuItem::new(
        "List local directories",
        local::list_directories,
    ));

    menu.insert(MenuItem::new(
        "Change permissions on local directory",
        local::change_permission,
    ));

    menu.insert(MenuItem::new(
        "Rename file on local machine",
        local::rename_file,
    ));
}

fn insert_remote_menu_items(menu: &mut Menu) {
    menu.insert(MenuItem::new(
        "List remote directories",
        remote::list_directories,
    ));

    menu.insert(MenuItem::new(
        "Create remote directory",
        remote::create_directory,
    ));

    menu.insert(MenuItem::new(
        "Delete remote directory",
        remote::delete_directory,
    ));

    menu.insert(MenuItem::new(
        "Copy remote directory",
        remote::copy_directory,
    ));

    menu.insert(MenuItem::new(
        "Change permissions on remote directory",
        remote::change_permission,
    ));

    menu.insert(MenuItem::new(
        "Rename remote file or directory",
        remote::rename_file,
    ));

    menu.insert(MenuItem::new("Create remote file", remote::create_file));

    menu.insert(MenuItem::new("Delete remote file", remote::delete_file));

    menu.insert(MenuItem::new(
        "Upload file onto remote server",
        remote::upload_file,
    ));

    menu.insert(MenuItem::new(
        "Upload multiple files onto remote server",
        remote::upload_multiple_files,
    ));

    menu.insert(MenuItem::new("Download remote file", remote::download_file));

    menu.insert(MenuItem::new(
        "Download multiple remote files",
        remote::download_multiple_files,
    ));
}
