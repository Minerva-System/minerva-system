#!/bin/bash
declare -a TARGETS=(
    "minerva_frontend"
    "minerva_rest"
    "minerva_runonce"
    "minerva_session"
    "minerva_user"
)

# Platforms to build. Currently, linux/arm64 is broken for buildx.
# See https://github.com/docker/build-push-action/issues/621.
#PLATFORMS=linux/amd64,linux/arm64
PLATFORMS=linux/amd64

# Rust language targets
for TARGET in "${TARGETS[@]}"
do
    IMGNAME="luksamuk/${TARGET}"

    # Scrape version information and use as tag
    if [ $TARGET != "minerva_frontend" ]; then
    	DIRNAME=${TARGET/_/-}
	IMGVERSION=`awk '/^version/{print $3}' ./$DIRNAME/Cargo.toml | tr -d '"'`;
    else
	DIRNAME=$TARGET
	IMGVERSION=`awk '/^version/{print $2}' ./$DIRNAME/pubspec.yaml`
    fi

    TAGGEDIMGNAME=$IMGNAME:$IMGVERSION
    
    # Generate image and tag it with "latest"
    echo "### Building $IMGNAME..."
    docker buildx build \
	   -m 2g \
	   -f build/Dockerfile \
	   --platform=$PLATFORMS \
	   --target $TARGET \
	   -t $TAGGEDIMGNAME \
	   -t $IMGNAME:latest \
	   .
done

