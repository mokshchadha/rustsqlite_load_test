# rustsqlite_load_test
load testing rustsqlite in different settings


### Install "wrk" for load testing
1. On macOS: brew install wrk
2. On Ubuntu: sudo apt-get install wrk

### How to run load test 
wrk -t12 -c400 -d30s -s load_test.lua http://localhost:8000

1. -t12: Uses 12 threads. 
2. -c400: Opens 400 connections.
3. -d30s: Runs the test for 30 seconds.
4. -s load_test.lua: Specifies the Lua script to use
5. http://localhost:8000: The URL of the server to test.