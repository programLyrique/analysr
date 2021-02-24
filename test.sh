#!/bin/bash

# More threads will fail as it would create one R interpreter for each thread and 
# R is no reentrant
cargo test --  --test-threads=1