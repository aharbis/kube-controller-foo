use futures::StreamExt;
use kube::{
    runtime::controller::{Action, Controller},
    Api, Client, ResourceExt,
};
use kube_controller_foo::{foo::Foo, Error, Result};
use std::{sync::Arc, time::Duration};

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let foos = Api::<Foo>::all(client);

    Controller::new(foos.clone(), Default::default())
        .run(reconcile, error_policy, Arc::new(()))
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

async fn reconcile(obj: Arc<Foo>, _ctx: Arc<()>) -> Result<Action> {
    println!("reconcile request: {}", obj.name_any());
    Ok(Action::requeue(Duration::from_secs(10)))
}

fn error_policy(_object: Arc<Foo>, _err: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(5))
}
