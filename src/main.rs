#[cxx::bridge(namespace = "org::dojo")]
mod ffi {
    struct FieldElement {
        pub inner: [u8; 32],
    }

    struct EntityModel {
        pub model: String,
        pub keys: Vec<FieldElement>,
    }

    extern "Rust" {
        type Client;

        // fn set_entities_to_sync(self: &mut ClientBuilder, entities: Vec<EntityModel>);
        fn build(
            torii_url: String,
            rpc_url: String,
            world: FieldElement,
            entities: Vec<EntityModel>,
        ) -> RustFutureClient;

        // fn metadata(self: &Client) -> WorldMetadata;
        // // fn subscribed_entities(&self) -> HashSet<EntityModel>;
        // // fn entity(&self, model: &str, keys: &[FieldElement]) -> Option<Ty>;
        // fn start_subscription(self: &mut Client) -> Result<SubscriptionService>;
        // fn add_entities_to_sync(self: &mut Client, entities: Vec<EntityModel>) -> Result<()>;
        // fn remove_entities_to_sync(self: &mut Client, entities: Vec<EntityModel>) -> Result<()>;
        // fn storage(&self) -> Arc<ModelStorage>;
    }

    unsafe extern "C++" {
        type RustFutureClient = crate::RustFutureClient;
    }
}

use dojo_types::schema::EntityModel;
use starknet::core::types::FieldElement;
use torii_client::client;
use torii_client::client::error::Error;

struct ClientBuilder {
    builder: client::ClientBuilder,
}
struct Client {
    client: client::Client,
}

fn build(
    torii_url: String,
    rpc_url: String,
    world: FieldElement,
    entities: Vec<ffi::EntityModel>,
) -> Result<Box<Client>, Error> {
    RustFutureClient::fallible(async move {
        let client = client::Client::builder()
            .set_entities_to_sync(
                entities
                    .iter()
                    .map(|e| EntityModel {
                        model: e.model.clone(),
                        keys: e
                            .keys
                            .iter()
                            .map(|k| FieldElement::from_bytes_be(&k.inner).unwrap())
                            .collect::<_>(),
                    })
                    .collect(),
            )
            .build(torii_url, rpc_url, world)
            .await?;

        Ok(Box::new(client))
    })
}

#[cxx_async::bridge]
unsafe impl Future for RustFutureClient {
    type Output = Result<Box<Client>, Error>;
}

fn main() {}
