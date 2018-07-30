use ssh2::{Session, Sftp};
use std::net::TcpStream;
use std::fs::OpenOptions;
//use std::fs::File;
use input;
use std::io::Write;
use std::io::Read;

pub struct Connection {
    pub tcp: TcpStream,
    pub session: Session,
}

impl Connection {
    pub fn new(host: &str, username: &str, password: &str) -> Connection {
        let tcp = TcpStream::connect(host).unwrap();
        let mut session = Session::new().unwrap();
        session.handshake(&tcp).unwrap();
        session.userauth_password(username, password).unwrap();

         let mut log = OpenOptions::new()                                                                                
            .read(true)                                                                                                        
            .append(true)                                                                                                      
            .create(true)                                                                                                      
            .open("connection.txt")
            .unwrap();

        if let Err(e) = writeln!(log, "Hostname: {}  Username: {}", host, username) {
            eprintln!("Couldn't write to log: {}", e);
        }

        Connection {tcp, session}
    }

        

    pub fn to_container() -> Connection {
        Connection::new("server:22", "root", "root")
    }

    pub fn from_prompt() -> Connection {
        let host = input::string("Enter host: ");
        let username = input::string("Enter username: ");
        let password = input::password();
        Connection::new(&host, &username, &password)
    }

    pub fn sftp(&self) -> Sftp {
        self.session.sftp().unwrap()
    }

    pub fn read_log(&self) {
        let mut log = OpenOptions::new()
            .read(true)
            .open("connection.txt")
            .unwrap();

        let mut info = String::new();
        log.read_to_string(&mut info)
            .expect("something broke");

        println!("{}", info);

    }
 }
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn log_into_remote_ftp_server() {
        Connection::to_container();
    }

    #[test]
    fn create_directory_on_remote_server() {
        let connection = Connection::to_container();
        let sftp = connection.sftp();
        let path = Path::new("/demo");
        sftp.mkdir(path, 0).unwrap();
        sftp.readdir(path).unwrap();
        sftp.rmdir(path).unwrap();
    }
}
