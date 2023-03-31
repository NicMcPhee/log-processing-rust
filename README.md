# Log processing in Rust

The goal here is a Rust implementation of the 
[log processing lab](https://github.com/UMM-CSci-Systems/Log-processing)
for the University of Minnesota Morris Computing Systems Practicum
course.

That lab does everything with `bash` scripting, which is pretty intense.
I thought it was be interesting to try the whole thing in Rust to see
what it looks like.

My goal is for the (binary CLI) app to be called liked:

```text
failed-logins this.tgz that.tgz ...
```

where the `.tgz` files are gzipped `tar` files containing different
log files. It will then write out `failed-logins.html` that will be the
HTML summary of the failed logins in the provided log files.
