#!/usr/bin/zsh

IP=$1
LOC=$2
EXP=$3
RUN=$4

RUN_DIR="run_${(l(4)(0))RUN}"

COMMAND="ls ${LOC}/${EXP}/${RUN_DIR}"

ssh $IP \"${COMMAND}\"