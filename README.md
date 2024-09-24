## Overview

This is a simple cli tool that can query a remote registry to get a list of images, a list of tags for a specific image, the e-tag for a specific manifest
as wel las copy from file base oci or dockerv2 images to a remote registry and vice versa, it does not do any registry to registry copy 
and from disk to remote mirror registry

## Usage

This assumes you have already installed Rust (refer to https://www.rust-lang.org/tools/install)

Clone this repo

Ensure that you have the correct permissions set in the $XDG_RUNTIME_DIR/containers/auth.json file

Build binary

```bash
make build
```

### List all repositories

```bash

./target/release/registry-tool list-catalog --registry mac-fedora:8443 --namespace init/ubi9/ubi-micro  
 [ INFO  2024-09-24 21:41:21.411 ]   : init/openshift/graph-image
 [ INFO  2024-09-24 21:41:21.411 ]   : init/openshift/release
 [ INFO  2024-09-24 21:41:21.411 ]   : init/redhat/redhat-operator-index
 [ INFO  2024-09-24 21:41:21.411 ]   : init/openshift4/ose-kubernetes-nmstate-handler-rhel9
 [ INFO  2024-09-24 21:41:21.411 ]   : init/openshift4/nmstate-console-plugin-rhel8
 [ INFO  2024-09-24 21:41:21.411 ]   : init/openshift4/kubernetes-nmstate-rhel9-operator
 [ INFO  2024-09-24 21:41:21.411 ]   : init/openshift4/kubernetes-nmstate-operator-bundle
 [ INFO  2024-09-24 21:41:21.411 ]   : init/openshift4/ose-kube-rbac-proxy
 [ INFO  2024-09-24 21:41:21.411 ]   : init/albo/aws-load-balancer-controller-rhel8
 [ INFO  2024-09-24 21:41:21.411 ]   : init/albo/aws-load-balancer-rhel8-operator
 [ INFO  2024-09-24 21:41:21.411 ]   : init/albo/aws-load-balancer-operator-bundle
 [ INFO  2024-09-24 21:41:21.411 ]   : init/ubi9/ubi-micro
 [ INFO  2024-09-24 21:41:21.411 ]   : init/oc-mirror
 [ INFO  2024-09-24 21:41:21.411 ]   : library/init
 [ INFO  2024-09-24 21:41:21.411 ]   : init/openshift
 [ INFO  2024-09-24 21:41:21.411 ]   : init/ubi9


```

### List all tags for an image

```bash
./target/release/rust-registry-tool list-tags --registry mac-fedora:8443 --namespace init/openshift --name release
--query-params="/v2/init/openshift/release/tags/list?n=50&next_page=gAAAAABm8xZbbw7PRezKbb5KFnXmKbypG5wRG0qcGX5Dn2qTWo0OkIexSsbBG6jGsQQFpZ79fQa5s7iZjC6c2NQ65hyTr2sagJZua-HHjxbnDyKYzi31RYE%3D"
 [ INFO  2024-09-24 21:43:23.198 ]   : image init/openshift/release
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-vsphere-problem-detector-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-vsphere-csi-driver-syncer-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-vsphere-csi-driver-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-vsphere-cluster-api-controllers-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-vsphere-cloud-controller-manager-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-tools-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-thanos-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-tests-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-telemeter-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-sdn-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-route-controller-manager-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-rhel-coreos-extensions-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-rhel-coreos-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-prometheus-node-exporter-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-prometheus-config-reloader-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-prometheus-alertmanager-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-prometheus-x86_64
 [ INFO  2024-09-24 21:43:23.198 ]   : tag   4.15.8-x86_64-prom-label-proxy-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-powervs-machine-controllers-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-powervs-cloud-controller-manager-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-powervs-block-csi-driver-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-pod-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-ovn-kubernetes-microshift-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-ovn-kubernetes-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-ovirt-machine-controllers-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-ovirt-csi-driver-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-openstack-machine-api-provider-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-openstack-cluster-api-controllers-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-openstack-cloud-controller-manager-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-openstack-cinder-csi-driver-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-openshift-state-metrics-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-openshift-controller-manager-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-openshift-apiserver-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-olm-rukpak-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-olm-catalogd-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-oc-mirror-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-oauth-server-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-oauth-proxy-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-oauth-apiserver-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-nutanix-machine-controllers-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-nutanix-cloud-controller-manager-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-network-tools-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-network-metrics-daemon-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-network-interface-bond-cni-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-must-gather-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-multus-whereabouts-ipam-cni-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-multus-route-override-cni-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-multus-networkpolicy-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-multus-cni-x86_64
 [ INFO  2024-09-24 21:43:23.199 ]   : tag   4.15.8-x86_64-multus-admission-controller-x86_64

```

### List all tags for an image (using link pagination "--query-params")

```bash

./target/release/rust-registry-tool list-tags --registry mac-fedora:8443 --namespace init/openshift --name release --query-params="/v2/init/openshift/release/tags/list?n=50&next_page=gAAAAABm8xZbbw7PRezKbb5KFnXmKbypG5wRG0qcGX5Dn2qTWo0OkIexSsbBG6jGsQQFpZ79fQa5s7iZjC6c2NQ65hyTr2sagJZua-HHjxbnDyKYzi31RYE%3D"
--query-params="/v2/init/openshift/release/tags/list?n=50&next_page=gAAAAABm8xb6_Xut-HaKxErSXhE7nipjZe2-SoI6NeoqyJ7cipRE0RnGSTz_k3mfr3vicIFAgmXA4_BwxHkhbaS3ZZlFlVkvU_vcmiO3Q3XMygWJQChKNN0%3D"
 [ INFO  2024-09-24 21:46:02.621 ]   : image init/openshift/release
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-monitoring-plugin-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-machine-os-images-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-machine-os-content-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-machine-image-customization-controller-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-libvirt-machine-controllers-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-kubevirt-csi-driver-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-kubevirt-cloud-controller-manager-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-kube-storage-version-migrator-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-kube-state-metrics-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-kube-rbac-proxy-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-kube-proxy-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-kube-metrics-server-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-keepalived-ipfailover-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-k8s-prometheus-adapter-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ironic-static-ip-manager-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ironic-machine-os-downloader-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ironic-agent-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ironic-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-installer-artifacts-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-installer-altinfra-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-installer-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ibmcloud-machine-controllers-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ibmcloud-cluster-api-controllers-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ibm-vpc-node-label-updater-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ibm-vpc-block-csi-driver-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-ibm-cloud-controller-manager-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-hypershift-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-hyperkube-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-haproxy-router-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-gcp-pd-csi-driver-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-gcp-machine-controllers-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-gcp-cluster-api-controllers-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-gcp-cloud-controller-manager-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-etcd-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-egress-router-cni-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-driver-toolkit-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-docker-registry-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-docker-builder-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-deployer-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-snapshot-validation-webhook-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-snapshot-controller-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-node-driver-registrar-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-livenessprobe-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-external-snapshotter-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-external-resizer-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-external-provisioner-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-external-attacher-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-driver-shared-resource-webhook-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-driver-shared-resource-x86_64
 [ INFO  2024-09-24 21:46:02.622 ]   : tag   4.15.8-x86_64-csi-driver-nfs-x86_64


```

### Get ETag

```bash
 ./target/release/rust-registry-tool list-tags --registry mac-fedora:8443 --namespace init/ubi9 --name ubi-micro   
 [ INFO  2024-09-24 21:48:46.543 ]   : image init/ubi9/ubi-micro
 [ INFO  2024-09-24 21:48:46.544 ]   : tag   sha256-11b5e26e24ce14b02372860115162e81ae011b748619b371f261e1e97d4cf2bf

$ ./target/release/rust-registry-tool digest --registry mac-fedora:8443 --namespace init/ubi9 --name ubi-micro --tag sha256-11b5e26e24ce14b02372860115162e81ae011b748619b371f261e1e97d4cf2bf  
 [ INFO  2024-09-24 21:49:13.900 ]   : etag digest sha256:799e688668dc5642e738f7cb5fb8fa763b42fc593c9c58c74ebf61b344e8a362

```
