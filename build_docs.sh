# reference: https://github.com/liufuyang/pingcap-talent-plan/blob/master/.travis.yml

rm -rf docs
cargo doc --no-deps -p bigtable_rs
echo "<meta http-equiv=refresh content=0;url=bigtable_rs/index.html>" > target/doc/index.html
mv target/doc ./docs