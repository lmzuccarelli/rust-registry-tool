## Overview

This is a simple cli tool that can query a remote registry to get a list of images, a list of tags for a specific image, the e-tag for a specific manifest
 


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


```

### List all tags from registry (namespace/name)

```bash
# not version must respect the regex expresion : v4.[0-9]{2}.0
./target/release/rust-registry-tool list-tags --registry mac-fedora:8443 --namespace init/openshift --name release --version 4.16.0


./target/release/rust-registry-tool list-tags --registry registry.redhat.io --namespace redhat --name redhat-operator-index --version 4.16.0
```

### Get Manifest

```bash
# get the manifest or manifest list and pipe to jq
registry-tool --loglevel info digest --registry registry.redhat.io --namespace redhat --name certified-operator-index --tag v4.16-1711400921 --no-format | jq

```
