#!/bin/bash
declare -a TARGETS=(
#    "minerva_frontend"
    "minerva_rest"
    "minerva_runonce"
    "minerva_session"
    "minerva_user"
    "minerva_dispatch"
)

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
    docker image build \
	   -f build/Dockerfile \
	   --target $TARGET \
	   -t $TAGGEDIMGNAME \
	   -t $IMGNAME:latest \
	   .
done

