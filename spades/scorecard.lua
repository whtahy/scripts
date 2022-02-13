-- Assumptions
-- scale_width = scale_length
-- scale_height = 1
scale_width = self.getScale().x
scale_height = self.getScale().y
scale_length = self.getScale().z

default_font_size = 32.5
default_textbox_width = 75

offset_length = -0.02
spacing_length = 0.1024
offset_width = 0
spacing_width = 0.171

function onLoad()
    -- grid = 15 rows by 4 columns
    -- 1 2 31 32
    -- 3 4 33 34
    -- 5 6 35 36
    -- . .  .  .

    -- columns 1 and 2
    for i = -5, 9 do
        for j = -1, 0 do
            textbox(
                offset_width + j * spacing_width,
                offset_length + i * spacing_length
            )
        end
    end

    -- columns 3 and 4
    for i = -5, 9 do
        for j = 1, 2 do
            textbox(
                offset_width + j * spacing_width,
                offset_length + i * spacing_length
            )
        end
    end

    -- team name
    textbox(-0.035, -0.84, 60, 90)

    -- total score
    textbox(0.33, -0.893, 37.5, 85)

    -- total bags
    textbox(0.33, -0.79, 37.5)
end

function textbox(x, z, font_size, width)
    local font_size = (font_size or default_font_size) * scale_width
    local width = (width or default_textbox_width) * scale_width

    self.createInput({
        function_owner = self,
        input_function = 'update_scorecard',
        label = '',
        position = {x, 0.101, z},
        scale = {1/scale_width, 1, 1/scale_length},
        font_size = font_size,
        font_color = {0/255, 60/255, 110/255, 150},
        height = font_size + 24,
        width = width,
        color = {1, 1, 1, 0},
        alignment = 3, -- center
        tab = 2, -- next input
    })
end

function update_scorecard(parent_obj, player_color, input_value, is_selected)
    if is_selected then
        return nil
    end

    -- calculate bags and scores
    local total_bags = 0
    local total_score = 0
    for i = 1, 15 do
        -- index starts at 1
        local bid = tonumber(self.getInputs()[i * 2 - 1].value)
        local tricks = tonumber(self.getInputs()[i * 2].value)

        local bags = 0
        local score = 0
        local bags_string = ''
        local score_string = ''

        if bid ~= nil and tricks ~= nil then
            -- bags
            if tricks > bid then
                bags = tricks - bid
            end
            bags_string = tostring(bags)

            -- score
            score = bid * 10
            if tricks < bid then
                score = score * -1
            end
            score = score + bags
            score_string = tostring(score)
        end

        -- update bags, index starts at 0
        self.editInput({
            index = i * 2 + 28,
            value = bags_string
        })
        total_bags = total_bags + bags

        -- update score, index starts at 0
        self.editInput({
            index = i * 2 + 29,
            value = score_string
        })
        total_score = total_score + score
    end

    total_score = total_score - math.floor(total_bags / 10) * 100

    -- update total score, index starts at 0
    self.editInput({
        index = 61,
        value = tostring(total_score)
    })

    -- update total bags, index starts at 0
    self.editInput({
        index = 62,
        value = tostring(total_bags)
    })
end
