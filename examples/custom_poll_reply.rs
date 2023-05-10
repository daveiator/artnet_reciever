use artnet_reciever::ArtnetRecieverBuilder;
use artnet_protocol::PollReply;

fn main() {

    let mut short_name = [0; 18];
    "rustartnet".bytes().enumerate().for_each(|(i, b)| short_name[i] = b);

    let mut long_name = [0; 64];
    "Rust Art-Net Reciever".bytes().enumerate().for_each(|(i, b)| long_name[i] = b);

    //For details on what to put here, see the offical artnet specification
    let poll_reply =  PollReply {
        address: [0, 0, 0, 0].into(), //will be automatically assigned
        port: 0, //will be automatically assigned
        version: [1, 0],
        port_address: [0, 0],
        oem: [0; 2],
        ubea_version: 0,
        status_1: 0,
        esta_code: 0,
        short_name,
        long_name,
        node_report: [0; 64],
        num_ports: [0, 1],
        port_types: [0x40, 0, 0, 0],
        good_input: [8; 4],
        good_output: [0x80, 0, 0, 0],
        swin: [0; 4],
        swout: [0; 4],
        sw_video: 0,
        sw_macro: 0,
        sw_remote: 0,
        style: 0x00,
        mac: [0; 6],
        bind_ip: [0, 0, 0, 0].into(), //will be automatically assigned
        bind_index: 1,
        status_2: 0,
        filler: [0; 26],
        spare: [0; 3],
    };

    let _reciever = ArtnetRecieverBuilder::default().poll_reply(poll_reply).build().unwrap();
    //...
}