ARG FROM_IMAGE_FOR_SOROBAN
ARG KLEE_IMAGE

FROM ${KLEE_IMAGE} AS klee
FROM ${FROM_IMAGE_FOR_SOROBAN}

RUN mkdir /home/user/klee-uclibc
COPY --from=klee /home/user/klee-uclibc /home/user/klee-uclibc/
RUN mkdir /home/user/klee
COPY --from=klee /home/user/klee /home/user/klee

ENV PATH=/home/user/klee/build/bin/:$PATH
