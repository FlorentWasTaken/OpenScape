function include(file)
    local chunk, err = loadfile(file)
    if chunk then
        return chunk()
    else
        error("Error while loading '" .. file .. "': " .. err)
    end
end

include("./src/script/include.lua")
include("./src/script/lib.lua")

-- example 1

local function example1()
    local content = openScape.readFile("./src/script/example1.txt")
    local lines = openScape.strToWordArray(content)

    if not lines then return end

    for k, v in ipairs(lines) do
        for i, j in ipairs(v) do
            if j == "1" then
                place_block(i, k)
            end
        end
    end
end

local function example2()
    for i = 0, math.random(1, 20), 1 do
        place_block(math.random(1, 10), math.random(1, 10))
    end
end

-----

function main()
    --example1()
    --example2()
end

main()
