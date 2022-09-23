ARG RUST_REPO_IMAGE
ARG FROM_IMAGE_FOR_RUST

FROM ${RUST_REPO_IMAGE} AS rust-repo
FROM ${FROM_IMAGE_FOR_RUST}

USER user

RUN mkdir /home/user/rust
COPY --from=rust-repo /home/user/rust /home/user/rust/

# clone rust and setup with LLVM 12
###################################

WORKDIR rust
RUN git submodule set-branch -b "rustc/12.0-2021-07-10" "src/llvm-project"
RUN cd src/llvm-project && git switch "rustc/12.0-2021-07-10"
RUN git config user.name 'NA'
RUN git config user.email '<>'
RUN git commit -a -m "use LLVM 12"
RUN git submodule update --recursive

# build rust
############

USER root
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y --no-install-recommends ninja-build
USER user

# python command not found; use python3:
RUN sed '1 s/python/python3/g' -i x.py
RUN ./x.py setup user
RUN echo '[llvm]' >> ./config.toml
RUN echo 'targets = "X86"' >> ./config.toml
RUN echo 'experimental-targets = ""' >> ./config.toml
RUN ./x.py build
RUN rustup toolchain link stage2 build/x86_64-unknown-linux-gnu/stage2
# check that the stage2 compiler is installed:
RUN rustc +stage2 -vV

WORKDIR /home/user/
