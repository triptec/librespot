extern crate futures;
extern crate librespot;
extern crate tokio_core;
extern crate protobuf;
extern crate librespot_protocol as protocol;
use std::env;
use tokio_core::reactor::Core;

use librespot::core::authentication::Credentials;
use librespot::core::config::SessionConfig;
use librespot::core::session::Session;
use librespot::metadata::{RootList, RootListMetadata, Playlist, PlaylistMetadata};
use futures::{Future};

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let config = SessionConfig::default();

    let args : Vec<_> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} USERNAME PASSWORD", args[0]);
    }
    let username = args[1].to_owned();
    let password = args[2].to_owned();
    let credentials = Credentials::with_password(username, password);

    println!("Connecting ..");
    let session = core.run(Session::connect(config, credentials, None, handle)).unwrap();

    let root_list = core.run(RootList::get(&session, session.username())).unwrap();
    
    let mut playlist_futures = Vec::new();
    playlist_futures = root_list.playlists.iter().map(|i|{
        Playlist::get(&session, i.clone())
    }).collect();
    
    let playlists = core.run(futures::future::join_all(playlist_futures)).unwrap();

    playlists.iter().enumerate().fold((), |(), (i, v)|{
        println!("{}. {}", i, v.name);
    });
    println!("Done");
}
