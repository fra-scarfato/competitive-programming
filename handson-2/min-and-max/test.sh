#!/bin/bash

# iterate on the number of test
for i in {0..10}; do
    # execute the program and save the output in res$i.txt
    ./target/debug/handson2 < Testset_handson2_p1/input$i.txt > res$i.txt
    # check if there are difference from the output and expected output
    pass=$(diff <(grep -v '^\s*$' Testset_handson2_p1/output$i.txt) <(grep -v '^\s*$' res$i.txt))
    if [[ -z $pass ]]; then
        echo  "Test $i passed."
        # if test is passed, remove the file of output
        rm res$i.txt
    else
        echo "ERROR! Test $i not passed"
        echo "$pass"
    fi
done
