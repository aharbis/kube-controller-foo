# kube-controller-foo

Minimal Kubernetes controller used for troubleshooting and recreating issues with various open-source crates.

## Install `Foo` CRD

```
cargo run --bin crdgen | kubectl apply -f -
```

## Run controller

```
cargo run
```

## Create `test` custom resource

```
kubectl apply -f test.yaml
```

## Uninstall `Foo` CRD

```
cargo run --bin crdgen | kubectl delete -f -
```
