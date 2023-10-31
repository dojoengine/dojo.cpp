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
        async fn build(
            torii_url: String,
            rpc_url: String,
            world: FieldElement,
            entities: Vec<EntityModel>,
        ) -> Result<Box<Client>>;

        // fn metadata(self: &Client) -> WorldMetadata;
        // // fn subscribed_entities(&self) -> HashSet<EntityModel>;
        // // fn entity(&self, model: &str, keys: &[FieldElement]) -> Option<Ty>;
        // fn start_subscription(self: &mut Client) -> Result<SubscriptionService>;
        // fn add_entities_to_sync(self: &mut Client, entities: Vec<EntityModel>) -> Result<()>;
        // fn remove_entities_to_sync(self: &mut Client, entities: Vec<EntityModel>) -> Result<()>;
        // fn storage(&self) -> Arc<ModelStorage>;
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

async fn build(
    torii_url: String,
    rpc_url: String,
    world: FieldElement,
    entities: Vec<ffi::EntityModel>,
) -> Result<Box<Client>, Error> {
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
}

fn main() {}
