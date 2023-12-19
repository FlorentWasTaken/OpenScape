openScape = {}

-- read a file and returns its content
function openScape.readFile(path)
    local file, err = io.open(path, "r")

    if not file then
        return nil
    end
    local data = file:read("*a")

    file:close()
    return data
end

function openScape.writeFile(path, content)
    if not content or not path then return false end

    local file, err = io.open(path, "a")

    if not file then
        return false
    end

    file:write(content)
    file:close()
    return true
end


-- transform a string into an array
function openScape.strToWordArray(str)
    local sep = '\n'
    local lines = {}

    if not str then return nil end

    for ligne in str:gmatch("[^\r\n]+") do
        local col = {}

        for i = 1, #ligne do
            local caractere = ligne:sub(i, i)
            table.insert(col, caractere)
        end
        table.insert(lines, col)
    end
    return lines
end

-- transform an array into a string
function openScape.wordArrayToStr(lines)
    local sep = '\n'
    local result = {}

    if not lines then return nil end

    for _, col in ipairs(lines) do
        local ligne = table.concat(col, "")
        table.insert(result, ligne)
    end

    return table.concat(result, sep)
end

