# Zerpto Proto

The gRPC contracts between Zerpto services, and the Rust crate `zerpto-proto` generated from them. REST for browsers
and third parties is in `com.zerpto.openapi`. Rules for changing a contract are in [AGENTS.md](AGENTS.md).

## Layout

```
proto/zerpto/
  common/v1/common.proto             shared types (postal address)
  accounting/v1/accounts.proto       chart of accounts lookups, owned by accounting
  accounting/v1/periods.proto        whether a date can be posted to, owned by accounting
  billing/v1/entitlements.proto      tenant entitlements and feature catalogue, owned by billing
  health/v1/health.proto             liveness probe
  iam/v1/authorization.proto         authorization checks, owned by the IAM service
  iam/v1/phone_verifications.proto   phone verified flag and phone hash, owned by the IAM service
  iam/v1/seats.proto                 seats in use per tenant, owned by the IAM service
  iam/v1/tenants.proto               tenant settings, owned by the IAM service
  inventory/v1/items.proto           item lookups, owned by inventory
  inventory/v1/stock.proto           quantity on hand, owned by inventory
  inventory/v1/stock_postings.proto  stock movements of other services' documents, owned by inventory
  inventory/v1/warehouses.proto      warehouse lookups, owned by inventory
  order/v1/purchase_orders.proto     purchase order reads, owned by order
  order/v1/sales_orders.proto        sales order reads, owned by order
  partner/v1/customers.proto         customer lookups, owned by partner
  partner/v1/vendors.proto           vendor lookups, owned by partner
  referencedata/v1/regions.proto     region tree and address checks, owned by reference-data
crate/                               the zerpto-proto crate (build.rs + src/lib.rs)
buf.yaml                             lint and breaking-change rules
```

Each service owns `proto/zerpto/<service>/v1/`. The package is `zerpto.<service>.v1`.

| Package | Feature | Owner (serves it) | Clients |
| --- | --- | --- | --- |
| `zerpto.accounting.v1` | `accounting` | accounting (not served yet) | order, fulfillment |
| `zerpto.billing.v1` | `billing` | billing | IAM (enterprise only; community IAM does not call it) |
| `zerpto.health.v1` | `health` | every service | any |
| `zerpto.iam.v1` | `iam` | IAM | every service |
| `zerpto.inventory.v1` | `inventory` | inventory (not served yet) | order, fulfillment |
| `zerpto.order.v1` | `order` | order (not served yet) | fulfillment |
| `zerpto.partner.v1` | `partner` | partner (not served yet) | order, fulfillment |
| `zerpto.referencedata.v1` | `reference-data` | reference-data | IAM; warehouses and partners later |

`zerpto.common.v1` has no feature: it is always compiled.

Events are not defined here. They travel as JSON over NATS through the shared outbox of `zerpto-core`.

## Use it from a Rust service

Needs `protoc` on the PATH (`brew install protobuf`). Pick the features you need; none are on by default.

```toml
[workspace.dependencies]
zerpto-proto = { git = "<repo url>", tag = "v0.1.0", features = ["iam"] }
```

Then in the service's `Cargo.toml`: `zerpto-proto = { workspace = true }`. Update by bumping `tag`.

Call a service (a client):

```rust
use zerpto_proto::iam::v1::{authorization_client::AuthorizationClient, AuthorizeRequest, Decision};

let mut client = AuthorizationClient::connect("http://iam:50051").await?;
let reply = client
    .authorize(AuthorizeRequest {
        user_id: user_id.to_string(),
        tenant_id: tenant_id.to_string(),
        permission_keys: vec!["expenses.read".into()],
    })
    .await?
    .into_inner();
if reply.decision() != Decision::Allowed {
    // deny
}
```

Serve a service (the owner):

```rust
use zerpto_proto::iam::v1::{
    authorization_server::{Authorization, AuthorizationServer},
    AuthorizeRequest, AuthorizeResponse,
};
use tonic::{Request, Response, Status};

struct Authz;

#[tonic::async_trait]
impl Authorization for Authz {
    async fn authorize(&self, req: Request<AuthorizeRequest>) -> Result<Response<AuthorizeResponse>, Status> {
        // ...
        todo!()
    }
}

tonic::transport::Server::builder()
    .add_service(AuthorizationServer::new(Authz))
    .serve(addr)
    .await?;
```

## Check it

```sh
buf lint
buf breaking --against '.git#tag=<last>'
cargo build --all-features
cargo build --no-default-features --features iam
```
