extern crate netblocks;

use netblocks::udp_client_push::UdpPush;
use netblocks::udp_server_pull::UdpPull;


//use netblocks::udp_client_push::UdpPush;
//use netblocks::udp_server_pull::UdpPull;

use tokio;
use std::thread;

const packets: u64 = 10000;
fn main() {

    
    let threaded_rt = tokio::runtime::Runtime::new().unwrap();
    let h= threaded_rt.spawn(async {
        let mut pull = UdpPull::bind("127.0.0.1:1234").await;
        let start = tokio::time::Instant::now();
        let mut c = 0;
        //let packets = packets.clone();
        while let Some(z)=  pull.recv().await {
                
                if c % (packets/10) == 0 {
                //println!("{:?}", z);
                let dur = tokio::time::Instant::now().duration_since(start);
                println!("{}/{}...{}", c, packets, packets as f64 / dur.as_secs_f64());
            }
            c += 1;

            if c == packets { break }
        }
        let dur = tokio::time::Instant::now().duration_since(start);
        println!("{}/{}...{}", c, packets, packets as f64 / dur.as_secs_f64());
          
        
    });
    thread::sleep(tokio::time::Duration::from_millis(1000));
    threaded_rt.block_on( async {
        let push = UdpPush::connect("127.0.0.1:1234").await;
        println!("Hello, world!");
      
        for xx in 0..packets {
            //push.send(format!("yello man {}", xx).into()).await;
            push.send().await;
            //push.send("yello man2".into()).await;
            //let x = pull.recv().await;
            //println!("{:?}", x);
//            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(35000)).await;
    });
    
}
