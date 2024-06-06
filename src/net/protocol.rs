use std::time::Duration;

use crate::*;

pub const PROTOCOL_ID: u64 = 0x1234abcd00000000
    | version::VERSION.protocol_v();

pub fn shared_config() -> SharedConfig {
    SharedConfig {
        client_send_interval: Duration::default(),
        server_send_interval: Duration::from_millis(40),
        tick: TickConfig {
            tick_duration: Duration::from_secs_f32(1.00 / 64.0),
        },
        mode: Mode::Separate,
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageUsi(pub usize);

#[derive(Channel)]
pub struct MyChannel;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<MessageUsi>(ChannelDirection::Bidirectional);
        app.add_channel::<MyChannel>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            direction: ChannelDirection::Bidirectional,
            ..default()
        });
    }
}
