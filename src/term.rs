pub struct Actor {
    global_cancellation_token: tokio_util::sync::CancellationToken,
    chan_trigger: (
        tokio::sync::mpsc::Sender<TriggerGlobalCancellation>,
        tokio::sync::mpsc::Receiver<TriggerGlobalCancellation>,
    ),
}

impl Actor {
    pub fn hook() -> Self {
        Self {
            global_cancellation_token: tokio_util::sync::CancellationToken::new(),
            chan_trigger: tokio::sync::mpsc::channel::<TriggerGlobalCancellation>(1),
        }
    }

    pub fn get_handle(&self) -> Handle {
        Handle {
            read: self.global_cancellation_token.child_token(),
            write: self.chan_trigger.0.clone(),
        }
    }

    pub async fn work(mut self) -> Summary {
        let mut sigint = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())
            .expect("failed to hook into SIGINT");
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to hook into SIGTERM");

        tokio::select! {
            _ = sigint.recv() => {
                log::info!("SIGINT");
            }
            _ = sigterm.recv() => {
                log::info!("SIGTERM");
            }
            received = self.chan_trigger.1.recv() => {
                if let Some(_triggerer) = received {
                    log::info!("Cancellation triggered another actor");
                } else {
                    log::error!("Cancellation trigger channel closed without signal");
                }
            }
        }
        self.global_cancellation_token.cancel();

        Summary
    }
}

pub struct Summary;

pub struct Handle {
    read: tokio_util::sync::CancellationToken,
    write: tokio::sync::mpsc::Sender<TriggerGlobalCancellation>,
}

impl Handle {
    pub async fn trigger_termination(&self, triggerer: TriggerGlobalCancellation) {
        if let Err(err) = self.write.send(triggerer).await {
            log::error!("{err}");
        }
    }

    pub fn token(self) -> tokio_util::sync::CancellationToken {
        self.read
    }
}

pub enum TriggerGlobalCancellation {
    WebServer,
}
