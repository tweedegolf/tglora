# tglora

This project demonstrates the `lorawan` crate for the LoRa-E5 Dev Board. To run it, first set the credentials in
`device/src/main.rs` to the credentials you received from you LoRaWAN network:

```rust
// TODO: Change to your own credentials
const APP_EUI: AppEui = AppEui::new(0x0000000000000000);
const DEV_EUI: DevEui = DevEui::new(0xFFFFFFFFFFFFFFFF);
const APP_KEY: AppKey = AppKey::new(0xAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA);
```

Then run it in its own directory:

```shell
cd device
DEFMT_LOG=trace cargo run --release
```

## The Things Network

If you don't have the aforementioned credentials yet, follow these steps to retrieve credentials for The Things Network:

1. Go to [The Things Network console](https://console.cloud.thethings.network/), create an account if necessary.
2. Pick the region you're in (`lorawan` currently only supports `EU868`, but adding support for other regions is easy to
   do).
3. Create a new application, remember the application identifier.
4. Add a new end device to the application. All `EU868` options are supported, the LoRaWAN version should be MAC V1.
   0.2, and the optional parameters V1.0.2 REV B. Use the default values for the other settings, and remember the device
   identifier.

### Retrieve Data

This repository also contains a binary that retrieves data from The Things Network using MQTT. To use it, follow 
these steps:

1. Click the 'Integrations' tab in The Things Network console.
2. select MQTT, generate a new API key and copy it.
3. Fill in your identifiers and the API key in `ttn/src/main.rs`:

```rust
// TODO: Replace with your own TTN identifiers
const HOST: &str = "eu1.cloud.thethings.network";
const TOPICS: [&str; 2] = [
    "v3/<application-id>@ttn/devices/<device-id>/join",
    "v3/<application-id>@ttn/devices/<device-id>/up",
];
const QOS: [i32; 2] = [1, 1];
const USERNAME: &str = "<application-id>@ttn";
const PASSWORD: &str = "<api-key>";
```

Run this package before `device` to make sure you receive all of its messages:

```shell
cargo run --release --package ttn
```

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
