# Changelog

## v0.1.0

First release. `health` and `authorization` moved here from `zerpto-core`. `health` is unchanged on the wire. The
identity package was renamed to `iam`, so its gRPC path changes (see below).

### Packages

- `zerpto.health.v1` (`Health.Check`), feature `health`.
- `zerpto.iam.v1`, feature `iam`. Owned by the IAM service.
  - `Authorization.Authorize`. It was `zerpto.identity.v1` in `zerpto-core`: the messages are the same, but the call
    path is now `/zerpto.iam.v1.Authorization/Authorize`, so the server and every client must move to this crate
    together.
  - `Tenants.GetTenant` (`tenants.proto`). The tenant's base currency.
  - `Seats.GetSeatsInUse` (`seats.proto`). Billing is the client. Returns the seats a tenant uses: active members (not
    deactivated) plus pending invitations not yet accepted. An unknown tenant answers 0.
  - `PhoneVerifications.GetPhoneVerification` (`phone_verifications.proto`). Billing is the client. Returns whether the
    user has a verified phone and a stable lowercase hex hash of the normalized number (empty when no phone; never the
    number itself), so billing can allow one free trial per phone. An unknown user answers `verified = false`.
- `zerpto.billing.v1` (`Entitlements.GetTenantEntitlements`, `Entitlements.GetTrialAvailability`,
  `Entitlements.ListFeatures`), feature `billing`. Owned by the billing service. Lets IAM ask a tenant's status,
  whether paid features are locked and its seat limit, whether a user can still start a trial, and read the feature
  catalogue (key, minimum plan, read or write). IAM applies the rules (owner check, seat count, permission filtering);
  billing never reads IAM data.
- `zerpto.referencedata.v1` (`Regions.ResolveAddress`), feature `reference-data`. Owned by the reference-data service.
  Lets a service (IAM first, later warehouses and partners) check an address against the region tree without repeating
  the depth rule. A bad address is an `Outcome`, not a gRPC error.
- `zerpto.partner.v1`, feature `partner`.
  - `Vendors.GetVendor`. Whether a tenant's vendor is active, archived or unknown. `GetVendorResponse` also carries the
    `Vendor`.
  - `Customers.GetCustomer`. The same status for a customer.
- `zerpto.inventory.v1`, feature `inventory`.
  - `Items.BatchGetItems`. Status, purchase and sales account and stocked flag of items. `Item` also carries the item's
    details (SKU, name, type, units, prices and so on).
  - `Stock.BatchGetStockOnHand`. The quantity of items on hand.
  - `Warehouses.GetWarehouse` (`warehouses.proto`). Whether a warehouse is active, archived or unknown, and its
    details.
  - `StockPostings.PostStockMovements` (`stock_postings.proto`). Applies the stock movements of a goods receipt or
    delivery once per source document, all or nothing, and returns the cost each was valued at.
- `zerpto.accounting.v1`, feature `accounting`.
  - `Accounts.BatchGetAccounts`. Whether accounts accept postings.
  - `Periods.GetPeriodForDate` (`periods.proto`). Whether a date's period is open or closed.
- `zerpto.order.v1` (`PurchaseOrders.BatchGetPurchaseOrderLines`, `PurchaseOrders.BatchGetPurchaseOrders`,
  `SalesOrders.GetSalesOrder`), feature `order`. What fulfillment needs to check receipts and deliveries, and to show a
  receipt's purchase orders.
- `zerpto.common.v1`: types shared by services, starting with `Address`. Always compiled.

The order and fulfillment services are the clients of `partner`, `inventory`, `accounting` and `order`. No server
implements those four packages yet.

### Crate

- Features: one per service package, plus `all` for every service. None is on by default.
- Depends on `prost-types`, which the generated code needs for `google.protobuf.Timestamp`.

