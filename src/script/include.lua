function place_block(x, y)
    local success, result = pcall(function() return create_block(x, y) end)

    return result
end

function destroy_block(x, y)
    local success, result = pcall(function() return remove_block(x, y) end)

    return result
end
