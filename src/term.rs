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

    pub async fn work(self) -> Summary {
        todo!("hook SIGINT and SIGTERM, and chan_trigger.rx to self.global_cancellation_token");
    }
}

pub struct Summary;

pub struct Handle {
    read: tokio_util::sync::CancellationToken,
    write: tokio::sync::mpsc::Sender<TriggerGlobalCancellation>,
}

impl Handle {
    pub async fn trigger_termination(&self) {
        if let Err(err) = self.write.send(TriggerGlobalCancellation).await {
            eprintln!("{err}");
        }
    }

    pub fn token(self) -> tokio_util::sync::CancellationToken {
        self.read
    }
}

pub struct TriggerGlobalCancellation;
