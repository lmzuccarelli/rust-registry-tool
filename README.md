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


```

### List all tags from registry (namespace/name)

```bash
./target/release/rust-registry-tool list-tags --registry mac-fedora:8443 --namespace init/openshift --name release --version v4.16.0


./target/release/rust-registry-tool list-tags --registry registry.redhat.io --namespace redhat --name redhat-operator-index --version v4.16.0

# a file with <registry>.json will be created 
# use the following jq to pull out some values an example
# cat redhat-operator-index.json | jq -c '[ .[] | select( .tags[] | test("v4.15-[0-9]")?) ]' | jq | more

```

### List all tags from registry by link url

```bash
# the previous cli creates a file called link.txt - use this as a reference
# take any line and use the following to get to a quick subset of tags
./target/release/registry-tool --loglevel debug list-tags-by-url --registry registry.redhat.io --url "/v2/redhat/redhat-operator-index/tags/list?n=100&last=v4.15-1697439109"

```


### Get ETag

```bash
 ./target/release/rust-registry-tool list-tags --registry mac-fedora:8443 --namespace init/ubi9 --name ubi-micro   

$ ./target/release/rust-registry-tool digest --registry mac-fedora:8443 --namespace init/ubi9 --name ubi-micro --tag sha256-11b5e26e24ce14b02372860115162e81ae011b748619b371f261e1e97d4cf2bf  

```
