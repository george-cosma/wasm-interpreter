if [[ $* == *--dev* ]]
then
    docker build -t easy/nix .
    docker run -it --rm -v $(pwd):/wasm-interpreter -p 1337:1337 easy/nix
else
    if [ "$(docker ps --all | grep easy-nix)" ]
    then
        docker start -i easy-nix
    else
        docker run -it --name easy-nix -v $(pwd):/wasm-interpreter -p 1337:1337 easy/nix
    fi
fi
