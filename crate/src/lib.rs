//! Generated gRPC code for the Zerpto contracts. No behaviour lives here: see
//! `AGENTS.md`. One module per package, each behind its Cargo feature.
//! `common` is always compiled.

/// Types shared by two or more services (`zerpto.common.v1`).
pub mod common {
    pub mod v1 {
        tonic::include_proto!("zerpto.common.v1");
    }
}

/// Chart of accounts and accounting period lookups, owned by the accounting
/// service (`zerpto.accounting.v1`).
#[cfg(feature = "accounting")]
pub mod accounting {
    pub mod v1 {
        tonic::include_proto!("zerpto.accounting.v1");
    }
}

/// Tenant entitlements and the feature catalogue, owned by the billing service
/// (`zerpto.billing.v1`).
#[cfg(feature = "billing")]
pub mod billing {
    pub mod v1 {
        tonic::include_proto!("zerpto.billing.v1");
    }
}

/// Liveness probe (`zerpto.health.v1`).
#[cfg(feature = "health")]
pub mod health {
    pub mod v1 {
        tonic::include_proto!("zerpto.health.v1");
    }
}

/// Authorization checks, seats, phone verification and tenant settings, owned by
/// the IAM service (`zerpto.iam.v1`).
#[cfg(feature = "iam")]
pub mod iam {
    pub mod v1 {
        tonic::include_proto!("zerpto.iam.v1");
    }
}

/// Items, warehouses, stock on hand and stock movements, owned by the inventory
/// service (`zerpto.inventory.v1`).
#[cfg(feature = "inventory")]
pub mod inventory {
    pub mod v1 {
        tonic::include_proto!("zerpto.inventory.v1");
    }
}

/// Purchase and sales order reads, owned by the order service
/// (`zerpto.order.v1`).
#[cfg(feature = "order")]
pub mod order {
    pub mod v1 {
        tonic::include_proto!("zerpto.order.v1");
    }
}

/// Customer and vendor lookups, owned by the partner service (`zerpto.partner.v1`).
#[cfg(feature = "partner")]
pub mod partner {
    pub mod v1 {
        tonic::include_proto!("zerpto.partner.v1");
    }
}

/// Region tree and address checks, owned by the reference-data service
/// (`zerpto.referencedata.v1`).
#[cfg(feature = "reference-data")]
pub mod referencedata {
    pub mod v1 {
        tonic::include_proto!("zerpto.referencedata.v1");
    }
}
