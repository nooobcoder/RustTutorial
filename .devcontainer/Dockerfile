FROM ubuntu:latest
# Install rust on the container
# mkdir /home if not exists
RUN mkdir -p /home
WORKDIR /home/
COPY . .
RUN bash ./setup.sh
ENV PATH="/root/.cargo/bin:$PATH"
