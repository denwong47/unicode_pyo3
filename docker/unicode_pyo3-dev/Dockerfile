FROM python:3.10-slim
# Required for source to work, because sh -c does not implement source.
SHELL ["/bin/bash", "-c"]

ARG FUNCTION_DIR="/home/app"

RUN apt-get clean

RUN apt-get update && apt-get install -y \
  build-essential \
  curl \
  git \
  htop \
  iperf3 \
  iputils-ping \
  make \
  nano \
  zsh

# Setup Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s - -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN python -m pip install --upgrade pip

# Append any aliases we wanted
ADD ./docker/unicode_pyo3-dev/.bashrc /root/.bashrc_append
RUN cat /root/.bashrc_append >> /root/.bashrc

# Install Python convenience packages
ADD ./docker/unicode_pyo3-dev/requirements.txt /root/app_requirements.txt
RUN python -m pip install -r /root/app_requirements.txt

# Add vscode workspace
ADD ./docker/unicode_pyo3-dev/unicode_pyo3.code-workspace ${FUNCTION_DIR}/unicode_pyo3.code-workspace

# Create the mount point, copy the current repo over, and install editable.
# If pyproject.toml changes sufficiently for this not to work, then
# manually use `make pip_reinstall`.
ADD ./ ${FUNCTION_DIR}/unicode_pyo3
RUN python -m pip install -e "${FUNCTION_DIR}/unicode_pyo3[dev]"
# RUN rm -rf ${FUNCTION_DIR}/unicode_pyo3/*

VOLUME ["${FUNCTION_DIR}/unicode_pyo3"]

ENTRYPOINT ["bash"]
