#/bin/bash

cd "$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
source .parser.sh

if [ -v DELETE ]; then
    k3d cluster delete
fi

if [ -v CREATE ]; then
    k3d cluster create \
        --api-port 6550 \
        --k3s-arg '--disable=traefik@server:*' \
        --agents 2 \
        --port '9080:80@loadbalancer' \
        --port '9443:443@loadbalancer'

    istioctl install \
        --skip-confirmation \
        --set profile=ambient \
        --set values.global.platform=k3d

    # kubectl get crd gateways.gateway.networking.k8s.io &> /dev/null || \
    kubectl apply --server-side -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.4.0/experimental-install.yaml

    # install_bloatware k3d-k3s-default
fi

k3s(){
    kubectl $1 --context k3d-k3s-default -f "$2"
}

if [ -v DELETE_ALL ]; then
    DELETE_THESE=({!(ns|pv),pv,ns}/manifests/{{ing,{svc,service}}*,!(ing*|svc*|service*)}.yml)
fi

if [ -v APPLY_ALL ]; then
    APPLY_THESE=({ns,pv,!(ns|pv)}/manifests/!(*.gke).yml)
fi

if [ -v DELETE_THESE ]; then
    for manifest in "${DELETE_THESE[@]}"; do
        if [[ "$manifest" =~ \.gke\.yml$ ]]; then
            continue
        fi
        k3s delete "$manifest"
    done
fi

if [ -v APPLY_THESE ]; then
    for manifest in "${APPLY_THESE[@]}"; do
        k3s apply "$manifest"
    done
fi

stop_timer
