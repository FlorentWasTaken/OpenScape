function place_block(x, y)
    local success, result = pcall(function() return create_block(x, y) end)
end

function destroy_block(x, y)
    print("block destroyed")
end
