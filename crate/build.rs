//! Generates the gRPC code from `../proto`. Needs `protoc` on the PATH
//! (`brew install protobuf`). Server and client code are both generated: the
//! owning service uses the server, everyone else the client.
//!
//! `prost` writes one file per package, so only the files of the enabled
//! features are compiled. `common` is always compiled.

/// Cargo feature name and the proto files it enables. Add a row per service.
const SERVICES: &[(&str, &[&str])] = &[
    (
        "ACCOUNTING",
        &[
            "../proto/zerpto/accounting/v1/accounts.proto",
            "../proto/zerpto/accounting/v1/periods.proto",
        ],
    ),
    (
        "BILLING",
        &["../proto/zerpto/billing/v1/entitlements.proto"],
    ),
    ("HEALTH", &["../proto/zerpto/health/v1/health.proto"]),
    (
        "IAM",
        &[
            "../proto/zerpto/iam/v1/authorization.proto",
            "../proto/zerpto/iam/v1/phone_verifications.proto",
            "../proto/zerpto/iam/v1/seats.proto",
            "../proto/zerpto/iam/v1/tenants.proto",
        ],
    ),
    (
        "INVENTORY",
        &[
            "../proto/zerpto/inventory/v1/items.proto",
            "../proto/zerpto/inventory/v1/stock.proto",
            "../proto/zerpto/inventory/v1/stock_postings.proto",
            "../proto/zerpto/inventory/v1/warehouses.proto",
        ],
    ),
    (
        "ORDER",
        &[
            "../proto/zerpto/order/v1/purchase_orders.proto",
            "../proto/zerpto/order/v1/sales_orders.proto",
        ],
    ),
    (
        "PARTNER",
        &[
            "../proto/zerpto/partner/v1/customers.proto",
            "../proto/zerpto/partner/v1/vendors.proto",
        ],
    ),
    // Cargo turns the `-` of `reference-data` into `_` in the env var name.
    (
        "REFERENCE_DATA",
        &["../proto/zerpto/referencedata/v1/regions.proto"],
    ),
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=../proto");

    let mut protos: Vec<&str> = vec!["../proto/zerpto/common/v1/common.proto"];
    for (feature, files) in SERVICES {
        // Cargo sets `CARGO_FEATURE_<NAME>` (upper case) for each enabled feature.
        if std::env::var_os(format!("CARGO_FEATURE_{feature}")).is_some() {
            protos.extend_from_slice(files);
        }
    }

    tonic_prost_build::configure().compile_protos(&protos, &["../proto"])?;
    Ok(())
}
