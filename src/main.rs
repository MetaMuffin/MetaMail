use std::io::{sink, Read, Write};

use mailin_embedded::{Handler, Server, SslConfig};

pub fn safe_filename(s: &str) -> String {
    s.replace(r"[^A-Za-z0-9]", "_")
}

pub fn save_mail(domain: &str, from: &str, to: &str) -> Box<dyn Write> {
    let path = format!("{}/{}/{}", "~/mail", safe_filename(to), safe_filename(from));
    let filename = safe_filename(format!("kekek").as_str());
    let filepath = format!("{}/{}", path, filename);

    println!("Saving mail from {} to '{}'", domain, path);
    std::fs::create_dir_all(&path).unwrap();

    let writer = std::fs::File::create(&filepath).unwrap();
    Box::new(writer)
}

#[derive(Clone)]
struct MyHandler {}

impl Handler for MyHandler {
    fn helo(&mut self, _ip: std::net::IpAddr, _domain: &str) -> mailin_embedded::HeloResult {
        mailin_embedded::HeloResult::Ok
    }

    fn mail(
        &mut self,
        _ip: std::net::IpAddr,
        _domain: &str,
        _from: &str,
    ) -> mailin_embedded::MailResult {
        mailin_embedded::MailResult::Ok
    }

    fn rcpt(&mut self, _to: &str) -> mailin_embedded::RcptResult {
        mailin_embedded::RcptResult::Ok
    }

    fn data(
        &mut self,
        domain: &str,
        from: &str,
        _is8bit: bool,
        to: &[String],
    ) -> mailin_embedded::DataResult {
        if to.len() < 1 {
            println!("Discarding a mail from '{}' as it has no recipient.", from);
            return mailin_embedded::DataResult::Ok(Box::new(sink()));
        } else if to.len() > 1 {
            println!(
                "A mail from '{}' has multiple recipients: {:?}  Only saving for the first one.",
                from, to
            );
        }
        let to_e = to[0].as_str();
        let writer = save_mail(domain, from, to_e);
        mailin_embedded::DataResult::Ok(writer)
    }

    fn auth_plain(
        &mut self,
        _authorization_id: &str,
        _authentication_id: &str,
        _password: &str,
    ) -> mailin_embedded::AuthResult {
        mailin_embedded::AuthResult::Ok
    }
}

fn main() {
    let handler = MyHandler {};
    let mut server = Server::new(handler);
    let mut srv_name = String::new();
    std::fs::File::open("server_name")
        .unwrap()
        .read_to_string(&mut srv_name)
        .unwrap();

    server
        .with_name(srv_name)
        .with_ssl(SslConfig::None)
        .unwrap()
        .with_addr("0.0.0.0:25")
        .unwrap();
    server.serve_forever().unwrap();
}
