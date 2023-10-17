DOCKER_CONTAINER = "test_instance"

default:
	cargo build --release

bench:
	./scripts/bench.sh

clean:
	cargo clean

coverage:
	cargo tarpaulin -t 1200 --out Html --all --all-features --no-fail-fast

dev:
	cargo build

doc:
	cargo doc --no-deps --workspace --all-features --document-private-items

docker: docker
	docker push mcaptcha/cache:0.1.0-beta 
	docker push mcaptcha/cache:latest

docker-build:
	docker buildx build -t mcaptcha/cache:0.1.0-beta -t mcaptcha/cache:latest  . --load

docker-run:
	docker run -d --name=$(DOCKER_CONTAINER) \
		--publish 6379:6379 \
		mcaptcha/cache:latest

docker-stop:
	docker stop $(DOCKER_CONTAINER) || true
	docker rm $(DOCKER_CONTAINER)

env:
	./scripts/setup.sh
	@-virtualenv venv || true
	@-pip install codespell

test:
	./tests/test.py
	#@ . venv/bin/activate && ./scripts/spellcheck.sh -c
#	cargo test  --all --all-features --no-fail-fast

xml-test-coverage:
	cargo tarpaulin -t 1200 --out Xml --all --all-features --no-fail-fast

run-redis:
	redis-server  --loadmodule ./target/release/libcache.so &

stop-redis:
	killall redis-server

lint: ## Lint codebase
	@ . venv/bin/activate && ./scripts/spellcheck.sh -w
	cargo fmt -v --all -- --emit files
	cargo clippy --workspace --tests --all-features

help:
	@echo  '  bench                   - run benchmarks'
	@echo  '  clean                   - drop builds and environments'
	@echo  '  coverage                - build test coverage in HTML format'
	@echo  '  doc                     - build documentation'
	@echo  '  docker-build            - build docker image'
	@echo  '  docker-run              - run docker container'
	@echo  '  docker-stop             - stop docker container'
	@echo  '  env                     - setup dev env'
	@echo  '  run-redis               - load and run redis on local machine'
	@echo  '  stop-redis              - kill local redis instance'
	@echo  '  test                    - run unit and integration tests'
	@echo  '  xml-coverage            - build test coverage in XML for upload to codecov'
	@echo  ''
