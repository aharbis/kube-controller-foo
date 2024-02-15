use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    kind = "Foo",
    group = "aharbis.dev",
    version = "v1alpha1",
    namespaced,
    status = "FooStatus"
)]
pub struct FooSpec {
    pub dummy: String,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug, JsonSchema)]
pub struct FooStatus {}
