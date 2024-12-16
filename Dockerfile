FROM lambda-rust-base

WORKDIR /usr/src/app
COPY . .

# Build the lambda function
# cargo lambda build --release
RUN cargo lambda build --release