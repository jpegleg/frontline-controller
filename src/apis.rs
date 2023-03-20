use std::io::{ Read, Write };
use std::sync::Arc;
use std::net::TcpStream;
use std::str;

use uuid::Uuid;
use chrono::prelude::*;
use rustls::{OwnedTrustAnchor, RootCertStore};
use blake2::{Blake2b, Digest};

extern crate base64;

pub fn apicon(uid: &Uuid, url: &str, uport: &str, trim: usize) -> String {
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| {
                OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            }),
    );
    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let server_name = url.try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(config), server_name).unwrap();
    let mut sock = TcpStream::connect(&uport).unwrap();
    let readu: DateTime<Utc> = Utc::now();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    tls.write_all(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: google.com\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
        )
        .as_bytes(),
    )
    .unwrap();
    let ciphersuite = tls
        .conn
        .negotiated_cipher_suite()
        .unwrap();
    writeln!(
        &mut std::io::stderr(),
        "{} {} ciphersuite in test {}: {:?}", 
        readu,
        &uid,
        uport,
        ciphersuite.suite()
    )
    .unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    if plaintext.len() >= trim {
        plaintext = plaintext.split_off(trim);
    } else {
        plaintext.clear();
    }
    let mut hasher = Blake2b::new();
    hasher.update(plaintext);
    let blake2_hash = hasher.finalize();
    let encoded_hash: String = base64::encode(blake2_hash);
    return encoded_hash

}
