VERSION = "0.1.0:alpha-test"
DOCKER_IMG = "mcaptcha/cache$(VERSION)"
DOCKER_CONTAINER = "mcaptcha_cache_test"

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

docker:
	docker build -t mcaptcha/cache:0.1.0-beta -t mcaptcha/cache:latest  .
	docker push mcaptcha/cache:0.1.0-beta 
	docker push mcaptcha/cache:latest

docker-build:
	docker build -t $(DOCKER_IMG) .

docker-run:
	docker run --detach --name=$(DOCKER_CONTAINER) \
		--publish 6379:6379 \
		$(DOCKER_IMG)

docker-stop:
	docker stop $(DOCKER_CONTAINER) || true
	docker rm $(DOCKER_CONTAINER)
env:
	./scripts/setup.sh

test: 
	cargo test  --all --all-features --no-fail-fast
	./tests/test.py

xml-test-coverage:
	cargo tarpaulin -t 1200 --out Xml --all --all-features --no-fail-fast

run-redis:
	redis-server  --loadmodule ./target/release/libcache.so &

stop-redis:
	killall redis-server

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
