use drogue_ttn::v3::{Message, Payload};
use futures::stream::StreamExt;
use paho_mqtt::{AsyncClient, ConnectOptionsBuilder};

// TODO: Replace with your own TTN identifiers
const HOST: &str = "eu1.cloud.thethings.network";
const TOPICS: [&str; 2] = [
    "v3/<application-id>@ttn/devices/<device-id>/join",
    "v3/<application-id>@ttn/devices/<device-id>/up",
];
const QOS: [i32; 2] = [1, 1];
const USERNAME: &str = "<application-id>@ttn";
const PASSWORD: &str = "<api-key>";

#[tokio::main]
async fn main() -> Result<(), paho_mqtt::Error> {
    let mut client = AsyncClient::new(HOST)?;
    let mut stream = client.get_stream(224);

    let options = ConnectOptionsBuilder::new()
        .user_name(USERNAME)
        .password(PASSWORD)
        .finalize();

    client.connect(options).await?;
    client.subscribe_many(&TOPICS, &QOS).await?;

    println!("listening for messages...");
    while let Some(Some(raw)) = stream.next().await {
        if let Ok(payload) =
            serde_json::from_slice::<Message>(raw.payload()).map(|message| message.payload)
        {
            match payload {
                Payload::JoinAccept(_) => println!("device joined"),
                Payload::Uplink(uplink) => println!(
                    "uplink received: {}",
                    String::from_utf8(uplink.frame_payload).expect("not utf-8")
                ),
            }
        }
    }

    Ok(())
}
