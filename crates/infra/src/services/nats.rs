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

        let mut consumers = vec![];

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

            // create stream
            //
            for consumer in stream_config.consumers.as_ref() {
                let durable_name = consumer.durable.as_ref().map(|v| v.to_string());
                let deliver_subject = consumer.deliver_subject.as_ref().map(|v| {
                    log::debug!("configuring push-based consumer = {}", consumer.name);
                    v.to_string()
                });

                let config: async_nats::jetstream::consumer::Config =
                    async_nats::jetstream::consumer::Config {
                        durable_name,
                        deliver_subject,
                        ..Default::default()
                    };

                let consumer = stream
                    .get_or_create_consumer(consumer.name.as_ref(), config)
                    .await
                    .unwrap();

                consumers.push(consumer);
            }
        }
        log::debug!("[NATS] {} consumers configured", consumers.len());

        self.nats_jetstream = Some(jetstream);
        self.nats_jetstream_consumers = consumers;
        Ok(self)
    }
}
