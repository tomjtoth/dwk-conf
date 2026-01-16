#/bin/bash

USAGE="proper usage is: $(basename $0) [OPTIONS]

where OPTIONS are:
--rm-cluster                        teardown the cluster
--add-cluster                       add cluster
--reset                             teardown + add cluster
-a | --apply path/to/manifest       apply specific manifest
-d | --delete path/to/manifest      delete specific manifest
--delete-all                        delete all manifests
"

APPLY_ALL=1

while [ $# -gt 0 ]; do
    case "$1" in 
        (--rm-cluster) 
            RM_CLUSTER=1 
            unset APPLY_ALL
            ;;

        (--add-cluster) ADD_CLUSTER=1;;

        (--reset) RM_CLUSTER=1 ADD_CLUSTER=1;;

        (--delete-all)
            unset APPLY_ALL
            DELETE_ALL=1
            ;;

        (-a|--apply)
            unset APPLY_ALL
            if [ ! -f "$2" ]; then
                WRONG_FLAGS+=("--apply MANI <- MANI must be an existing file, got \"$2\"")
            else
                APPLY_THESE+=("$2")
            fi
            shift
            ;;

        (-d|--delete)
            unset APPLY_ALL
            if [ ! -f "$2" ]; then
                WRONG_FLAGS+=("--delete MANI <- MANI must be an existing file, got \"$2\"")
            else
                DELETE_THESE+=("$2")
            fi
            shift
            ;;

        (*) WRONG_FLAGS+=("$1")
    esac
    shift
done

if [ -v WRONG_FLAGS ]; then
    printf '%s\n' \
        "the following flags are wrong/unknown: ${WRONG_FLAGS[*]}" \
        "" \
        "$USAGE"
    exit 1
fi

START_TIME=$(date +%s)

if [ -v RM_CLUSTER ]; then
    k3d cluster delete
fi

if [ -v ADD_CLUSTER ]; then
    k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
fi

k3s_op(){
    kubectl --context k3d-k3s-default "$@"
}

script_dir="$(dirname "$(realpath "${BASH_SOURCE[0]}")")"
shopt -s extglob

if [ -v DELETE_ALL ]; then
    for manifest in "$script_dir"/{!(ns),ns}/manifests/!(*.gke).yml; do
        k3s_op delete -f $manifest
    done
fi

if [ -v APPLY_ALL ]; then
    for manifest in "$script_dir"/{ns,pv,!(ns|pv)}/manifests/!(*.gke).yml; do
        k3s_op apply -f $manifest
    done
fi

if [ -v APPLY_THESE ]; then
    for manifest in "${APPLY_THESE[@]}"; do
        k3s_op apply -f "$manifest"
    done
fi

if [ -v DELETE_THESE ]; then
    for manifest in "${DELETE_THESE[@]}"; do
        k3s_op delete -f "$manifest"
    done
fi

echo "ops took $(($(date +%s) - $START_TIME)) seconds to finish"
