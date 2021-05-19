extern crate netblocks;

use netblocks::tcp_client_push::TcpPush;
use netblocks::tcp_server_pull::TcpPull;


//use netblocks::udp_client_push::UdpPush;
//use netblocks::udp_server_pull::UdpPull;

use tokio;
use std::thread;

fn main() {
    
    let threaded_rt = tokio::runtime::Runtime::new().unwrap();
    let h= threaded_rt.spawn(async {
        let mut pull = TcpPull::bind("127.0.0.1:1234").await;
        let start = tokio::time::Instant::now();
        let mut c = 0;
        while let Some(z)=  pull.recv().await {
            c += 1;
            if c % 10000 == 0 {
            //println!("{:?}", z);
            let dur = tokio::time::Instant::now().duration_since(start);
            println!("{}", 1000000 as f64 / dur.as_secs_f64());
        }
        }
     
        //
    });
    thread::sleep(tokio::time::Duration::from_millis(1000));
    threaded_rt.block_on( async {
        let push = TcpPush::connect("127.0.0.1:1234").await;
        println!("Hello, world!");
      
        for xx in 0i32..1000000 {
            push.send(format!("yello man {}", xx).into()).await;
            //push.send("yello man2".into()).await;
            //let x = pull.recv().await;
            //println!("{:?}", x);
        
        }
       
    });
    thread::sleep(tokio::time::Duration::from_millis(1000));
}
