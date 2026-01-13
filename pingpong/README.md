# Pingpong

You probably only need `rustc` or `rustup` for this. Then you can run it with

```sh
IP=127.0.0.1 \
PORT=3000 \
DATABASE_URL=postgres://postgres:password@localhost/postgres \
cargo run
```

or deploy via

```sh
kubectl apply -f manifests
kubectl apply -f ../log_output/manifests/ingress.yml
```

and access at http://localhost:8081/pingpong, but first:

```sh
kubectl delete -f ../todo_app/ingress.yml
```
