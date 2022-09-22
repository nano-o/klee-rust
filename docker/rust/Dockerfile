ARG FROM_IMAGE_FOR_RUST
FROM ${FROM_IMAGE_FOR_RUST}

USER user

# install current rust with rustup
##################################
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/home/user/.cargo/bin:$PATH"

# clone rust and setup with LLVM 13
###################################

RUN git clone https://github.com/rust-lang/rust.git

WORKDIR rust
RUN git checkout tags/1.63.0 -b 1.63
RUN git submodule set-branch -b "rustc/13.0-2021-09-30" "src/llvm-project"
RUN git submodule init
RUN git submodule update --recursive
RUN cd src/llvm-project && git switch "rustc/13.0-2021-09-30"
RUN git config user.name 'NA'
RUN git config user.email '<>'
RUN git commit -a -m "use LLVM 13"

# build rust
############

USER root
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends ninja-build
USER user

# python command not found; use python3:
RUN sed '1 s/python/python3/g' -i x.py
RUN ./x.py setup user
RUN ./x.py build
RUN rustup toolchain link stage2 build/x86_64-unknown-linux-gnu/stage2
# check that the stage2 compiler is installed:
RUN rustc +stage2 -vV

WORKDIR /home/user/
