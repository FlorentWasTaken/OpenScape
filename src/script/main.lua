function include(file)
    local chunk, err = loadfile(file)
    if chunk then
        return chunk()
    else
        error("Error while loading '" .. file .. "': " .. err)
    end
end

include("./src/script/include.lua")

function main()
    local success, result = pcall(function() return test_func(42, 3.14) end)
end

main()
