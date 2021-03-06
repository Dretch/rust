fn main() {
    assert []/_.count(22u) == 0u;
    assert [1u, 3u]/_.count(22u) == 0u;
    assert [22u, 1u, 3u]/_.count(22u) == 1u;
    assert [22u, 1u, 22u]/_.count(22u) == 2u;
    assert None.count(22u) == 0u;
    assert Some(1u).count(22u) == 0u;
    assert Some(22u).count(22u) == 1u;
}
