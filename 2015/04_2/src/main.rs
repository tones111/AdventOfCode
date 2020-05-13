extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::sync_channel;
use std::thread;

fn main() {
    const THREADS: u8 = 3;
    const HASH_BEGIN: &'static str = "000000";
    const CHUNK_SIZE: u16 = 256;

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let key = Arc::new(input.trim_right().to_string());

    let data = Arc::new(Mutex::new(1u64));
    let mut chans = Vec::new();

    for _ in 0..THREADS {
        let key = key.clone();

        let (tx, rx) = sync_channel(CHUNK_SIZE as usize);
        let (data, tx) = (data.clone(), tx.clone());
        chans.push(rx);

        thread::spawn(move || {
            let mut digest = Md5::new();
            'send: loop {
                let base;
                {
                    let mut data = data.lock().unwrap();
                    base = *data;
                    *data += CHUNK_SIZE as u64;
                }

                for i in 0..CHUNK_SIZE {
                    let index = base + i as u64;
                    digest.reset();
                    digest.input(key.as_bytes());
                    digest.input(format!("{}", index).as_bytes());

                    if tx.send((index, digest.result_str())).is_err() {
                        break 'send;
                    }
                }
            }
        });
    }

    let mut next_msg = Vec::with_capacity(chans.len());
    for chan in &chans {
        next_msg.push(chan.recv().unwrap());
    }

    let mut processed = 0u64;
    'receive: loop {
        // Process the values in order
        // Determine which channel sent the next value
        let mut next_val = u64::max_value();
        let mut next_chan = 0;
        for (i, msg) in next_msg.iter().enumerate() {
            if msg.0 < next_val {
                next_val = msg.0;
                next_chan = i;
                if next_val == processed + 1 {
                    break;
                }
            }
        }
        let msg = &mut next_msg[next_chan];

        if msg.0 != processed + 1 {
            println!("Error: couldn't find next value {} != {}",
                     processed + 1,
                     msg.0);
            return;
        }

        while msg.0 == processed + 1 {
            if msg.1.starts_with(HASH_BEGIN) {
                println!("{}: {}", msg.0, msg.1);
                break 'receive;
            }
            processed += 1;
            // println!("{}: {}", msg.0, msg.1);
            *msg = chans[next_chan].recv().unwrap();
        }
    }
}
