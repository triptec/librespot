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

    /*
    let req = session.mercury().get(format!("hm://playlist/user/{}/publishedrootlist", session.username()));

    let res = req.and_then(move |response| {
        let data = response.payload.first().expect("Empty payload");
        let msg:protocol::playlist4changes::SelectedListContent = protobuf::parse_from_bytes(data).unwrap();
        let items: Vec<String> = msg.get_contents().get_items().iter().map(|i| {
            println!("{}, {}", i.get_uri().to_string(), i.get_attributes().get_message());
            i.get_uri().to_string()
        }).collect();
        Ok(items)
    });
    let items = core.run(res).unwrap();
    let mut i = 0;
    let mut fvec = Vec::new();
    for item in items.clone() {
        let split:Vec<&str> = item.split(":").collect();

        //println!("{:?}", split);

        let req = session.mercury().get(format!("hm://playlist/user/{}/playlist/{}", split[2],split[4]));

        let res = req.and_then(move |response| {
            let data = response.payload.first().expect("Empty payload");
            let msg:protocol::playlist4changes::SelectedListContent = protobuf::parse_from_bytes(data).unwrap();

            //println!("{}. {}", i, msg.get_attributes().get_name());
            let name = msg.get_attributes().get_name().to_string();
            /*
            let items: Vec<String> = msg.get_contents().get_items().iter().map(|i| {
                println!("{}", i.get_uri().to_string());
                i.get_uri().to_string()
            }).collect();
            */
            Ok((i, name))
        });
        i=i+1;
        fvec.push(res);
        //core.run(res);
    }
    let mut farray = futures::future::select_all(fvec);
    let mut far = farray;
    for i in 0..i-1 {
        let ar = match core.run(far) {
            Ok(v) => {
                let (v, idx, ar) = v;
                println!("sucess:{:?},{},{}", v, i, idx);
                ar
                // let (idx, res, ar) = v;
            },
            Err(e) => {
                let (e, idx, ar) = e;
                println!("err:{:?},{},{}", e, i, idx);
                ar
            }
        };
        if ar.len() <= 0 {
            break;
        }else{
            far = futures::future::select_all(ar);
        }
    }
    */
    let root_list = core.run(RootList::get(&session, session.username())).unwrap();
    let mut playlists = Vec::new();
    playlists = root_list.playlists.iter().map(|i|{
        Playlist::get(&session, i.clone())
    }).collect();
    let p = core.run(futures::future::join_all(playlists));
    println!("{:?}", p);
    println!("Done");
}
