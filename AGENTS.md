# Zerpto Proto: working rules

Contract-first gRPC spec for Zerpto ERP: one source of truth for service-to-service calls. REST for browsers and
third parties lives in `com.zerpto.openapi`, not here. Services consume this repo as the Rust crate `zerpto-proto`,
by git tag, the same way they consume `zerpto-core`. The `.proto` files are the contract; the crate only generates code
from them.

## Layout

- `proto/zerpto/<service>/v1/`: the protos of one service. Package name is `zerpto.<service>.vN`, and the directory
  matches the package (`zerpto.iam.v1` is `proto/zerpto/iam/v1/`).
- `proto/zerpto/common/v1/`: types shared by two or more services. Never put a single service's type here.
- `crate/`: the `zerpto-proto` crate. `build.rs` compiles the protos, `src/lib.rs` has one `pub mod <service> { pub mod v1 }`
  per package, each behind a Cargo feature of the same name. `common` is always compiled.
- The root `Cargo.toml` is a workspace (`members = ["crate"]`) so the repo works as a git dependency.
- `buf.yaml`: lint (STANDARD, minus `SERVICE_SUFFIX`, explained in the file) and breaking (FILE).

## Ownership

- The owner of a package is the service that serves it. The owner changes the proto and implements the server. Every
  other service is a client and never edits that package without the owner's agreement.
- `iam` is owned by the IAM service. `billing` is owned by the billing service (the enterprise
  edition of IAM is its client; community IAM does not call it).
  `accounting`, `inventory` and `partner` are owned by those services (order and fulfillment are their
  clients). `order` is owned by the order service (fulfillment is its client). `health` is a shared example every
  service serves.

## Wire rules

- Ids are plain UUID strings, lowercase 8-4-4-4-12. The `ten-`/`usr-` style prefixes exist only on HTTP; never put one
  on the wire.
- Money and quantity are decimal strings (`"1250.50"`), never `float` or `double`. Say the scale in the field comment.
- Timestamps are `google.protobuf.Timestamp`. Dates without a time are a string `YYYY-MM-DD`; say so in the comment.
- The tenant is always an explicit request field (`tenant_id`), never ambient (no metadata header, no channel default).
- Every service, rpc, message, field and enum value has a comment, written for the person who calls it.
- Enums start with `*_UNSPECIFIED = 0` and every value is prefixed with the enum name (`DECISION_ALLOWED`).
- Fields are `snake_case`, messages `PascalCase`, rpcs `PascalCase` verbs (`Authorize`, `GetItem`).

## Compatibility

- Additive only. Services upgrade at their own pace, so a release must not break a service on the previous tag.
- Never renumber a field, never reuse a field number or name, never change a field's type.
- Removing a field: delete it and add `reserved <number>;` and `reserved "<name>";`.
- A breaking change is a new package `zerpto.<service>.v2` in `proto/zerpto/<service>/v2/`, next to `v1`. Keep `v1`
  until no service calls it. Add the module `v2` beside `v1` in `lib.rs`.

## Releasing

1. Update `CHANGELOG.md` and `version` in `crate/Cargo.toml`.
2. `git tag vX.Y.Z` (the push is the user's).
3. In each service: bump `tag = "vX.Y.Z"` in `[workspace.dependencies]`, then run its checks.

## Adding an rpc

1. Edit the proto in the owner's folder: the rpc, its request and response messages, all with comments.
2. `buf lint` and `buf breaking --against '.git#tag=<last>'`. Both must pass.
3. Release (see above): CHANGELOG, version bump, tag.
4. The owner implements the server method; callers bump the tag and use the generated client.

## Adding a service

1. Create `proto/zerpto/<service>/v1/<name>.proto` with `package zerpto.<service>.v1;`.
2. Add the feature `<service>` to `crate/Cargo.toml` and to `all`.
3. Add its row (feature name in upper case, `-` as `_`, proto files) to `SERVICES` in `crate/build.rs`.
4. Add `#[cfg(feature = "<service>")] pub mod <service> { pub mod v1 { tonic::include_proto!("zerpto.<service>.v1"); } }`
   to `crate/src/lib.rs`, and the feature name to the loop in `.github/workflows/ci.yaml`.
5. Lint, build, release.

## Commands (the user runs them)

Do not run any `cargo` command. Write the change and tell the user to run:

```sh
buf lint
buf breaking --against '.git#tag=<last>'
cargo build --all-features
cargo build --no-default-features --features <svc>
```

## Do not

- Do not edit generated code. It is built into `target/` and never committed.
- Do not copy a proto into a service. Depend on this crate with the feature you need.
- Do not put behaviour in this crate: no clients with logic, no server implementations, no channel helpers. Those live
  in `zerpto-core` and in the services.
- Do not use `float`/`double` for money or quantity, and do not put a prefixed id on the wire.
