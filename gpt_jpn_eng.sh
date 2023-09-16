#!/bin/bash

text=$(pbpaste)
cd $HOME/Dropbox/code/rust/gpt_jpn_eng
./target/release/gpt_jpn_eng $text