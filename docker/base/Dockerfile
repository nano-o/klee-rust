ARG UBUNTU_VERSION
FROM ubuntu:${UBUNTU_VERSION}

RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
ARG DEBIAN_FRONTEND=noninteractive
# are both the above needed?

# always run apt-get update before install to make sure we don't have a stale database
RUN apt-get update && apt-get install -y --no-install-recommends apt-utils
RUN apt-get update && apt-get install -y --no-install-recommends dialog ca-certificates

# Install and configure locale `en_US.UTF-8`
RUN apt-get update && apt-get install -y locales && \
    sed -i -e "s/# $en_US.*/en_US.UTF-8 UTF-8/" /etc/locale.gen && \
    dpkg-reconfigure --frontend=noninteractive locales && \
    update-locale LANG=en_US.UTF-8
ENV LANG=en_US.UTF-8
ENV TZ="America/Los_Angeles"

# create a user who can use sudo:
RUN apt-get update && apt-get install -y sudo
RUN useradd -ms /bin/bash user && echo 'user:user' | chpasswd && adduser user sudo
RUN echo "ALL ALL=(ALL) NOPASSWD: ALL" >> /etc/sudoers

# install a bunch of tools:
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends \
  vim \
  vim-nox \
  curl \
  aptitude \
  cmake \
  python3 \
  bear \
  gdb \
  universal-ctags \
  rsync \
  git \
  build-essential \
  pkg-config \
  autoconf \
  automake \
  libtool \
  bison \
  flex \
  libpq-dev \
  parallel \
  ccache \
  cmake \
  ripgrep \
  bat \
  fd-find \
  fzf \
  zsh \
  tmux \
  tmate \
  stow \
  wget

# Switch back to dialog for any ad-hoc use of apt-get
RUN echo 'debconf debconf/frontend select Interactive' | debconf-set-selections

USER user
WORKDIR /home/user/

# configure stuff:
RUN curl -fLo /home/user/.vim/autoload/plug.vim --create-dirs https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim
RUN mkdir dotfiles
ADD ./dotfiles/ dotfiles/
WORKDIR /home/user/dotfiles/
RUN stow vim
RUN stow ctags
RUN mkdir /home/user/.vim/backupfiles
RUN mkdir /home/user/.vim/swapfiles
RUN mkdir /home/user/.vim/undofiles
RUN stow tmux
RUN stow zsh
RUN stow gdb
# RUN stow bash
RUN echo '\n\
# disable C-S in case it causes freezing in vim\n\
stty -ixon\n\
# Avoid duplicates\n\
HISTCONTROL=ignoredups:erasedups\n\
# When the shell exits, append to the history file instead of overwriting it\n\
shopt -s histappend\n\
# After each command, append to the history file and reread it\n\
PROMPT_COMMAND="${PROMPT_COMMAND:+$PROMPT_COMMAND$'\n'}history -a; history -c; history -r"\n'\
>> /home/user/.bashrc

ENV PATH=/home/user/bin:${PATH}
ENV BAT_THEME=ansi

# install current rust with rustup
##################################
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/home/user/.cargo/bin:$PATH"


WORKDIR /home/user/
