ARG FROM_IMAGE_FOR_RUST
FROM ${FROM_IMAGE_FOR_RUST}

USER user

# clone rust and update submodules
# this takes forever, so we isolate it in this image in order to not have to redo it when other stuff changes

RUN git clone https://github.com/rust-lang/rust.git

WORKDIR rust
RUN git checkout tags/1.63.0 -b 1.63
RUN git submodule init
RUN git submodule update --recursive
