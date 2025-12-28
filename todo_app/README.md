# TODO app

Written in Dioxus, overridable env vars are `IP`, `PORT`, `CHANGE_INTERVAL`, `IMAGE_PATH` and `BACKEND_URL`.

## Devlopment

Follow the [step-by-step](https://dioxuslabs.com/learn/0.7/getting_started/) to setup your workstation for Dx. To get styles download [tailwindcss](https://github.com/tailwindlabs/tailwindcss/releases/), and run

```sh
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

then serve the app via

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
