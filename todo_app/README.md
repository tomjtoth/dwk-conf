# TODO app

Written in Dioxus, overridable env vars are `CHANGE_INTERVAL`, `IMAGE_PATH` and `PORT`.

## Dev

Follow [their guide](https://dioxuslabs.com/learn/0.7/getting_started/) to setup your workstation. To get styles download [tailwindcss](https://github.com/tailwindlabs/tailwindcss/releases/), and run

```sh
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

then run via

```sh
dx serve
```

## Deploy

```sh
kubectl apply -f manifests
```

and access at http://localhost:8081, but first:

```sh
kubectl delete -f ../log_output/ingress.yml
```
