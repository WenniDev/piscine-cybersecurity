FROM alpine:latest

RUN apk --no-cache --no-progress add nginx tor torsocks openssh &&\
    rm /etc/nginx/http.d/default.conf &&\
    mkdir -p /var/www/jopadova /run/nginx &&\
    ssh-keygen -A &&\
    echo "root:root" | chpasswd

COPY nginx.conf /etc/nginx/http.d
COPY index.html /var/www/jopadova
COPY sshd_config /etc/ssh/sshd_config
COPY torrc /etc/tor

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["sh","/entrypoint.sh"]
