FROM mcr.microsoft.com/devcontainers/rust:0-bullseye
RUN curl -o /usr/local/share/ca-certificates/certificates.crt http://browserconfig.target.com/tgt-certs/tgt-ca-bundle.crt
RUN update-ca-certificates
RUN apt-get update && apt-get install -y python-is-python3
# ENV NODE_EXTRA_CA_CERTS=/tgt-ca-bundle.crt 
# TODO -- get into the working folder and do npm install
# RUN pwd
# WORKDIR /workspaces/video-export-service
# RUN npm install