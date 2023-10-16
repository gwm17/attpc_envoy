#!/usr/bin/zsh

CONFIG=$1
BACK=$2
EXP=$3
RUN=$4

RUN_DIR="run_${(l(4)(0))RUN}"
RUN_PATH="${BACK}/${EXP}/${RUN_DIR}"

mkdir -p $RUN_PATH
cp -f $CONFIG/*.xcfg $RUN_PATH