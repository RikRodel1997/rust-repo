
assert_eq() {
    local actual=$1
    local expected=$2
    local test_name=$3

    if [ "$actual" -eq "$expected" ]; then
        echo -e " \xE2\x9C\x94  $test_name"
    else
        echo -e " \e[31m\u274C\e[0m  $test_name (expected $expected, actual: $actual)"
    fi
}

expected=( 10 5 )
idx=0

cargo build

for file in hy-files/*; do
    ./target/debug/hydrogen-compiler debug $file && ./asm.sh > /dev/null 2>&1
    asm_output=$(./asm.sh)
    assert_eq $asm_output "${expected[$index]}" $file
    index=$((index + 1))
done