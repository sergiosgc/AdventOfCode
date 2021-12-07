#!/bin/zsh
zmodload -i zsh/mathfunc

INPUT=$(cat - | tr "," "\n" | sort -n)
MEDIAN=$(echo $INPUT | head -n $(( $(echo "$INPUT" | wc -l) / 2 + 1 )) | tail -n 2 | ( read A; read B; echo $(( ($A + $B) / 2)) ) )
echo $(( $(echo "$INPUT" | sed "s/\(.*\)/abs($MEDIAN - \1) +/"; echo 0) ))