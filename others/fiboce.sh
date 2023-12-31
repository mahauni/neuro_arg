#!/bin/bash

fibonacci() {
    len="$1"
    f1=0
    f2=1

    for (( i=0; i < len; i++ )); do
        # use bc over bash's built-in arithmetic $(( )) because bash can't 
        # handle large numbers. Specifically anything past the 92nd number 
        # in the fibonacci series is too large (92 = 7540113804746346429)

        echo "$f1 % 26" | bc

        fn=$(echo "$f1 + $f2" | bc)
        f1=$f2
        f2=$fn
    done
}

rot() {
#    alpha='abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz'
#    rotate=$(($2 % 26))
#    echo $(echo $1 | tr "${alpha:0:26}" "${alpha:${rotate}:26}")

    if grep -q "^[A-Z]" <<< $1; then
        alpha="ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    else
        alpha="abcdefghijklmnopqrstuvwxyz"
    fi

    rotate=$(echo "$2 % 26" | bc)
    sed "y/${alpha}/${alpha:$rotate}${alpha::$rotate}/" <<< $1
}


ACTION="$1"
shift 1

MESSAGE="$@"
LENGTH=$(wc -c <<< "$MESSAGE")
FIBO_STREAM=$(fibonacci $LENGTH)


FIBO_INDEX=0
for (( i=0; i < $LENGTH; i++ )); do
    CHAR=${MESSAGE:$i:1}

    if grep -q '^[A-Za-z]$' <<< $CHAR; then
        FIBO_ROTATE=$(awk "NR == $((FIBO_INDEX+1))" <<< $FIBO_STREAM)
        FIBO_INDEX=$((FIBO_INDEX+1))

        if [ "$ACTION" = "encrypt" ]; then
            ROTATE=$FIBO_ROTATE
        elif [ "$ACTION" = "decrypt" ]; then
            ROTATE=$(( 26 - $FIBO_ROTATE ))
        fi

        echo -n "$(rot $CHAR $ROTATE)"
    else
        echo -n "$CHAR"
    fi
done
