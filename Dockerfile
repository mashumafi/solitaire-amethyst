FROM debian

ARG CHANNEL=stable
ARG MDBOOK_RELEASE=0.3.0
ARG KCOV_VERSION=36
ARG KCOV_BUILD_DIR=kcov-build

RUN apt-get update && \
	apt-get install -y \
	git \
	libasound2-dev \
	libx11-xcb-dev \
	libssl-dev \
	cmake \
	libfreetype6-dev \
	libexpat1-dev \
	libxcb1-dev \
	python3 \
	build-essential \
	libsdl2-dev \
	curl \
	jq \
	libcurl4-openssl-dev \
	libelf-dev \
	libdw-dev \
	gcc \
	binutils-dev \
	zlib1g-dev \
	libiberty-dev \
	pkg-config \
	python2.7 \
	wget \
	zip && \
	apt-get clean

# --- kcov --- #

# For some reason `python` isn't the default executable file name.
# Needed for kcov
RUN ln -s /usr/bin/python2.7 /usr/bin/python

# Download kcov
# -nc is okay since we are downloading a tagged version
RUN wget -nc https://github.com/SimonKagstrom/kcov/archive/v$KCOV_VERSION.zip -O kcov.zip && \
	unzip -uoq kcov.zip && \
	rm kcov.zip

# Build KCov
RUN mkdir -p "kcov-$KCOV_VERSION/${KCOV_BUILD_DIR}" && \
	cd "kcov-$KCOV_VERSION/${KCOV_BUILD_DIR}" && \
	cmake .. -DCMAKE_INSTALL_PREFIX=/usr/local -DCMAKE_BUILD_TYPE=Release && \
	make && \
	make install && \
	rm -rf "kcov-$KCOV_VERSION/${KCOV_BUILD_DIR}"

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain ${CHANNEL}

RUN echo $HOME
RUN ls $HOME
ENV PATH=/home/root/.cargo/bin:$PATH

RUN rustup component add rustfmt
RUN rustup component add clippy

RUN cargo install mdbook --vers ${MDBOOK_RELEASE}
