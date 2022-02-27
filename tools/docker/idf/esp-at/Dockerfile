FROM ubuntu:18.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    apt-utils \
    bison \
    ca-certificates \
    ccache \
    check \
    curl \
    flex \
    git \
    gperf \
    lcov \
    libncurses-dev \
    libusb-1.0-0-dev \
    make \
    ninja-build \
    python3 \
    python3-pip \
    unzip \
    wget \
    xz-utils \
    zip \
    openssh-server \
    net-tools inetutils-ping psmisc \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/* \
    && update-alternatives --install /usr/bin/python python /usr/bin/python3 10


RUN python -m pip install --upgrade pip virtualenv
RUN python -m pip install pyyaml xlrd


ENV IDF_PATH=/opt/esp/esp-idf
ENV IDF_AT_PATH=/opt/esp/esp-at
ENV IDF_TOOLS_PATH=/opt/esp


ARG IDF_CLONE_URL=https://github.com/espressif/esp-idf.git
ARG IDF_AT_CLONE_URL=https://github.com/espressif/esp-at.git


RUN git clone --recursive  $IDF_CLONE_URL $IDF_PATH && \
    git clone --recursive  $IDF_AT_CLONE_URL $IDF_AT_PATH


RUN $IDF_PATH/tools/idf_tools.py --non-interactive install required \
    && $IDF_PATH/tools/idf_tools.py --non-interactive install cmake \
    && $IDF_PATH/tools/idf_tools.py --non-interactive install-python-env \
    && rm -rf $IDF_TOOLS_PATH/dist


# Ccache is installed, enable it by default
ENV IDF_CCACHE_ENABLE=1

RUN echo "PermitRootLogin yes" >> /etc/ssh/sshd_config

RUN source $IDF_PATH/export.sh && env | grep _ >> /etc/environment

RUN /etc/init.d/ssh start

EXPOSE 22

ENTRYPOINT [ "/usr/sbin/sshd", "-D" ]
