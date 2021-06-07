VERSION = "0.1.0:alpha-test"
DOCKER_IMG = "mcaptcha/cache$(VERSION)"
DOCKER_CONTAINER = "mcaptcha_cache_test"

default:
	cargo build --release

test: 
	cargo test  --all --all-features --no-fail-fast
	./tests/test.py

docker-build:
	docker build -t $(DOCKER_IMG) .

docker-stop:
	docker stop $(DOCKER_CONTAINER) || true
	docker rm $(DOCKER_CONTAINER)

docker-run:
	docker run --detach --name=$(DOCKER_CONTAINER) \
		--publish 6379:6379 \
		$(DOCKER_IMG)

run-redis-server:
	redis-server  --loadmodule ./target/release/libcache.so &

stop-redis-server:
	killall redis-server

dev-env:
	./scripts/setup.sh

docker:
	docker build -t mcaptcha/cache:0.1.0-beta -t mcaptcha/cache:latest  .
	ducker push mcaptcha/cache:0.1.0-beta 
	ducker push mcaptcha/cache:latest

xml-test-coverage:
	cargo tarpaulin -t 1200 --out Xml --all --all-features --no-fail-fast

coverage:
	cargo tarpaulin -t 1200 --out Html --all --all-features --no-fail-fast
dev:
	cargo build

doc:
	cargo doc --no-deps --workspace --all-features

clean:
	cargo clean

help:
	@echo  '  run                     - run developer instance'
	@echo  '  test                    - run unit and integration tests'
	@echo  '  docker-build            - build docker image'
	@echo  '  docker-run              - run docker container'
	@echo  '  docker-stop             - stop docker container'
	@echo  '  dev-env                 - setup dev env'
	@echo  '  doc                     - build documentation'
	@echo  '  clean                   - drop builds and environments'
	@echo  '  coverage                - build test coverage in HTML format'
	@echo  '  xml-coverage            - build test coverage in XML for upload to codecov'
	@echo  ''
