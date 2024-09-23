use state::AppState;

pub mod entities;
pub mod state;

pub async fn serve(state: AppState) -> anyhow::Result<()> {
    let consumers = &state.services.jetstream_consumers;

    for consumer in consumers.iter() {

    }

    Ok(())
}
