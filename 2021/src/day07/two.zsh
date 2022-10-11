#!/bin/zsh
zmodload -i zsh/mathfunc

function individual_cost {
    FROM=$([ $1 -lt $2 ] && echo $1 || echo $2)
    TO=$([ $1 -lt $2 ] && echo $2 || echo $1)
    RESULT=0
    STEP=1
    while [ $FROM -lt $TO ]
    do
        RESULT=$(( $RESULT + $STEP ))
        STEP=$(( $STEP + 1 ))
        FROM=$(( $FROM + 1))
    done
    echo $RESULT
}
function aggregate_cost {
    DESTINATION=$1
    RESULT=0
    for POS in $(echo $INPUT | tr "\n" " ")
    do
        RESULT=$(( $RESULT + $(individual_cost $POS $DESTINATION) ))
    done
    echo $RESULT
}
INPUT=$(cat - | tr "," "\n" | sort -n)
for POS in $(seq $( echo $INPUT | head -n 1 ) $( echo $INPUT | tail -n 1) ) 
do
    aggregate_cost $POS
done | sort -n | head -n 1