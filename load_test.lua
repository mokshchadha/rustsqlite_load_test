-- load_test.lua
wrk.method = "GET"
wrk.headers["Content-Type"] = "application/json"

-- Customize the request path if needed
request = function()
   return wrk.format(nil, "/")
end
