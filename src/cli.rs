use std::{io::prelude::*, net::TcpStream, path::Path};
use ssh2::Session;

struct NamePasswd<'a> {
    name: &'a str,
    passwd: &'a str,
}

struct NameKey<'a> {
    name: &'a str,
    pubkey: Option<&'a Path>,
    privatekey: &'a Path,
    passphrase: Option<&'a str>,
}

pub enum LoginInfo<'a> {
    PASSWD(NamePasswd<'a>),
    KEY(NameKey<'a>),
}

pub fn login(host_port: &str, login_info: LoginInfo) -> Result<Session, String> {
    let tcp = TcpStream::connect(host_port).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    match login_info {
        LoginInfo::PASSWD(np) => sess.userauth_password(np.name, np.passwd).unwrap(),
        LoginInfo::KEY(key) => sess
            .userauth_pubkey_file(key.name, key.pubkey, key.privatekey, key.passphrase)
            .unwrap(),
    }

    if sess.authenticated() {
        Ok(sess)
    } else {
        Err("something went wrong".into())
    }
}

pub fn run_command(sess: &Session, command: &str) -> String {
    let mut channel = sess.channel_session().unwrap();
    channel.exec(command).unwrap();
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    print!("{}", output);
    channel.wait_close();
    println!("Session close at code {}", channel.exit_status().unwrap());
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_with_key() {
        let login_info = LoginInfo::KEY(NameKey {
            name: "root",
            pubkey: None,
            privatekey: Path::new("~/.ssh/id_rsa"),
            passphrase: None,
        });

        let sess = login("192.168.7.79:22", login_info).unwrap();
        let s = run_command(&sess, "ls -al");
        assert!(!s.is_empty());
    }

    #[test]
    fn test_login_with_passwd() {
        let login_info = LoginInfo::PASSWD(NamePasswd {
            name: "aaron",
            passwd: "aabbcc",
        });

        let sess = login("192.168.7.79:22", login_info).unwrap();
        let s = run_command(&sess, "ls -al");
        assert!(!s.is_empty());
    }
}
