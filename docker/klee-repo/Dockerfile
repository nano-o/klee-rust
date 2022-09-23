ARG FROM_IMAGE_FOR_KLEE
FROM ${FROM_IMAGE_FOR_KLEE}

USER user

WORKDIR /home/user
RUN git clone https://github.com/klee/klee.git
RUN git clone https://github.com/klee/klee-uclibc.git
