install_bloatware(){
    if [ $# -gt 0 ]; then
        local prev_default=$(kubectl config get-contexts | awk '$1 == "*" {print $2}')
        kubectl config use-context $1
    fi

    kubectl create namespace prometheus
    helm install prometheus-community/kube-prometheus-stack \
        --generate-name \
        --namespace prometheus

    kubectl create namespace argo-rollouts
    kubectl apply -n argo-rollouts \
        -f https://github.com/argoproj/argo-rollouts/releases/latest/download/install.yaml

    # helm install --set auth.enabled=false my-nats oci://registry-1.docker.io/bitnamicharts/nats

    # helm upgrade --install my-nats oci://registry-1.docker.io/bitnamicharts/nats   --set auth.enabled=false   --set image.registry=docker.io   --set image.repository=bitnamilegacy/nats   --set image.tag=2.11.8-debian-12-r0


    kubectl create namespace argocd
    kubectl apply -n argocd -f https://raw.githubusercontent.com/argoproj/argo-cd/stable/manifests/install.yaml
    kubectl patch svc argocd-server -n argocd -p '{"spec": {"type": "LoadBalancer"}}'

    echo "argocd password:"
    kubectl get -n argocd secrets argocd-initial-admin-secret -o yaml

    if [ $# -gt 0 ]; then
        kubectl config use-context $prev_default
    fi
}