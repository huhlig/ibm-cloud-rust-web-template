IBM Cloud Rust Web Template
===========================

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Build Status](https://travis-ci.org/huhlig/ibm-cloud-rust-web-template.svg?branch=master)](https://travis-ci.org/huhlig/ibm-cloud-rust-web-template)

Example project used to accelerate using Rust in the IBM Cloud. 

### Deploying

```bash
$ pwd
/home/ibm/projects/ibm-cloud-rust-web-template

$ bx info
Listing cloud info...

Name:                          Bluemix
Description:                   IBM Bluemix

$ bx cf push
Invoking 'cf push'...

Using manifest file /home/ibm/projects/ibm-cloud-rust-web-template/manifest.yml

Uploading ibm-cloud-rust-web-template...
Uploading app files from: /home/ibm/projects/ibm-cloud-rust-web-template
Uploading 14.3K, 18 files
Done uploading
OK

Starting app ibm-cloud-rust-web-template in org IBM Cloud Examples / space Rust Examples as noreply@ibm.com...
Creating container
Successfully created container
Downloading app package...
Downloaded app package (13.4K)
Downloading build artifacts cache...
Downloading build artifacts cache...
Downloaded build artifacts cache (233.8M)
Staging...
-----> Checking for new releases of Rust stable channel
info: checking for self-updates
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.22.1 (05e2e1c41 2017-11-22)
info: using existing install for 'stable-x86_64-unknown-linux-gnu'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'
  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.22.1 (05e2e1c41 2017-11-22)
-----> Building application using Cargo
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling language-tags v0.2.2
   Compiling byteorder v1.2.1
   Compiling safemem v0.2.0
   Compiling unicode-normalization v0.1.5
   Compiling route-recognizer v0.1.12
   Compiling percent-encoding v1.0.1
   Compiling httparse v1.2.3
   Compiling sequence_trie v0.3.5
   Compiling typeable v0.1.2
   Compiling matches v0.1.6
   Compiling modifier v0.1.0
   Compiling cfg-if v0.1.2
   Compiling version_check v0.1.3
   Compiling siphasher v0.2.2
   Compiling libc v0.2.34
   Compiling traitobject v0.1.0
   Compiling base64 v0.6.0
   Compiling unicode-bidi v0.3.4
   Compiling log v0.4.0
   Compiling unsafe-any v0.4.2
   Compiling rand v0.3.19
   Compiling num_cpus v1.8.0
   Compiling time v0.1.38
   Compiling typemap v0.3.3
   Compiling unicase v1.4.2
   Compiling log v0.3.9
   Compiling idna v0.1.4
   Compiling plugin v0.2.6
   Compiling mime v0.2.6
   Compiling phf_shared v0.7.21
   Compiling phf_generator v0.7.21
   Compiling phf v0.7.21
   Compiling phf_codegen v0.7.21
   Compiling mime_guess v1.8.3
   Compiling url v1.6.0
   Compiling hyper v0.10.13
   Compiling iron v0.6.0
   Compiling mount v0.4.0
   Compiling logger v0.4.0
   Compiling router v0.6.0
   Compiling staticfile v0.5.0
   Compiling ibm-cloud-rust-web-template v0.0.1 (file:///tmp/app)
    Finished release [optimized] target(s) in 118.76 secs
  Installing /tmp/app/bin/ibm-cloud-rust-web-template
Uploading droplet, build artifacts cache...
Exit status 0
Staging complete
Uploading droplet...
Uploading build artifacts cache...
Uploaded droplet (1.5M)
Uploaded build artifacts cache (233.9M)
Uploading complete

1 of 1 instances running
App started

App ibm-cloud-rust-web-template was started using this command `bin/ibm-cloud-rust-web-template`
OK

Showing health and status for app ibm-cloud-rust-web-template in org IBM Cloud Examples / space Rust Examples as noreply@ibm.com...
OK

requested state: started
instances: 1/1
usage: 1G x 1 instances
urls: ibm-cloud-rust-web-template.mydomain.net
last uploaded: Sat Dec 31 11:59:59 UTC 2017
stack: cflinuxfs2
buildpack: https://github.com/huhlig/rust-buildpack

     state     since                    cpu    memory     disk         details
#0   running   2017-12-29 07:11:08 PM   0.0%   4M of 1G   6.3M of 1G
```
