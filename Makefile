IMAGE=canmi/jellyfish
TAG=$(shell git rev-parse --short HEAD)

push:
	docker buildx build \
	  --platform linux/amd64,linux/arm64 \
	  -t $(IMAGE):latest \
	  -t $(IMAGE):$(TAG) \
	  . \
	  --push
	docker pushrm $(IMAGE)
