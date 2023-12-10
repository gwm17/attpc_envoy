#!/usr/bin/zsh

CONFIG=$1
BACK=$2
EXP=$3
RUN=$4

RUN_DIR="run_${(l(4)(0))RUN}"
RUN_PATH="${BACK}/${EXP}/${RUN_DIR}"

PREPARE="${CONFIG}/prepare-${EXP}.xcfg"
DESCRIBE="${CONFIG}/describe-${EXP}.xcfg"
DESCRIBE_COBOS="${CONFIG}/describe-cobo"
CONFIGURE="${CONFIG}/configure-${EXP}.xcfg"

mkdir -p $RUN_PATH
cp -f $DESCRIBE $RUN_PATH
cp -f $PREPARE $RUN_PATH
cp -f $CONFIGURE $RUN_PATH
cp -f "${DESCRIBE_COBOS}"*".xcfg" $RUN_PATH