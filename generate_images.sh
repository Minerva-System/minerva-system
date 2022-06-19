#!/bin/bash
declare -a TARGETS=(
    "minerva_rest"
    "minerva_runonce"
    "minerva_session"
    "minerva_users"
)

# Rust language targets
for TARGET in "${TARGETS[@]}"
do
    IMGNAME="luksamuk/${TARGET}"

    # Generate image and tag it with "latest"
    echo "### Building $IMGNAME..."
    docker image build -f build/Dockerfile \
	   --target $TARGET \
	   -t $IMGNAME:latest .;

    # Scrape version information and use as tag
    DIRNAME=${TARGET/_/-}
    DIRNAME=${DIRNAME/users/user} # users/user dichotomy. Should be removed!
    IMGVERSION=`awk '/^version/{print $3}' ./$DIRNAME/Cargo.toml | tr -d '"'`;
    TAGGEDIMGNAME=$IMGNAME:$IMGVERSION

    # Tag image according to Cargo.toml
    echo "### Tagging $TAGGEDIMGNAME..."
    docker image tag $IMGNAME $TAGGEDIMGNAME
done

# FrontEnd target
IMGVERSION=`awk '/^version/{print $2}' ./minerva_frontend/pubspec.yaml`
echo "### Building luksamuk/minerva_frontend..."
docker image build -f build/Dockerfile.frontend \
       -t luksamuk/minerva_frontend:latest \
       .

echo "### Tagging luksamuk/minerva_frontend:$IMGVERSION"
docker image tag luksamuk/minerva_frontend luksamuk/minerva_frontend:$IMGVERSION

# PgAdmin
docker image build -f build/Dockerfile.pgadmin \
       -t luksamuk/minerva_pgadmin:latest \
       build

