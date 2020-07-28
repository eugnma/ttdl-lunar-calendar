# See below for examples:
#   - https://github.com/microsoft/vscode-dev-containers/tree/master/containers
#   - vscode-dev-containers/containers/rust: https://git.io/JfOjM
#   - https://github.com/microsoft/vscode-remote-try-rust

##
# downloader
FROM debian:buster AS downloader

WORKDIR /downloads

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl/buster \
        ca-certificates/buster \
        gnupg/buster \
    #
    # Download and verify the NodeSource package signing key
    && url='https://deb.nodesource.com/gpgkey/nodesource.gpg.key' \
    && curl -sSLf -o nodesource.gpg.key "$url" \
    # How to display gpg key details without importing it?
    # https://stackoverflow.com/q/22136029
    && gpg nodesource.gpg.key 2>/dev/null \
        | head -n -2 \
        | tail -n 1 \
        | xargs echo \
        | grep -1 '^9FD3B784BC1C6FC31A8A0A1C1655A0AB68576280$' \
    #
    # Clean up
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*


##
# editorconfig-checker-getter
FROM golang:1-buster AS editorconfig-checker-getter

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Install editorconfig-checker
#   - Repository: https://github.com/editorconfig-checker/editorconfig-checker
#   - Alternative: https://gitlab.com/greut/eclint
RUN set -eu; \
    readonly VERSION='2.1.0'; \
    if [ "$(uname -m)" = 'x86_64' ]; then \
        url="https://github.com/editorconfig-checker/editorconfig-checker/releases/download/${VERSION}/ec-linux-amd64.tar.gz"; \
        checksum='ab047668a046353fb9618bc7de09d8e80e2a81d229722132709cdd60b56f7ec9'; \
        curl -sSLf -o ec-linux-amd64.tar.gz "$url"; \
        echo "$checksum ec-linux-amd64.tar.gz" | sha256sum -c --status; \
        tar -x \
            -f ec-linux-amd64.tar.gz \
            -C /opt/ \
            bin/ec-linux-amd64; \
        mv /opt/bin/ec-linux-amd64 /opt/ec; \
    else \
        git clone \
            -c advice.detachedHead=false \
            -b "$VERSION" \
            --depth 1 \
            https://github.com/editorconfig-checker/editorconfig-checker.git; \
        echo "Revision: $(git -C editorconfig-checker rev-parse HEAD)"; \
        make -C editorconfig-checker build; \
        mv editorconfig-checker/bin/ec /opt/; \
    fi; \
    chmod 755 /opt/ec;


##
# hadolint-getter
FROM haskell:8-buster AS hadolint-getter

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Download or build hadolint
#   - Repository: https://github.com/hadolint/hadolint
RUN set -eu; \
    readonly VERSION='v1.18.0'; \
    if [ "$(uname -m)" = 'x86_64' ]; then \
        url="https://github.com/hadolint/hadolint/releases/download/${VERSION}/hadolint-Linux-x86_64"; \
        checksum='f9bc9de12438b463ca84e77fde70b07b155d4da07ca21bc3f4354a62c6199db4'; \
        curl -sSLf -o hadolint "$url"; \
        echo "$checksum hadolint" | sha256sum -c --status; \
        cp hadolint /opt/; \
    else \
        git clone \
            -c advice.detachedHead=false \
            -b "$VERSION" \
            --depth 1 \
            https://github.com/hadolint/hadolint.git; \
        echo "Revision: $(git -C hadolint rev-parse HEAD)"; \
        stack setup; \
        stack install hadolint; \
        cp "$HOME/.local/bin/hadolint" /opt/; \
    fi; \
    chmod 755 /opt/hadolint;

##
# shfmt-getter
FROM golang:1-buster AS shfmt-getter

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# NOTE: DO NOT USE shfmt OUTSIDE OF CONTAINER!
# Download or build shfmt
#   - Repository: https://github.com/mvdan/sh
#   - GO111MODULE: https://golang.org/cmd/go/#hdr-Module_support
#   - Go module compatibility and semantic versioning: https://golang.org/cmd/go/#hdr-Module_compatibility_and_semantic_versioning
RUN set -eu; \
    readonly VERSION='v3.1.2'; \
    if [ "$(uname -m)" = 'x86_64' ]; then \
        url="https://github.com/mvdan/sh/releases/download/${VERSION}/shfmt_${VERSION}_linux_amd64"; \
        checksum='c5794c1ac081f0028d60317454fe388068ab5af7740a83e393515170a7157dce'; \
        curl -sSLf -o "shfmt_${VERSION}_linux_amd64" "$url"; \
        echo "$checksum shfmt_${VERSION}_linux_amd64" | sha256sum -c --status; \
        cp "shfmt_${VERSION}_linux_amd64" /opt/shfmt; \
    else \
        GO111MODULE=on go get "mvdan.cc/sh/v3/cmd/shfmt@$VERSION"; \
        cp /go/bin/shfmt /opt/; \
    fi; \
    chmod 755 /opt/shfmt;


##
# toml-cli-builder
FROM rust:1-buster as toml-cli-builder

RUN cargo install --version '>= 0.2.0' toml-cli


##
# cargo-audit-builder
FROM rust:1-buster as cargo-audit-builder

RUN cargo install --version '>= 0.12.0' cargo-audit


##
# the final image
FROM rust:1-buster

ARG USERNAME=vscode
ARG USER_GNAME=$USERNAME
ARG USER_UID=1000
ARG USER_GID=$USER_UID
ARG ENABLE_LICENSEFINDER=0

# Support unicode characters
ENV LANG C.UTF-8

SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# Create a non-root user, see below for details:
#   - Adding a non-root user to your dev container: https://aka.ms/vscode-remote/containers/non-root-user
#   - groupadd: https://manpages.debian.org/buster/passwd/groupadd.8.en.html
#   - useradd: https://manpages.debian.org/buster/passwd/useradd.8.en.html
RUN groupadd --gid $USER_GID $USER_GNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

COPY container/shebang-run /usr/local/bin/
COPY --chown=$USERNAME:$USER_GNAME \
    container/git/.gitmessage.txt \
    "/home/$USERNAME/"
COPY --chown=$USERNAME:$USER_GNAME \
    container/git/hooks \
    "/home/$USERNAME/.githooks/"
COPY --from=downloader /downloads/nodesource.gpg.key /root
COPY --from=editorconfig-checker-getter /opt/ec /usr/local/bin/
COPY --from=hadolint-getter /opt/hadolint /usr/local/bin/
COPY --from=shfmt-getter /opt/shfmt /usr/local/bin/

# The following package are already existed in the base image:
#   - git
#   - gnupg
#   - ca-certificates
RUN apt-get update \
    #
    # Add backports to sources.list
    #   - Backports Instructions: https://backports.debian.org/Instructions/
    #   - SourcesList: https://wiki.debian.org/SourcesList
    # Will install the following packages from backports:
    #   - shellcheck
    && apt-get install -y --no-install-recommends \
        apt-transport-https/buster \
    && echo 'deb https://deb.debian.org/debian buster-backports main' > \
        /etc/apt/sources.list.d/buster-backports.list \
    && apt-get update \
    #
    # Setup the shebang-run command
    && chmod 755 /usr/local/bin/shebang-run \
    #
    # Setup Git
    #   - Git Configuration: https://git-scm.com/book/en/v2/Customizing-Git-Git-Configuration
    #   - Multiline Dockerfile syntax: https://github.com/moby/moby/issues/1799
    #   - Using Version Control in VS Code: https://code.visualstudio.com/Docs/editor/versioncontrol
    && su - $USERNAME -c " \
        set -eu; \
        git config --global core.editor 'code --wait'; \
        git config --global diff.tool default-difftool; \
        git config --global difftool.default-difftool.cmd \
            'code --wait --diff \$LOCAL \$REMOTE'; \
        git config --global commit.template ~/.gitmessage.txt; \
        git config --global core.hooksPath ~/.githooks; \
    " \
    && chmod -R 755 "/home/$USERNAME/.githooks/" \
    #
    # Install Node.js
    #   - Required by some conventional Commits tool and lint
    #   - Manual installation: https://git.io/Jfs3o
    #   - gnupg is required for executing the command `apt-key add ...`
    && apt-key add /root/nodesource.gpg.key \
    && version='node_12.x' \
    && distro='buster' \
    && echo "deb https://deb.nodesource.com/$version $distro main" > \
        /etc/apt/sources.list.d/nodesource.list \
    && echo "deb-src https://deb.nodesource.com/$version $distro main" >> \
        /etc/apt/sources.list.d/nodesource.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends nodejs/buster \
    && rm /root/nodesource.gpg.key \
    #
    # Conventional Commits
    #   - commitlint
    #   - standard-version
    && npm i -g \
        @commitlint/cli@">=9.1.1" \
        @commitlint/config-conventional@">=9.1.1" \
        standard-version@">=8.0.2" \
    #
    # Linters
    #   - [Installed] ec for EditorConfig
    #   - [Installed] hadolint for Dockerfile
    #   - [Installed] shfmt for Shell script
    #   - [Installed] commitlint for commit messages
    #   - Prettier for JavaScript, Markdown, YAML, etc.
    #   - markdownlint-cli for Markdown
    #   - ShellCheck for Shell script
    && npm i -g \
        prettier@">=2.0.5" \
        markdownlint-cli@">=0.23.2" \
    && apt-get install -y --no-install-recommends shellcheck/buster-backports \
    #
    # Licenses
    #   - LicenseFinder (disabled by default)
    #       - Recommend approving a specific version and saving the decisions
    #         file to ".license_finder/dependency_decisions.yml"
    #       - Recommend saving the report to ".license_finder/report.txt" for
    #         tracking what dependencies are removed
    && \
    if [ "$ENABLE_LICENSEFINDER" = '1' ]; then \
        apt-get install -y --no-install-recommends ruby/buster \
        && gem install license_finder -v '>=6.7.0'; \
    fi \
    #
    # Release
    && npm i -g commonmark@">=0.29.1" \
    #
    # Clean up
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# The second stage for the specific language
COPY --from=toml-cli-builder /usr/local/cargo/bin/toml /usr/local/bin/
COPY --from=cargo-audit-builder /usr/local/cargo/bin/cargo-audit /usr/local/bin/
RUN apt-get update \
    #
    # Install lldb, vadimcn.vscode-lldb VSCode extension dependencies
    #   - vscode-lldb requirements: https://git.io/JfOjF
    #   - vscode-lldb setup: https://git.io/JfOjb
    && apt-get install -y --no-install-recommends \
        lldb/stable \
        python3/stable \
        libpython3.7/stable \
    #
    # Update and install Rust components
    #   rustup: https://git.io/JfOjN
    #   rls-vscode: https://github.com/rust-lang/rls-vscode#requirements
    #   rustfmt: https://github.com/rust-lang/rustfmt
    #   rust-clippy: https://github.com/rust-lang/rust-clippy
    && rustup update 2>&1 \
    && rustup component add rls rust-src rust-analysis rustfmt clippy 2>&1 \
    #
    # Clean up
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Reset to the default shell:
# https://docs.docker.com/engine/reference/builder/#shell
SHELL ["/bin/sh", "-c"]

# Run as a non-root user by default, it can be overridden in devcontainer.json.
USER $USERNAME

# Set the USER environment variable as it is required for `cargo new <path>`.
ENV USER=$USERNAME
