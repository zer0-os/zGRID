#![allow(unused_imports)]
use async_std::{ io, task };
use futures::{StreamExt, future::Either, prelude::*, select};
use libp2p::{
    gossipsub::{ IdentityTransform, Behaviour, MessageAuthenticity, AllowAllSubscriptionFilter, IdentTopic, ConfigBuilder, ValidationMode, DataTransform, TopicSubscriptionFilter },
    core::{ muxing::StreamMuxerBox, transport::OrTransport, upgrade },
    mdns,
    noise,
    tcp,
    yamux,
    identity,
    swarm::{Swarm, SwarmBuilder, SwarmEvent},
    Multiaddr, PeerId, Transport,
};
use libp2p_quic as quic;
use tokio;
use std::error::Error;
use std::env;


struct MyBehaviour {
    gossipsub: Behaviour,
    mdns: mdns::async_io::Behaviour,
}

#[async_std::main]
async fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() > 3 || args[1] != "-p" {
        eprintln!("Usage: cargo run -- -p <port>");
        std::process::exit(1);
    }
    
    // CMD-LINE: Fetch port number from cmd-line arguments
    // CMD-LINE: Port (-P)
    let port: u16 = match args[2].parse() {
        Ok(port) => port,
        Err(_) => {
            eprintln!("Invalid port number.");
            std::process::exit(1);
        }
    };
    // CMD-LINE: Fetch address from cmd-line arguments
    //if let Some(addr) = std::env::args().nth(1) {
    //}

    // Node: Setup private_key and node_id
    let local_node_key = identity::Keypair::generate_ed25519();
    let local_node_id = PeerId::from(local_node_key.public());
    println!("Node:Init: Id: {}", local_node_id);

    // Node: Setup transport layer
    let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default().nodelay(true))
        .upgrade(upgrade::Version::V1Lazy)
        .authenticate(noise::Config::new(&local_node_key).expect("signing libp2p-noise static keypair"))
        .multiplex(yamux::Config::default())
        .timeout(std::time::Duration::from_secs(20))
        .boxed();
    let quic_config = quic::Config::new(&local_node_key);
    let mut quic_transport = quic::async_std::Transport::new(quic_config);
    let transport = OrTransport::new(quic_transport, tcp_transport)
        .map(|either_output, _| match either_output {
            Either::Left((local_node_id, muxer)) => (local_node_id, StreamMuxerBox::new(muxer)),
            Either::Right((local_node_id, muxer)) => (local_node_id, StreamMuxerBox::new(muxer)),
        })
        .boxed();

    // Node: Setup (and subscribe) to unique  topic
    let topic = IdentTopic::new("grid_topic");

    // Node: Configure and init gossipsub instance
    let mut gossipsub = init_gossipsub(local_node_key.clone());

    // Node: Create Node Swarm. A Swarm controls the state of the network and how it behaves.
    let mut swarm = {
        let mut behaviour = init_gossipsub(local_node_key);
        behaviour.subscribe(&topic);
        SwarmBuilder::with_async_std_executor(transport, behaviour, local_node_id).build()
 
        //Arc::new(Mutex::new(SwarmBuilder::with_async_std_executor(transport, behaviour, local_node_id).build()))
    };

    // Node: Define MultiAddrs
    let node_address_tcp: Multiaddr = format!("/ip4/127.0.0.1/tcp/{}", port).parse().unwrap();
    let node_address_quic: Multiaddr = format!("/ip4/0.0.0.0/udp/0/quic-v1").parse().unwrap();

    println!("Node:Init: TCP MultiAddr set: {}", node_address_tcp);
    println!("Node:Init: Quic MultiAddr set: {}", node_address_quic);

    // Node: Initalize Swarm listeners for each MultiAddr
    let node_swarm_tcp = swarm.listen_on(node_address_tcp.clone()).unwrap();
    println!("Node:Init: TCP Swarm activated: {:?}", node_swarm_tcp);

    let node_swarm_quic = swarm.listen_on(node_address_quic.clone()).unwrap();
    println!("Node:Init: Quic Swarm initialized: {:?}", node_swarm_quic);

    // Node: Delay prior to showing connection 
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;
    
    // Node: Listeners -> Not sure if this is working correctly due to timing
    let node_swarm_listeners_count = swarm.listeners().count();
    println!("Node:Init: # of starting listeners: {:?}", node_swarm_listeners_count);

    // Node: End of init phase
    println!("-------------------");

    let remote_addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/3330").parse().unwrap();
    swarm.dial(remote_addr.clone());
    println!("attempted to dial remote: {:?}", remote_addr);

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Node:Event: Listening on: {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {
                // Handle all other events
                println!("meow")
            }
        }
    }
}

pub fn init_gossipsub(local_key: identity::Keypair) -> Behaviour<IdentityTransform, AllowAllSubscriptionFilter> {
    let gossipsub_config = ConfigBuilder::default()
        .validation_mode(ValidationMode::Strict)
        .build()
        .expect("Valid config");

    let mut gossipsub: Behaviour<IdentityTransform, AllowAllSubscriptionFilter> = Behaviour::new(
        MessageAuthenticity::Signed(local_key),
        gossipsub_config,
    ).unwrap();
    gossipsub
}
