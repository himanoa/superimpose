#!/usr/bin/env zsh

ROOT=${0:h:h:A}
PATH=$ROOT/bin/resolver:$ROOT/bin/zunit:$ROOT/target/debug:$PATH

cd $ROOT/bin/zunit && ./build.zsh
cd $ROOT && zunit ${@:-tests/*.zunit}
