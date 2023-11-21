
test_feature () {
    echo ""
    echo "------------------- $1 --------------------"
    echo ""
    cargo test --features "$1"
}

test_feature "rayon"
test_feature "tokio"
test_feature "timed_log"
test_feature "timed_tracing"

cargo test