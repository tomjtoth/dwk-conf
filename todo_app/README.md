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

## DBaaS vs DIY "comparison"

1. Configuration / initialization
   - DBaas
     - pros:
       - configurable via the browser GUI
       - enforces restrictive best-practices rules
     - cons:
       - no tweaks possible?

   - DIY
     - pros:
       - full control over the Postgres configurations
       - same as above, probably possible to tweak more, than in DBaaS
     - cons:
       - maintain the manifests for PVCs, statefulSets
       - additional secrets must be defined in repo, e.g. DATABASE_URL

1. Maintenance
   - DBaaS
     - pros:
       - integrated monitoring & alerts?
       - maintenance simplified to a restart?
     - cons:
       - might be outdated DB app, similarly to packages on Debian vs Arch
   - DIY
     - pros:
       - 100% control over updates - whether you like it...
     - cons:
       - 100% control over updates - whether you don't like it...
