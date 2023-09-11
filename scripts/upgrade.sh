SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

for crate in crates/*; do
	current="$SCRIPT_DIR/../$crate"
  echo "$current"
	cd $current
	cargo upgrade
done

current="$SCRIPT_DIR/../"
echo $current
cd $current
cargo upgrade
