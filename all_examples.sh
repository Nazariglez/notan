for f in examples/*.rs; do
  f=${f/examples\//""}
  f=${f/.rs/""}
  ./web_example.sh $f
done