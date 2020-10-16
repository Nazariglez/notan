mkdir -p output
mkdir -p output/examples
cp -R examples/assets output/examples/assets

for f in examples/*.rs; do
  f=${f/examples\//""}
  f=${f/.rs/""}
  if [[ $1 == 'web' ]];
  then
    ./web_example.sh $f $2
  else
    cargo build --example $f $2
    if [[ $2 == "--release" ]];
    then
      cp ./target/release/examples/$f output/$f
    else
      cp ./target/debug/examples/$f output/$f
    fi
  fi
done