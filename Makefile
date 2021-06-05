default:
	cargo build --release

test: 
	cargo test  --all --all-features --no-fail-fast
	./tests/test.py

docker-build:
	docker build -t mcaptcha/cache:0.1.1-alpha .

docker-stop:
	docker stop mcaptcha_cache_test || true
	docker rm mcaptcha_cache_test

docker-run:
	docker run --detach --name=mcaptcha_cache_test \
		--publish 6379:6379 \
		mcaptcha/cache:0.1.1-alpha 

dev-env:
	./scripts/setup.sh

docs:
	cargo doc --no-deps --workspace --all-features

xml-test-coverage:
	cargo tarpaulin -t 1200 --out Xml --all --all-features --no-fail-fast

coverage:
	cargo tarpaulin -t 1200 --out Html --all --all-features --no-fail-fast
dev:
	cargo build

clean:
	cargo clean

help:
	@echo  '  run                     - run developer instance'
	@echo  '  test                    - run unit and integration tests'
	@echo  '  docker-build            - build docker image'
	@echo  '  docker-run              - run docker container'
	@echo  '  docker-stop             - stop docker container'
	@echo  '  dev-env                 - setup dev env'
	@echo  '  docs                    - build documentation'
	@echo  '  clean                   - drop builds and environments'
	@echo  '  coverage                - build test coverage in HTML format'
	@echo  '  xml-coverage            - build test coverage in XML for upload to codecov'
	@echo  ''
