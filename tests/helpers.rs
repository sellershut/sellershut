use fake::{
    faker::internet::en::{IPv4, Username},
    Fake, Faker,
};
use sellershut::{state::AppState, HutConfig};
use sellershut_services::{tracing::TracingBuilder, Configuration};

use std::sync::Once;

static TRACING: Once = Once::new();

pub struct TestApp {
    pub state: AppState,
    pub instance_name: String,
}

impl TestApp {
    pub async fn new(tx: tokio::sync::oneshot::Sender<u16>) -> Self {
        // Set port to 0 so tests can spawn multiple servers on OS assigned ports.

        // Setup tracing. Once.
        TRACING.call_once(|| {
            TracingBuilder::new().build(Some("warn".into()));
        });

        let config_path = "sellershut.toml";

        let config = config::Config::builder()
            .add_source(config::File::new(config_path, config::FileFormat::Toml))
            .build()
            .unwrap();

        let config = config.try_deserialize::<Configuration>().unwrap();
        let mut hut_config: HutConfig = serde_json::from_value(config.misc.clone()).unwrap();
        let instance: String = Username().fake();
        hut_config.instance_name = instance.clone();
        hut_config.hostname = format!("http://localhost:8080");

        let state = AppState::new(0, hut_config).await.unwrap();

        tokio::spawn(sellershut::run(state.clone(), tx, config));

        Self {
            state,
            instance_name: instance,
        }
    }
}
