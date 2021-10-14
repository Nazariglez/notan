#!/bin/bash
mkdir -p ./docs/examples/assets
cp -R ./examples/assets ./docs/examples

doc_body="<ul>\n"

for f in ./examples/*.rs; do
  f=${f/\.\/examples\//""}
  f=${f/.rs/""}
  ./scripts/web_example.sh $f --release --no-assets

  url="/examples/${f}.html"
  doc_body="${doc_body}\n<li><a href=\"${url}\">${f}</a></li>"
done

doc_body="${doc_body}\n</ul>"
cp ./scripts/docs.html ./docs/index.html
index=$(sed "s#{{ BODY }}#${doc_body}#g" "./scripts/docs.html")
echo "${index}" > ./docs/index.html