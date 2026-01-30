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
        --port '9082:30080@agent:0' \
        --port '9443:443@loadbalancer'

    # istoi installation
    # istioctl install \
    #     --skip-confirmation \
    #     --set profile=ambient \
    #     --set values.global.platform=k3d

    # kubectl apply --server-side -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.4.0/experimental-install.yaml

    # knative installation
    kubectl apply -f https://github.com/knative/serving/releases/download/knative-v1.21.0/serving-crds.yaml
    kubectl apply -f https://github.com/knative/serving/releases/download/knative-v1.21.0/serving-core.yaml

    kubectl apply -l knative.dev/crd-install=true -f https://github.com/knative-extensions/net-istio/releases/download/knative-v1.21.0/istio.yaml
    kubectl apply -f https://github.com/knative-extensions/net-istio/releases/download/knative-v1.21.0/istio.yaml

    kubectl apply -f https://github.com/knative-extensions/net-istio/releases/download/knative-v1.21.0/net-istio.yaml

    kubectl patch configmap/config-network \
        --namespace knative-serving \
        --type merge \
        --patch '{"data":{"ingress-class":"istio.ingress.networking.knative.dev"}}'

    kubectl --namespace istio-system get service istio-ingressgateway

    kubectl apply -f https://github.com/knative/serving/releases/download/knative-v1.21.0/serving-default-domain.yaml
    kubectl set env deployment --all -n knative-serving KUBERNETES_MIN_VERSION=1.31.0-0

    kubectl get pods -n knative-serving

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
