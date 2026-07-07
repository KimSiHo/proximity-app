mod cli;
mod communication;
mod logic;
mod protocol_bindings;
mod tracking;

use std::mem::size_of;

use clap::Parser;

use crate::cli::Opts;
use crate::communication::{Communication, UdsServer};
use crate::logic::{Recorder};
use crate::protocol_bindings::{DetectionPacket, DS_SOCKET_PATH};
use crate::tracking::TrackerHistory;

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();
    println!("options: {}", opts.socket);

    println!("DetectionPacket size = {}", size_of::<DetectionPacket>());

    let path = std::ffi::CStr::from_bytes_with_nul(DS_SOCKET_PATH)
        .unwrap()
        .to_str()
        .unwrap();

    let mut comm = UdsServer::new(path);
    comm.connect()?;

    let packet_size = size_of::<DetectionPacket>();

    let mut tracker = TrackerHistory::new();
    let mut recorder = Recorder::new();

    loop {
        let mut buf = vec![0u8; packet_size];
        let mut received = 0;

        while received < packet_size {
            let n = comm.recv(&mut buf[received..])?;

            if n == 0 {
                return Ok(());
            }

            received += n;
        }

        let packet = unsafe { &*(buf.as_ptr() as *const DetectionPacket) };

        #[cfg(debug_assertions)]
        {
            println!(
                "frame={}, objects={}",
                packet.frame_num, packet.object_count
            );

            for i in 0..packet.object_count as usize {
                let obj = &packet.objects[i];

                println!(
                    "id={} class={} conf={:.2} ({:.1},{:.1},{:.1},{:.1})",
                    obj.object_id,
                    obj.class_id,
                    obj.confidence,
                    obj.left,
                    obj.top,
                    obj.width,
                    obj.height
                );
            }
        }

        tracker.update(packet);

        if let Some(cmd) = recorder.update(&mut tracker) {
            println!("{cmd:?}");
            comm.send(cmd.as_str().as_bytes())?;
        }

        //println!("{}", tracker);
    }
}
