## Overview

This is a simple cli tool that can query a remote registry to get a list of images, a list of tags for a specific image, the e-tag for a specific manifest
as wel las copy from file base oci or dockerv2 images to a remote registry and vice versa, it does not do any registry to registry copy 
and from disk to remote mirror registry

## Usage

This assumes you have already installed Rust (refer to https://www.rust-lang.org/tools/install)

Clone this repo

Ensure that you have the correct permissions set in the $XDG_RUNTIME_DIR/containers/auth.json file

