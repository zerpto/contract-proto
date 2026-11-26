# Changelog

## 0.5.0

Additive; no service has to change. The order and fulfillment services are the clients. No server implements these
yet. Rust code that builds `inventory.v1.Item` or `partner.v1.GetVendorResponse` with a struct literal (test stubs)
must add `..Default::default()`; the wire format is only extended.

- `zerpto.partner.v1` (`Vendors.GetVendor`), feature `partner`. Whether a tenant's vendor is active, archived or
  unknown.
- `zerpto.partner.v1` (`Customers.GetCustomer`). The same for a customer.
- `zerpto.inventory.v1` (`Items.BatchGetItems`, `Stock.BatchGetStockOnHand`), feature `inventory`. Status, purchase and
  sales account and stocked flag of items, and their quantity on hand.
- `zerpto.accounting.v1` (`Accounts.BatchGetAccounts`), feature `accounting`. Whether accounts accept postings.
- `zerpto.iam.v1` `Tenants.GetTenant` (new file `tenants.proto`), feature `iam`. The tenant's base currency.
- `zerpto.order.v1` (`PurchaseOrders.BatchGetPurchaseOrderLines`, `PurchaseOrders.BatchGetPurchaseOrders`,
  `SalesOrders.GetSalesOrder`), new feature `order`. What fulfillment needs to check receipts and deliveries, and to
  show a receipt's purchase orders.
- `zerpto.inventory.v1` `Warehouses.GetWarehouse` (new file `warehouses.proto`). Whether a warehouse is active,
  archived or unknown, and its details.
- `zerpto.inventory.v1` `StockPostings.PostStockMovements` (new file `stock_postings.proto`). Applies the stock movements of a goods receipt or delivery once
  per source document, all or nothing, and returns the cost each was valued at.
- `zerpto.inventory.v1` `Item` also carries the item's details (SKU, name, type, units, prices and so on).
- `zerpto.accounting.v1` `Periods.GetPeriodForDate` (new file `periods.proto`). Whether a date's period is open or
  closed.
- `zerpto.partner.v1` `GetVendorResponse` also carries the `Vendor`.
- `zerpto.common.v1` has its first type, `Address`. The module `common` is always compiled.
- The crate depends on `prost-types`, which the generated code needs for `google.protobuf.Timestamp`.
- `all` now includes `accounting`, `inventory`, `order` and `partner`.

## 0.4.0

Additive; no service has to change. Billing is the client; IAM implements the servers.

- `zerpto.iam.v1` `Seats.GetSeatsInUse` (new file `seats.proto`), feature `iam`. Returns the seats a tenant uses:
  active members (not deactivated) plus pending invitations not yet accepted. An unknown tenant answers 0.
- `zerpto.iam.v1` `PhoneVerifications.GetPhoneVerification` (new file `phone_verifications.proto`), feature `iam`.
  Returns whether the user has a verified phone and a stable lowercase hex hash of the normalized number (empty when
  no phone; never the number itself), so billing can allow one free trial per phone. An unknown user answers
  `verified = false`.

## 0.3.0

Additive; no service has to change.

- `zerpto.billing.v1` (`Entitlements.GetTenantEntitlements`, `Entitlements.GetTrialAvailability`,
  `Entitlements.ListFeatures`), feature `billing`. Owned by the billing service. Lets IAM ask a tenant's status,
  whether paid features are locked and its seat limit, whether a user can still start a trial, and read the feature
  catalogue (key, minimum plan, read or write). IAM applies the
  rules (owner check, seat count, permission filtering); billing never reads IAM data.
- `all` now includes `billing`.

## 0.2.0

Additive; no service has to change.

- `zerpto.referencedata.v1` (`Regions.ResolveAddress`), feature `reference-data`. Owned by the reference-data service.
  Lets a service (IAM first, later warehouses and partners) check an address against the region tree without
  repeating the depth rule. A bad address is an `Outcome`, not a gRPC error.
- `all` now includes `reference-data`.

## 0.1.0

First release. `health` and `authorization` moved here from `zerpto-core`. `health` is unchanged on the wire. The
identity package was renamed to `iam`, so its gRPC path changes (see below).

- `zerpto.health.v1` (`Health.Check`), feature `health`.
- `zerpto.iam.v1` (`Authorization.Authorize`), feature `iam`. Owned by the IAM service. It was `zerpto.identity.v1` in
  `zerpto-core`: the messages are the same, but the call path is now `/zerpto.iam.v1.Authorization/Authorize`, so the
  server and every client must move to this crate together.
- `zerpto.common.v1`: placeholder, no types yet.
- Features: `health`, `iam`, `all`. None is on by default.
