#!/bin/bash
declare -a TARGETS=(
    "minerva_rest"
    "minerva_runonce"
    "minerva_session"
    "minerva_user"
)

# Rust language targets
for TARGET in "${TARGETS[@]}"
do
    IMGNAME="luksamuk/${TARGET}"

    # Scrape version information and use as tag
    DIRNAME=${TARGET/_/-}
    IMGVERSION=`awk '/^version/{print $3}' ./$DIRNAME/Cargo.toml | tr -d '"'`;
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

# FrontEnd target
IMGVERSION=`awk '/^version/{print $2}' ./minerva_frontend/pubspec.yaml`
echo "### Building luksamuk/minerva_frontend..."
docker image build -f build/Dockerfile.frontend \
       -t luksamuk/minerva_frontend:$IMGVERSION \
       -t luksamuk/minerva_frontend:latest \
       .

# PgAdmin
docker image build -f build/Dockerfile.pgadmin \
       -t luksamuk/minerva_pgadmin:latest \
       build

