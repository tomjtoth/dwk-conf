# TODO app

You probably only need `rustc` or `rustup` for this. Then you can run it with

```sh
cargo run
```

or deploy via

```sh
kubectl apply -f manifests
```

and access at http://localhost:8081, but first:

```sh
kubectl delete -f ../log_output/ingress.yml
```
