use bevy::log::LogPlugin;
use magic_game::net::protocol::{MessageUsi, MyChannel};
use magic_game::*;
use magic_game::client::*;
use net::client;
use net::protocol::shared_config;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(client::client_plugin(shared_config()))
        .add_plugins(net::ProtocolPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, (on_connect, on_disconnect, on_message))
        .run();
}
fn init(mut commands: Commands) {
    commands.connect_client();
}

fn on_connect(
    mut connections: EventReader<ConnectEvent>,
    mut conn: ResMut<ConnectionManager>,
) {
    for c in connections.read() {
        info!("i have connected to the server with id {}", c.client_id());
        let Ok(_) = conn.send_message::<MyChannel, _>(&MessageUsi(420))
        else {
            error!("could not send message to server :(");
            continue;
        };
    }
}

fn on_message(
    mut commands: Commands,
    mut messages: EventReader<MessageEvent<MessageUsi>>,
) {
    for m in messages.read() {
        info!("server sent us a packet: {:?}", m.message);
        commands.disconnect_client();
    }
}

fn on_disconnect(
    mut disconnects: EventReader<DisconnectEvent>
) {
    for _ in disconnects.read() {
        info!("disconnected :(");
    }
}