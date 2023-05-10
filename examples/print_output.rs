use artnet_reciever::ArtnetRecieverBuilder;

fn main() {
    let reciever = ArtnetRecieverBuilder::default().build().unwrap();
    for packet in reciever {
        println!("{:?}", packet);
    }
}