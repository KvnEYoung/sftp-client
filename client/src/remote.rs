use connection::Connection;
use input;
use std::io::Read;

pub fn list_directories(connection: &Connection) -> String {
    let path = input::path();
    connection
        .sftp()
        .readdir(&path)
        .unwrap()
        .into_iter()
        .for_each(|d| println!("{:?}", d.0));
    format!("User listed remote directories at {:?}", path)
}

pub fn create_directory(connection: &Connection) -> String {
    let path = input::path();
    connection.sftp().mkdir(&path, 0).unwrap();
    format!("User created remote directory {:?}", path)
}

pub fn delete_directory(connection: &Connection) -> String {
    let path = input::path();
    connection.sftp().rmdir(&path).unwrap();
    format!("User deleted remote directory {:?}", path)
}

pub fn rename_file(connection: &Connection) -> String {
    let source = &input::prompt_path("\nEnter source: ");
    let destination = &input::prompt_path("\nEnter destination: ");
    connection.sftp().rename(source, destination, None).unwrap();
    format!("User renamed remote file {:?} to {:?}", source, destination)
}

pub fn delete_file(connection: &Connection) -> String {
    let path = input::path();
    connection.sftp().unlink(&path).unwrap();
    format!("User deleted remote file {:?}", path)
}

pub fn create_file(connection: &Connection) -> String {
    let sftp = connection.sftp();
    let path = input::path();
    sftp.create(&path).unwrap();
    format!("User created remote file {:?}", path)
}

pub fn download_file(connection: &Connection) -> String {
	let target = &input::prompt_path("\nWhich file would you like to download?: ");
	let (mut remote_file,stat) = connection.session.scp_recv(target).unwrap();
	let mut contents = Vec::new();
	remote_file.read_to_end(&mut contents).unwrap();
	std::fs::write(target,contents).unwrap();
	format!("User downloaded a remote file {:?}", target)	
}
