# TODO-reminder

This is the hourly run cronjob.
You probably only need `rustc` or `rustup` for this. Then you can run it with

```sh
DATABASE_URL=postgres://postgres:password@localhost/postgres \
cargo run
```

or deploy via

```sh
kubectl apply -f manifests
```
