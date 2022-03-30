FROM ubuntu
RUN apt-get update \
  && DEBIAN_FRONTEND=noninteractive apt-get install -y netcat \
  && apt-get clean

CMD ["nc"]
