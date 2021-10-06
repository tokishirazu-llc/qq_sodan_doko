.PHONY: build clean deploy

init:
	npm install -g serverless serverless-plugin-custom-binary

build:
	docker run --rm -it \
	-v "$(CURDIR)":/home/rust/src \
	-v cargo-git:/home/rust/.cargo/git \
 	-v cargo-registry:/home/rust/.cargo/registry \
	-v "$(CURDIR)"/target:/home/rust/src/target \
	ekidd/rust-musl-builder:1.51.0 \
	bash -c "sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry /home/rust/src/target && cargo build"

test:
	docker run --rm -it \
	-v "$(CURDIR)":/home/rust/src \
	-v cargo-git:/home/rust/.cargo/git \
 	-v cargo-registry:/home/rust/.cargo/registry \
	-v "$(CURDIR)"/target:/home/rust/src/target \
	ekidd/rust-musl-builder:1.51.0 \
	bash -c "sudo chown -R rust:rust /home/rust/.cargo/git /home/rust/.cargo/registry /home/rust/src/target && cargo test"
	
package:
	cp target/x86_64-unknown-linux-musl/debug/api /tmp/bootstrap
	zip -j ./api.zip /tmp/bootstrap
	
deploy: package
	npx sls deploy --verbose --stage dev

deploy-prd: package
	npx sls deploy --verbose --stage prd