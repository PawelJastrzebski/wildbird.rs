
test_feature () {
    echo ""
    echo "------------------- $1 --------------------"
    echo ""
    cargo test --features "$1"
}

test_feature "rayon"
test_feature "tokio"
test_feature "timed-log"
test_feature "timed-tracing"

cargo test