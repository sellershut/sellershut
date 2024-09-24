use std::ops::Deref;

use activitypub_federation::config::FederationConfig;
use async_nats::jetstream::{
    consumer::{
        pull::{self},
        push, Consumer,
    },
    Message,
};
use futures_util::{Stream, StreamExt, TryFutureExt};
use tracing::{error, trace};

use crate::state::AppState;

pub async fn start_consumer(state: FederationConfig<AppState>) {
    let pull_consumer = state.services.jetstream_pull_consumers.as_ref();
    let push_consumer = state.services.jetstream_push_consumers.as_ref();
    //
    tokio::join!(
        pull(pull_consumer, state.clone()),
        push(push_consumer, state.clone())
    );
}

async fn pull(consumers: &[Consumer<pull::Config>], state: FederationConfig<AppState>) {
    let mut messages = Vec::with_capacity(consumers.len());

    for consumer in consumers.to_vec().into_iter() {
        let state = state.clone();
        let stream = tokio::spawn(async move {
            let stream = consumer
                .messages()
                .and_then(|stream| async move { process_stream(stream, state.clone()).await });
            if let Err(e) = stream.await {
                error!("{e}");
            }
        });
        messages.push(stream);
    }

    futures_util::future::join_all(messages).await;
}

async fn process_stream<S, T>(
    mut stream: S,
    _state: FederationConfig<AppState>,
) -> Result<(), async_nats::error::Error<async_nats::jetstream::consumer::StreamErrorKind>>
where
    S: Stream<Item = Result<Message, T>> + Unpin,
{
    while let Some(Ok(event)) = stream.next().await {
        trace!(subject = event.subject.deref(), "received event");
        /* depending on subject, you may also want to fire some events fed config
        let data = _state.to_request_data();
         CreateListing::verify(todo!(), &data);
        */
    }

    Ok(())
}

async fn push(consumers: &[Consumer<push::Config>], state: FederationConfig<AppState>) {
    let mut messages = Vec::with_capacity(consumers.len());

    for consumer in consumers.to_vec().into_iter() {
        let state = state.clone();
        let stream = tokio::spawn(async move {
            let stream = consumer
                .messages()
                .and_then(|messages| async move { process_stream(messages, state).await });
            if let Err(e) = stream.await {
                error!("{e}");
            }
        });
        messages.push(stream);
    }

    futures_util::future::join_all(messages).await;
}
