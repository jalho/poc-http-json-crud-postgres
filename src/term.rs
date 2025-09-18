pub struct Actor {}

impl Actor {
    pub fn hook() -> Self {
        Self {}
    }

    pub async fn work(self) -> Summary {
        todo!();
    }
}

pub struct Summary;
