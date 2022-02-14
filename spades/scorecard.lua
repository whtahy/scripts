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

    -- columns 1 and 2, for navigation using the tab keyboard key
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

    local total_bags, total_score = nil

    -- calculate bags and scores
    for i = 1, 15 do
        local bid_string = self.getInputs()[i * 2 - 1].value
        local tricks_string = self.getInputs()[i * 2].value
        local bags, score = nil
        local bags_string, score_string = '', ''

        -- nil
        if (bid_string:find('^[nb]%+%d+$') -- nil + bid
                or bid_string:find('^%d+%+[nb]$') -- bid + nil
                or bid_string:find('^[nb]%+[nb]$')) -- nil + nil
            and tricks_string:find('^%d+%+%d+$') then
            local _, _, bid_a, bid_b =
                bid_string:find('^(.+)%+(.+)$')
            local _, _, tricks_a, tricks_b =
                tricks_string:find('^(%d+)%+(%d+)$')
            local bags_a, score_a = calculate_bags_and_score(bid_a, tricks_a)
            local bags_b, score_b = calculate_bags_and_score(bid_b, tricks_b)
            bags = bags_a + bags_b
            score = score_a + score_b
        -- standard
        elseif bid_string:find('^%d+$')
            and tricks_string:find('^%d+$') then
            bags, score = calculate_bags_and_score(bid_string, tricks_string)
        -- invalid row
        elseif bid_string ~= '' and tricks_string ~= '' then
            local r = 'row = '..i
            local b = 'bid = '..bid_string
            local t = 'tricks = '..tricks_string
            print(r..', '..b..', '..t..' is invalid.')
        -- else empty string
        end

        if bags and score then
            bags_string = tostring(bags)
            score_string = tostring(score)
            if total_bags == nil and total_score == nil then
                total_bags = 0
                total_score = 0
            end
            total_bags = total_bags + bags
            total_score = total_score + score
        end

        self.editInput({index = i * 2 + 28, value = bags_string})
        self.editInput({index = i * 2 + 29, value = score_string})
    end

    if total_score ~= nil and total_bags ~= nil then
        total_score = total_score - math.floor(total_bags / 10) * 100
        -- index starts at 0
        self.editInput({index = 61, value = tostring(total_score)})
        self.editInput({index = 62, value = tostring(total_bags)})
    end
end

function calculate_bags_and_score(bid, tricks)
    -- parse bid
    if bid ~= 'n' and bid ~= 'b' then
        bid = tonumber(bid)
    end

    -- parse tricks
    tricks = tonumber(tricks)

    -- invalid inputs -> early return
    if bid ~= 'b' and bid ~= 'n' and (bid < 0 or bid > 13) then
        print('Error: bid = '..bid..' is out of range.')
        print("Fix: use 1 thru 13, or 'n' for nil, 'b' for blind nil")
        return nil
    elseif tricks < 0 or tricks > 13 then
        print('Error: tricks = '..tricks..' is out of range.')
        print('Fix: use 0 thru 13')
        return nil
    elseif bid == 0 then
        print('Error: bid = 0 is invalid.')
        print("Fix: use 'n' for nil and 'b' for blind nil, instead of '0'.")
        return nil
    end

    local score = nil

    -- successful bid
    if bid == 'n' then
        score = 100
        bid = 0
    elseif bid == 'b' then
        score = 200
        bid = 0
    else
        score = bid * 10
    end

    -- failed bid
    if (bid == 0 and tricks > 0) or (bid > 0 and tricks < bid) then
        score = score * -1
    end

    local bags = 0

    -- bags
    if tricks > bid then
        bags = tricks - bid
    end

    return bags, score + bags
end
