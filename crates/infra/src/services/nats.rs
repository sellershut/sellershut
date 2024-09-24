use crate::ServicesBuilder;

impl ServicesBuilder {
    #[cfg(feature = "nats-core")]
    pub async fn with_nats_core(
        mut self,
        config: &crate::config::nats::Nats,
    ) -> Result<Self, crate::ServiceError> {
        log::trace!("initialising nats-core");

        let hosts = config.hosts();
        let client = async_nats::connect(hosts).await?;
        self.nats = Some(client);
        Ok(self)
    }

    #[cfg(feature = "nats-jetstream")]
    pub async fn with_nats_jetstream(
        mut self,
        config: &crate::config::nats::Nats,
    ) -> Result<Self, crate::ServiceError> {
        log::trace!("initialising nats-jetstream");

        let hosts = config.hosts();
        let client = async_nats::connect(hosts).await?;

        let jetstream = async_nats::jetstream::new(client);

        let mut pull_consumers_config = vec![];
        let mut push_consumers_config = vec![];
        let mut pull_consumers = vec![];
        let mut push_consumers = vec![];

        for stream_config in config.jetstream.as_ref() {
            log::debug!("creating stream");

            let stream = jetstream
                .get_or_create_stream(async_nats::jetstream::stream::Config {
                    name: stream_config.name.to_string(),
                    max_messages: stream_config.max_msgs,
                    subjects: stream_config
                        .subjects
                        .iter()
                        .map(String::from)
                        .collect::<Vec<_>>(),
                    max_bytes: stream_config.max_bytes,
                    ..Default::default()
                })
                .await
                .unwrap();

            for consumer in stream_config.consumers.as_ref() {
                let durable_name = consumer.durable.as_ref().map(|v| v.to_string());
                let deliver_subject = consumer.deliver_subject.as_ref().map(|v| {
                    log::debug!("configuring push-based consumer = {}", consumer.name);
                    v.to_string()
                });

                match deliver_subject {
                    Some(deliver_subject) => {
                        let cons = async_nats::jetstream::consumer::push::Config {
                            durable_name,
                            deliver_subject,
                            ..Default::default()
                        };
                        push_consumers_config.push((consumer.name.clone(), cons));
                    }
                    None => {
                        let cons = async_nats::jetstream::consumer::pull::Config {
                            durable_name,
                            ..Default::default()
                        };
                        pull_consumers_config.push((consumer.name.clone(), cons))
                    }
                }
            }

            pull_consumers = Vec::with_capacity(pull_consumers.len());
            for (name, config) in pull_consumers_config.iter() {
                let consumer = stream
                    .get_or_create_consumer(&name, config.clone())
                    .await
                    .unwrap();
                pull_consumers.push(consumer);
            }

            push_consumers = Vec::with_capacity(push_consumers.len());
            for (name, config) in push_consumers_config.iter() {
                let consumer = stream
                    .get_or_create_consumer(&name, config.clone())
                    .await
                    .unwrap();
                push_consumers.push(consumer);
            }
        }
        log::debug!("[NATS] {} pull consumers configured", pull_consumers.len());

        self.nats_jetstream = Some(jetstream);
        self.nats_jetstream_pull_consumers = pull_consumers;
        self.nats_jetstream_push_consumers = push_consumers;
        Ok(self)
    }
}
