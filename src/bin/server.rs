use bevy::log::LogPlugin;
use magic_game::net::protocol::{MessageUsi, MyChannel};
use magic_game::*;
use magic_game::server::*;
use net::server;
use net::protocol::shared_config;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(LogPlugin::default())
        .add_plugins(server::server_plugin(shared_config()))
        .add_plugins(net::ProtocolPlugin)
        .add_systems(Startup, init)
        .add_systems(Update, (on_connect, on_disconnect, on_message))
        .run();
}

fn init(mut commands: Commands) {
    commands.start_server();
}

fn on_connect(
    mut connections: EventReader<ConnectEvent>
) {
    for c in connections.read() {
        info!("{} has connected to the server", c.client_id);
    }
}

fn on_message(
    mut messages: EventReader<MessageEvent<MessageUsi>>,
    mut conn: ResMut<ConnectionManager>,
) {
    for m in messages.read() {
        info!("{} sent us a packet: {:?}", m.context, m.message);
        let Ok(_) =
            conn.send_message::<MyChannel, _>(m.context, &MessageUsi(69))
        else {
            error!("could not send message to {}", m.context);
            continue;
        };
    }
}

fn on_disconnect(
    mut disconnects: EventReader<DisconnectEvent>
) {
    for d in disconnects.read() {
        info!("{} has disconnected from the server:", d.client_id);
    }
}
