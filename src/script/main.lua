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
    place_block(10, 10)
    destroy_block(10, 10)
end

main()
