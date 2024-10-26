use std::any::Any;

pub trait Node {
    // const NAME: &'static str; // const UUID: u64;
    // type Input;
    // type Output;

    fn node_name(&self) -> &'static str;
    fn node_instance_id(&self) -> u64;
    async fn run(&self);
    /// Call `subscribe()` to obtain
    fn subscribe(&self) -> tokio::sync::broadcast::Receiver<dyn Any>;
}

pub(crate) struct BaseNode {
    pub(crate) instance_id: u64,
}