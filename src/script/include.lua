function place_block(x, y)
    local success, result = pcall(function() return create_block(x, y) end)

    return result
end

function destroy_block(x, y)
    local success, result = pcall(function() return remove_block(x, y) end)

    return result
end

function get_cam_pos()
    local success_x, result_x = pcall(function() return get_camera_x() end)
    local success_y, result_y = pcall(function() return get_camera_y() end)

    return result_x, result_y
end

function set_cam_pos(x, y)
    local success, result = pcall(function() return set_camera_pos(x, y) end)

    return result
end
