#!/usr/bin/zsh

IP=$1
LOC=$2
EXP=$3
RUN=$4

RUN_DIR="run_${(l(4)(0))RUN}"

RUN_PATH="${LOC}/${EXP}/${RUN_DIR}"

MAKE_COMMAND="mkdir -p ${RUN_PATH}"

MOVE_COMMAND="mv -f ${LOC}/*.graw ${RUN_PATH}"

ssh $IP "${MAKE_COMMAND}"
ssh $IP "${MOVE_COMMAND}"