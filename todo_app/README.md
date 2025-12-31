# TODO app

Written in Dioxus, required (since 2.6) env vars are `IP`, `PORT`, `CHANGE_INTERVAL`, `IMAGE_PATH` and `BACKEND_URL`.

## Devlopment

Follow the [step-by-step](https://dioxuslabs.com/learn/0.7/getting_started/) to setup your workstation for Dx. To get styles download [tailwindcss](https://github.com/tailwindlabs/tailwindcss/releases/), and run

```sh
tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

then serve the app via

```sh
IP=127.0.0.1 \
PORT=3000 \
CHANGE_INTERVAL=600 \
IMAGE_PATH=data/image \
BACKEND_URL=http://localhost:3001/todos \
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
