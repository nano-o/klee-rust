ARG KLEE_REPO_IMAGE
ARG FROM_IMAGE_FOR_KLEE

FROM ${KLEE_REPO_IMAGE} AS klee-repo
FROM ${FROM_IMAGE_FOR_KLEE}

USER user

RUN mkdir /home/user/klee
COPY --from=klee-repo /home/user/klee /home/user/klee/
WORKDIR klee
RUN git checkout "tags/v2.3"
WORKDIR /home/user/

USER root

RUN apt-get update && apt-get install -y --no-install-recommends build-essential cmake curl file g++-multilib gcc-multilib git libcap-dev libgoogle-perftools-dev libncurses5-dev libsqlite3-dev libtcmalloc-minimal4 python3-pip unzip graphviz doxygen zlib1g-dev
RUN apt-get update && apt-get install -y --no-install-recommends python3-tabulate
RUN apt-get update && apt-get install -y --no-install-recommends clang-13 llvm-13 llvm-13-dev llvm-13-tools
RUN apt-get update && apt-get install -y --no-install-recommends z3

USER user

RUN pip3 install lit wllvm

RUN mkdir /home/user/klee-uclibc
COPY --from=klee-repo /home/user/klee-uclibc /home/user/klee-uclibc/
WORKDIR /home/user/klee-uclibc
RUN ./configure --make-llvm-lib --with-cc clang-13 --with-llvm-config llvm-config-13
RUN make -j8

WORKDIR /home/user/klee
RUN mkdir build
WORKDIR build
RUN cmake -DENABLE_SOLVER_Z3=ON -DENABLE_POSIX_RUNTIME=ON -DKLEE_UCLIBC_PATH=/home/user/klee-uclibc -DENABLE_UNIT_TESTS=OFF -DENABLE_SYSTEM_TESTS=OFF -DLLVM_CONFIG_BINARY=/usr/bin/llvm-config-13 ..
RUN make -j8

ENV PATH=/home/user/klee/build/bin/:$PATH
WORKDIR /home/user
