FROM espressif/idf

RUN sed -i 's/archive.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list && \
    sed -i 's/security.ubuntu.com/mirrors.aliyun.com/g' /etc/apt/sources.list && \
    apt update && \
    apt install -y curl openssh-server net-tools inetutils-ping psmisc vim

EXPOSE 22

ENV IDF_PATH=/opt/esp/idf

RUN echo ". ${IDF_PATH}/export.sh" >> /etc/bash.bashrc

RUN echo "PermitRootLogin yes" >> /etc/ssh/sshd_config


RUN /etc/init.d/ssh start

ENTRYPOINT [ "/usr/sbin/sshd", "-D" ]
