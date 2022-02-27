FROM espressif/idf

RUN sed -i 's/archive.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list && \
    sed -i 's/security.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list && \
    apt update && \
    apt install -y curl openssh-server net-tools inetutils-ping psmisc vim

RUN apt install sudo
EXPOSE 22

RUN echo "PermitRootLogin yes" >> /etc/ssh/sshd_config

RUN env | grep _ >> /etc/environment

RUN /etc/init.d/ssh start

ENTRYPOINT [ "/usr/sbin/sshd", "-D" ]
