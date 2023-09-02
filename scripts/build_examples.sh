#!/bin/bash
mkdir -p ./docs/examples/assets
cp -R ./examples/assets ./docs/examples

doc_body="<ul>\n"

compile() {
  f=$1
  f=${f/\.\/examples\//""}
  f=${f/.rs/""}
  ./scripts/web_example.sh $f --no-assets

  url="examples/${f}.html"
  image="examples/images/${f}.jpg"
  doc_body="${doc_body}\n<li><a href=\"${url}\"><div class=\"example-image\"><img src=\"${image}\" alt=\"${f}\"></div><div class=\"example-link\">${f}</div></a></li>"
}

for f in ./examples/*.rs; do
  compile "$f" 
done

wait

doc_body="${doc_body}\n</ul>"
cp ./scripts/docs.html ./docs/index.html
index=$(sed "s#{{ BODY }}#${doc_body}#g" "./scripts/docs.html")
echo "${index}" > ./docs/index.html
