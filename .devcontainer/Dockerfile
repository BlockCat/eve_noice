#-------------------------------------------------------------------------------------------------------------
# Copyright (c) Microsoft Corporation. All rights reserved.
# Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
#-------------------------------------------------------------------------------------------------------------

FROM rust:1

RUN echo "Installing rust toolchain"
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup component add rls rust-analysis rust-src

# Install git, process tools
RUN echo "Install dependencies"
RUN apt-get update && apt-get -y install git procps
RUN apt-get install -y lldb-3.9 mariadb-server mariadb-client
RUN cargo install diesel_cli --no-default-features --features mysql


RUN echo "Starting database"

# Clean up
RUN echo "Cleaning up"
RUN apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Set the default shell to bash instead of sh
ENV DATABASE_URL "mysql://root:example@localhost/eve_noice"
EXPOSE 8000:8000
ENV SHELL /bin/bash

# ENTRYPOINT service mysql start && mysql -u root -e "CREATE DATABASE IF NOT EXISTS eve_noice;" && diesel migration run